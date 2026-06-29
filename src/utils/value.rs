use std::ffi::c_int;

use bstr::BString;

use crate::ffi;
use super::strings::BStringFromLuau;

/// Owned representation of any value that can sit on the Luau stack.
///
/// Produced by `StateExt::to_seal`. Owning types (`String`, `Buffer`) are cloned
/// out of Luau's memory immediately so callers don't have to manage stack lifetimes.
/// Non-ownable types (`Table`, `Function`) are marker variants — use the table/call
/// helpers on the state to work with them while they remain on the stack.
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
///     SealValue::Buffer(bytes)         => println!("buffer: {} bytes", bytes.len()),
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
    /// Luau integer (distinct from float since Luau 0.725).
    Integer(i32),
    /// Luau float number.
    Number(f64),
    Vector(f32, f32, f32),
    /// Cloned out of Luau's memory — safe to hold past stack changes.
    String(BString),
    /// Cloned out of Luau's memory — safe to hold past stack changes.
    Buffer(Vec<u8>),
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
            SealValue::None => "none (index out of range)".to_string(),
            SealValue::Nil => "nil".to_string(),
            SealValue::Boolean(b) => format!("boolean({b})"),
            SealValue::Integer(n) => format!("integer({n})"),
            SealValue::Number(f) => format!("number({f})"),
            SealValue::Vector(x, y, z) => format!("vector({x}, {y}, {z})"),
            SealValue::String(s) => format!("string({s:?})"),
            SealValue::Buffer(b) => format!("buffer({} bytes)", b.len()),
            SealValue::UserData { tag, ptr, type_name } => match type_name {
                Some(name) => format!("userdata({name}, tag={tag}, ptr={ptr:?})"),
                None       => format!("userdata(tag={tag}, ptr={ptr:?})"),
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

/// Reads the value at `idx` and returns an owned `Value`, cloning any
/// heap-allocated data (strings, buffers) out of Luau's memory immediately.
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
        let mut isnum: c_int = 0;
        return SealValue::Integer(unsafe { ffi::lua_tointegerx(state, idx, &mut isnum) });
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
        let ptr = unsafe { ffi::lua_tobuffer(state, idx, &mut len) } as *const u8;
        return SealValue::Buffer(unsafe { std::slice::from_raw_parts(ptr, len).to_vec() });
    }
    if tag == ffi::LUA_TUSERDATA() {
        let tag_id = unsafe { ffi::lua_userdatatag(state, idx) };
        let ptr    = unsafe { ffi::lua_touserdata(state, idx) };
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

    // Unknown type tag — future Luau versions may add new types; treat as None
    // rather than panicking so plugins degrade gracefully.
    SealValue::None
}
