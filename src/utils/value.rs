use std::ffi::c_int;

use bstr::BString;

use crate::ffi;
use super::strings::BStringFromLuau;

/// Representation of any value that can sit on the Luau stack.
///
/// Produced by `StateExt::to_seal`. Most lightweight types, 
/// such as numbers, integers, strings, and vectors are cloned out of Luau's 
/// memory immediately so you don't have to do it yourself.
/// 
/// Buffers are represented by `SealBuffer` which provides a non-owned view
/// into a Luau buffer. To get an owned `Vec<u8>` instead, use `SealBuffer::to_owned`.
/// 
/// Reference types like tables and functions are just markers that signify you should
/// use other functions on their stack indices instead (like `state.get_field`, `lua_call`, etc.)
/// 
/// # Example
/// ```ignore
/// use sealbindings::SealValue;
///
/// match state.to_seal(-1) {
///     SealValue::String(s)             => println!("string: {s}"),
///     SealValue::Integer(n)            => println!("integer: {n}"),
///     SealValue::Number(f)             => println!("number: {f}"),
///     SealValue::Boolean(b)            => println!("bool: {b}"),
///     SealValue::Vector(x, y, z)       => println!("vector: {x} {y} {z}"),
///     SealValue::Buffer(buf)           => println!("buffer: {} bytes", buf.len()),
///     SealValue::UserData { type_name, tag, ptr } => { /* use type_name or tag to figure out which ud it is */ }
///     SealValue::LightUserData { tag, ptr } => { /* cast ptr based on tag */ }
///     SealValue::Thread(state)         => { /* coroutine pointer */ }
///     SealValue::Table                 => { /* use for_each_in_table etc. */ }
///     SealValue::Function              => { /* use lua_call etc. */ }
///     SealValue::Nil                   => {}
///     SealValue::None                  => { /* idx was out of range */ }
/// }
/// ```
pub enum SealValue {
    /// The stack index was out of range (no value there). Mirrors `LUA_TNONE`.
    None,
    Nil,
    Boolean(bool),
    /// A Luau 64-bit integer with the "integer" type. Not to be confused with an integer Luau Number.
    Integer(i64),
    /// Luau number.
    Number(f64),
    Vector(f32, f32, f32),
    /// Cloned out of Luau's memory — safe to hold past stack changes.
    String(BString),
    /// A non-owning view into a Luau buffer's bytes. See `SealBuffer`.
    Buffer(SealBuffer),
    /// Tagged userdata. Cast `ptr` based on `tag` to recover the concrete type.
    /// `type_name` is cloned from the metatable's `__type` field if present.
    UserData { type_name: Option<BString>, tag: i32, ptr: *mut std::ffi::c_void },
    /// Light (unmanaged) userdata.
    LightUserData { tag: i32, ptr: *mut std::ffi::c_void },
    /// A coroutine/thread state pointer.
    Thread(*mut ffi::lua_State),
    /// A table. Remains on the stack — use `for_each_in_table`, `get_field`, etc.
    Table,
    /// A function (C or Luau). Remains on the stack — use `lua_call` etc.
    Function,
}

impl SealValue {
    /// Returns a human-readable description of this value, including its type and content.
    /// Useful for logging and debugging without reaching back into the Luau stack.
    pub fn debug_info(&self) -> String {
        match self {
            SealValue::None => "none (stack index out of range)".to_string(),
            SealValue::Nil => "nil".to_string(),
            SealValue::Boolean(b) => format!("boolean({b})"),
            SealValue::Integer(n) => format!("64-bit integer({n})"),
            SealValue::Number(f) => format!("number({f})"),
            SealValue::Vector(x, y, z) => format!("vector({x}, {y}, {z})"),
            SealValue::String(s) => format!("string({s:?})"),
            SealValue::Buffer(buf) => format!("buffer({} bytes)", buf.len()),
            SealValue::UserData { tag, ptr, type_name } => match type_name {
                Some(name) => format!("userdata({name}, tag={tag}, ptr={ptr:?})"),
                None => format!("userdata(tag={tag}, ptr={ptr:?})"),
            },
            SealValue::LightUserData { tag, ptr } => format!("lightuserdata(tag={tag}, ptr={ptr:?})"),
            SealValue::Thread(ptr) => format!("thread({ptr:?})"),
            SealValue::Table => "table".to_string(),
            SealValue::Function => "function".to_string(),
        }
    }
}

impl std::fmt::Debug for SealValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.debug_info())
    }
}

/// Reads the value at `idx` and returns an owned `Value`. Strings are cloned
/// out of Luau's memory immediately; buffers are returned as a non-owning
/// `SealBuffer` view — see `SealBuffer` for why.
///
/// # Safety
/// - `state` must be non-null
/// - `idx` must be a valid stack index or pseudo-index
pub unsafe fn create(state: *mut ffi::lua_State, idx: c_int) -> SealValue {
    let tag = unsafe { ffi::lua_type(state, idx) };

    if tag == ffi::LUA_TNONE()    { return SealValue::None; }
    if tag == ffi::LUA_TNIL()     { return SealValue::Nil; }

    if tag == ffi::LUA_TBOOLEAN() {
        return SealValue::Boolean(unsafe { ffi::lua_toboolean(state, idx) != 0 });
    }
    if tag == ffi::LUA_TINTEGER() {
        return SealValue::Integer(unsafe { ffi::lua_tointeger64(state, idx) });
    }
    if tag == ffi::LUA_TNUMBER() {
        let mut isnum: c_int = 0;
        return SealValue::Number(unsafe { ffi::lua_tonumberx(state, idx, &mut isnum) } as f64);
    }
    if tag == ffi::LUA_TVECTOR() {
        let ptr = unsafe { ffi::lua_tovector(state, idx) };
        return SealValue::Vector(unsafe { *ptr }, unsafe { *ptr.add(1) }, unsafe { *ptr.add(2) });
    }
    if tag == ffi::LUA_TSTRING() {
        return SealValue::String(unsafe { BString::clone_lstring_from_stack(state, idx) });
    }
    if tag == ffi::LUA_TBUFFER() {
        let mut len: usize = 0;
        let ptr = unsafe { ffi::lua_tobuffer(state, idx, &mut len) } as *mut u8;
        return SealValue::Buffer(unsafe { SealBuffer::new(ptr, len) });
    }
    if tag == ffi::LUA_TUSERDATA() {
        let tag_id = unsafe { ffi::lua_userdatatag(state, idx) };
        let ptr = unsafe { ffi::lua_touserdata(state, idx) };
        let type_name = unsafe {
            let name_ptr = ffi::luaL_typename(state, idx);
            let name = std::ffi::CStr::from_ptr(name_ptr).to_bytes();
            if name != b"userdata" {
                Some(BString::from(name))
            } else {
                None
            }
        };
        return SealValue::UserData { tag: tag_id, ptr, type_name };
    }
    if tag == ffi::LUA_TLIGHTUSERDATA() {
        let tag_id = unsafe { ffi::lua_lightuserdatatag(state, idx) };
        let ptr = unsafe { ffi::lua_tolightuserdata(state, idx) };
        return SealValue::LightUserData { tag: tag_id, ptr };
    }
    if tag == ffi::LUA_TTHREAD()   { return SealValue::Thread(unsafe { ffi::lua_tothread(state, idx) }); }
    if tag == ffi::LUA_TTABLE()    { return SealValue::Table; }
    if tag == ffi::LUA_TFUNCTION() { return SealValue::Function; }

    // future Luau versions will add classes, objects, and other types. for now treat them as None

    SealValue::None
}

/// A non-owning view into a Luau buffer's bytes.
///
/// Luau buffers are fixed-size and can hold up to 2 GB, so `SealValue::create`
/// does not eagerly copy their contents like it does for strings. Instead this
/// holds a raw pointer into Luau's GC-owned memory — borrow it with `as_slice`
/// / `as_mut_slice`, or call `to_owned` if you need an owned `Vec<u8>` that
/// outlives the underlying Luau value.
pub struct SealBuffer {
    ptr: *mut u8,
    len: usize,
}

impl SealBuffer {
    /// # Safety
    /// `ptr` must point to `len` valid, initialized bytes owned by a live Luau buffer.
    pub(crate) unsafe fn new(ptr: *mut u8, len: usize) -> Self {
        Self { ptr, len }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Borrows the buffer's contents without copying.
    ///
    /// # Safety
    /// The Luau buffer this view points into must still be alive — i.e. reachable
    /// by Luau's GC and not collected — for the duration of the borrow. Don't hold
    /// this past a Luau call that could collect or move the buffer.
    pub unsafe fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }

    /// Mutably borrows the buffer's contents without copying, so plugins can
    /// write directly into Luau's buffer memory.
    ///
    /// # Safety
    /// Same as `as_slice`, plus: no other live borrow of this buffer may exist.
    pub unsafe fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    /// Clones the buffer's contents into an owned `Vec<u8>` that is safe to
    /// hold past stack changes or Luau GC cycles.
    pub fn to_owned(&self) -> Vec<u8> {
        unsafe { self.as_slice() }.to_vec()
    }
}