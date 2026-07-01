use std::ffi::{CStr, c_char, c_int};

use bstr::BString;

use crate::{LuauStackGuard, ffi};
use super::strings::BStringFromLuau;

/// Maximum safe pre-allocation size for Luau tables. Exceeding this causes a C++
/// exception that Rust cannot catch, aborting the process. Matches seal's own limit.
pub const MAX_TABLE_SIZE: usize = 134_217_728;

/// Ergonomic wrapper around the most common Luau C-Stack API operations.
/// This trait allows you to call methods directly on the `LuauState` passed by *seal*.
/// 
/// Some methods in this trait are unsafe if they take an `idx` param and don't check if the
/// type is actually valid for the function, if they take a row `*const c_char` pointer,
/// or can affect the Luau state in ways that could corrupt it if safety preconditions
/// aren't upheld.
pub trait StateExt {
    // --- Type checks (safe: lua_type handles any idx, returns LUA_TNONE for invalid) ---

    /// Returns `true` if the value's actual type is a number (float).
    /// Note: `ffi::lua_isnumber` returns true for numeric strings due to Lua coercion —
    /// this method uses the type tag directly to avoid that footgun.
    fn is_number(self, idx: c_int) -> bool;

    /// Returns `true` if the value's actual type is a string.
    /// Note: `ffi::lua_isstring` returns true for numbers due to Lua coercion —
    /// this method uses the type tag directly to avoid that footgun.
    fn is_string(self, idx: c_int) -> bool;

    /// Returns `true` if the value is an integer.
    fn is_integer(self, idx: c_int) -> bool;

    /// Returns `true` if the value is a boolean.
    fn is_boolean(self, idx: c_int) -> bool;

    /// Returns `true` if the value is a vector.
    fn is_vector(self, idx: c_int) -> bool;

    /// Returns `true` if the value is a table.
    fn is_table(self, idx: c_int) -> bool;

    /// Returns `true` if the value is any function (C or Luau).
    fn is_function(self, idx: c_int) -> bool;

    /// Returns `true` if the value is a C function specifically.
    fn is_cfunction(self, idx: c_int) -> bool;

    /// Returns `true` if the value is a Luau (non-C) function specifically.
    fn is_lfunction(self, idx: c_int) -> bool;

    /// Returns `true` if the value is a userdata (tagged or untagged).
    fn is_userdata(self, idx: c_int) -> bool;

    /// Returns `true` if the value is a light userdata.
    fn is_lightuserdata(self, idx: c_int) -> bool;

    /// Returns `true` if the value is a thread (coroutine).
    fn is_thread(self, idx: c_int) -> bool;

    /// Returns `true` if the value is a buffer.
    fn is_buffer(self, idx: c_int) -> bool;

    /// Returns `true` if the value is nil.
    fn is_nil(self, idx: c_int) -> bool;

    /// Returns `true` if the index is out of range (no value there).
    fn is_none(self, idx: c_int) -> bool;

    /// Returns `true` if the index is out of range or the value is nil.
    fn is_none_or_nil(self, idx: c_int) -> bool;

    /// Returns `true` if the table at `idx` is read-only.
    fn is_readonly(self, idx: c_int) -> bool;

    /// Returns `true` if this thread has been reset.
    fn is_thread_reset(self) -> bool;

    /// Returns `true` if this coroutine can yield.
    fn is_yieldable(self) -> bool;

    // --- Type tag (safe: returns LUA_TNONE for invalid idx) ---

    /// Returns the Luau type tag (`LUA_T*`) of the value at `idx`.
    fn type_tag(self, idx: c_int) -> c_int;

    /// Returns the type name of the value at `idx`.
    /// For userdata with a `__type` metafield, returns that string instead of `"userdata"`.
    fn type_name(self, idx: c_int) -> BString;

    // --- Stack reads (safe) ---

    /// Returns the number of values on the stack (index of the top element).
    fn top(self) -> c_int;

    /// Pops `n` values from the stack. Takes `u32` instead of `c_int` so passing a
    /// negative count is a compile error.
    /// 
    /// # Safety
    /// - You may not pop more values than that which currently exist on the stack.
    unsafe fn pop(self, n: u32);

    // --- Conversions ---

    /// Returns the boolean at `idx` as a Rust `bool`.
    /// Avoids the `match b { 0 => false, 1 => true, _ => unreachable!() }` boilerplate.
    /// 
    /// # Safety
    /// - Value at stack `idx` must be a boolean.
    unsafe fn to_boolean(self, idx: c_int) -> bool;

    /// Returns the vector at `idx` as an `(x, y, z)` tuple of `f32`.
    ///
    /// # Safety
    /// - Value at `idx` must be a Luau vector
    unsafe fn get_vector(self, idx: c_int) -> (f32, f32, f32);

    // --- Push helpers (safe: no idx, no type preconditions) ---

    /// Pushes nil onto the stack.
    fn push_nil(self);

    /// Pushes a Rust `bool` onto the stack as a Luau boolean.
    fn push_boolean(self, b: bool);

    /// Pushes a Rust `i32` onto the stack as a Luau integer.
    fn push_integer(self, n: c_int);

    /// Pushes a Luau vector onto the stack.
    fn push_vector(self, x: f32, y: f32, z: f32);

    // --- Table field access ---

    /// Pushes `table[field]` onto the stack and returns its type tag.
    /// 
    /// Works on both tables and userdata.
    /// 
    /// # Safety
    /// - Value at `object_idx` must be a table or userdata.
    unsafe fn get_field(self, object_idx: c_int, field: &CStr) -> c_int;

    /// Pops the top value and sets it as `table[field]`.
    /// 
    /// # Safety
    /// - Value at `table_idx` must be a table or userdata that supports setting fields.
    unsafe fn set_field(self, object_idx: c_int, field: &CStr);

    // --- String helpers ---

    /// Clones the string at `idx` into an owned `BString` using `lua_tolstring`.
    /// Handles strings with interior NUL bytes correctly.
    /// Prefer this over `get_cstring` unless you know the string is NUL-free.
    ///
    /// # Safety
    /// - Value at `idx` must be a Luau string
    unsafe fn get_string(self, idx: c_int) -> BString;

    /// Clones the string at `idx` into an owned `BString` using `lua_tostring`.
    /// Faster than `get_string` but incorrect if the string contains interior NUL bytes.
    ///
    /// # Safety
    /// - Value at `idx` must be a Luau string
    /// - String must not contain interior NUL bytes
    unsafe fn get_cstring(self, idx: c_int) -> BString;

    /// Pushes a string onto the stack using `lua_pushlstring`.
    /// Accepts any `AsRef<str>` — `&str`, `String`, `format!(...)`, etc.
    /// Does not require a NUL terminator or `CString` allocation.
    fn push_str(self, s: impl AsRef<str>);

    /// Pushes a byte slice onto the stack as a Luau string using `lua_pushlstring`.
    /// Useful for `BStr`/`BString` values that may contain non-UTF-8 bytes.
    fn push_lstring(self, s: &[u8]);

    // --- Buffer helpers ---

    /// Allocates a Luau buffer of `len` bytes, copies `data` into it, and pushes it
    /// onto the stack. The buffer size is fixed at allocation.
    fn push_buffer(self, data: &[u8]);

    /// Copies the contents of the Luau buffer at `idx` into an owned `Vec<u8>`.
    ///
    /// # Safety
    /// - Value at `idx` must be a Luau buffer
    unsafe fn get_buffer(self, idx: c_int) -> Vec<u8>;

    /// Returns a mutable slice into the Luau buffer at `idx`.
    /// The slice is valid only while the buffer remains on the stack.
    /// 
    /// The lifetime is a lie, do not hold this buffer
    /// past any Luau call that could collect or move the buffer.
    ///
    /// # Safety
    /// - Value at `idx` must be a Luau buffer
    unsafe fn get_buffer_mut(self, idx: c_int) -> &'static mut [u8];

    /// Pushes a copy of the value at `idx` onto the top of the stack.
    /// Wraps `lua_pushvalue` — distinct from `to_value` which converts to a Rust enum.
    /// 
    /// # Safety
    /// - Value at `idx` must exist.
    unsafe fn push_value(self, idx: c_int);

    // --- Numeric conversions ---

    /// Returns the integer at `idx`. Avoids the `&mut isnum` out-param boilerplate.
    ///
    /// # Safety
    /// - Value at `idx` should be an integer; result is 0 if not convertible.
    unsafe fn to_integer(self, idx: c_int) -> i32;

    /// Returns the number at `idx`. Avoids the `&mut isnum` out-param boilerplate.
    ///
    /// # Safety
    /// - Value at `idx` should be a number; result is 0.0 if not convertible.
    unsafe fn to_number(self, idx: c_int) -> f64;

    // --- Stack manipulation ---

    /// Removes the value at `idx`, shifting elements above it down.
    unsafe fn remove(self, idx: c_int);

    /// Moves the top value to `idx`, shifting elements up to make room.
    unsafe fn insert(self, idx: c_int);

    /// Ensures at least `n` extra stack slots are available.
    /// Panics with `msg` if the stack cannot grow.
    unsafe fn ensure_stack(self, n: c_int, msg: *const c_char);

    /// Returns the #length of the value `idx`.
    unsafe fn object_len(self, idx: c_int) -> usize;

    // --- Registry ---

    /// Pushes `registry[name]` onto the stack and returns its type tag.
    unsafe fn get_named_registry_value(self, name: &CStr) -> c_int;

    /// Pops the top value and stores it as `registry[name]`.
    unsafe fn set_named_registry_value(self, name: &CStr);

    // --- Globals ---

    /// Pushes the global `name` onto the stack and returns its type tag.
    unsafe fn get_global(self, name: &CStr) -> c_int;

    /// Pops the top value and sets it as the global `name`.
    unsafe fn set_global(self, name: &CStr);

    // --- Calling ---

    /// Pops the function at `-(nargs + 1)` and its `nargs` arguments, calls it,
    /// then pushes `nresults` return values onto the stack.
    unsafe fn call(self, nargs: c_int, nresults: c_int);

    /// Like `call` but catches errors instead of propagating them.
    /// Pops the function and its `nargs` arguments; on success pushes `nresults` return values,
    /// on error pushes the error object. Returns 0 on success, non-zero on error.
    /// `errfunc` is the stack index of an error handler, or 0 for none.
    unsafe fn pcall(self, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int;

    // --- Tables ---

    /// Creates an empty table and pushes it onto the stack.
    fn newtable(self);

    /// Creates a table pre-allocated for `narr` array entries and `nrec` hash entries,
    /// and pushes it onto the stack. Takes `u32` instead of `c_int` so negative counts
    /// are a compile error. Both params are silently clamped to `MAX_TABLE_SIZE`.
    fn create_table(self, narr: u32, nrec: u32);

    /// Pops a table from the top and sets it as the metatable of the value at `idx`.
    /// Returns `true` if the metatable was set successfully.
    unsafe fn setmetatable(self, idx: c_int) -> bool;

    /// Pushes the metatable of the value at `idx`. Returns `true` if it has one.
    unsafe fn getmetatable(self, idx: c_int) -> bool;

    /// Pushes `table[n]` onto the stack without invoking metamethods. Returns type tag.
    unsafe fn rawgeti(self, idx: c_int, n: c_int) -> c_int;

    /// Pops a value and sets `table[n]` to it without invoking metamethods.
    unsafe fn rawseti(self, idx: c_int, n: c_int);

    // --- Stack guards ---

    /// Creates a stack guard that asserts the stack changed by exactly `delta` on drop.
    /// Must be called after `sealbindings::initialize()` — panics otherwise.
    fn stack_changes<'a>(self, delta: c_int) -> LuauStackGuard<'a>;

    /// Creates a guard that asserts exactly `n` values were net-pushed on drop.
    /// Must be called after `sealbindings::initialize()` — panics otherwise.
    fn stack_returns<'a>(self, n: c_int) -> LuauStackGuard<'a>;

    /// Creates a guard that asserts the stack is the same height on drop.
    /// Must be called after `sealbindings::initialize()` — panics otherwise.
    fn stack_balanced<'a>(self) -> LuauStackGuard<'a>;

    /// Creates a guard that accepts either 0 values pushed (success) or 1 value pushed
    /// that is a seal WrappedError (failure). Use for functions that return nothing on
    /// success and push an error on failure.
    /// Must be called after `sealbindings::initialize()` — panics otherwise.
    fn stack_returns_none_or_errs<'a>(self) -> LuauStackGuard<'a>;

    /// Creates a guard that accepts either `n` values pushed (success) or 1 value pushed
    /// that is a seal WrappedError (failure). Use for functions that return `n` values on
    /// success and push an error on failure.
    /// Must be called after `sealbindings::initialize()` — panics otherwise.
    fn stack_returns_or_errs<'a>(self, n: c_int) -> LuauStackGuard<'a>;

    // --- Value ---

    /// Reads the value at stack `idx` and returns an owned `SealValue` that's easy to
    /// match against for exhaustiveness. Strings are cloned out of Luau immediately;
    /// buffers come back as a non-owning `SealBuffer` view (see `SealBuffer::to_owned`
    /// and `SealBuffer::as_mut_slice` for cloning/mutating in place).
    ///
    /// Returns `SealValue::None` if `idx` is out-of-range.
    fn to_seal(self, idx: c_int) -> crate::utils::value::SealValue;

    // --- seal helpers ---

    /// Pushes a wrapped `@std/err` error object onto the stack and returns `1`,
    /// so you can write `return state.push_wrapped_error(...)` directly from a `lua_CFunction`.
    /// Accepts any `AsRef<str>` — `&str`, `String`, `format!(...)`, etc.
    ///
    /// This should normally only be used inside a function registered with
    /// `push_wrapped_c_function`, which causes the returned error userdata to be thrown
    /// as a seal error on the Luau side. Without `push_wrapped_c_function`, the error
    /// userdata is returned as a plain value rather than thrown.
    ///
    /// # Panics
    /// Panics if `msg` contains interior NUL bytes.
    fn push_wrapped_error(self, msg: impl AsRef<str>) -> c_int;

    /// Returns `true` if the value at `idx` is a seal `@std/err` WrappedError.
    /// Checks by inspecting the metatable's `__type` field. Safe to call with any idx.
    fn is_wrapped_error(self, idx: c_int) -> bool;

    /// Pushes `func` wrapped by the seal global `ecall` onto the stack.
    /// Allows wrapped errors returned by the function to be thrown as seal errors.
    /// Pass a `Some(&CStr)` debug name to have it show in stack traces and seal's output formatters.
    ///
    /// # Safety
    /// - Stack must have at least 3 free slots
    unsafe fn push_wrapped_c_function(self, func: ffi::lua_CFunction, debug_name: Option<&CStr>);

    /// Assign a function that can return a wrapped error to a table at idx -1.
    /// 
    /// Unlike using `state.push_wrapped_c_function`, this allows setting a debug name,
    /// which in seal is typically a function signature so that a function can be pretty printed
    /// via seal's `print`, `pp`, and formatting utilities in `@std/io/format`.
    /// 
    /// # Usage
    /// 
    /// ```ignore
    /// unsafe {
    ///     state.create_table(0, 1);
    ///     state.set_wrapped_function(
    ///         c"new",
    ///         ThingWrapper::new,
    ///         c"ThingWrapper.new(name: string, sockets: number) -> ThingWrapper"
    ///     )
    /// }
    /// ```
    /// # Safety
    /// - Stack must have at least 3 free slots
    /// - Top of stack must be a table
    unsafe fn set_wrapped_function(self, name: &CStr, f: ffi::lua_CFunction, signature: &CStr);
}

impl StateExt for *mut ffi::lua_State {
    fn is_number(self, idx: c_int) -> bool {
        unsafe { ffi::lua_type(self, idx) == ffi::LUA_TNUMBER() }
    }
    fn is_string(self, idx: c_int) -> bool {
        unsafe { ffi::lua_type(self, idx) == ffi::LUA_TSTRING() }
    }
    fn is_integer(self, idx: c_int) -> bool {
        unsafe { ffi::lua_type(self, idx) == ffi::LUA_TINTEGER() }
    }
    fn is_boolean(self, idx: c_int) -> bool {
        unsafe { ffi::lua_isboolean(self, idx) != 0 }
    }
    fn is_vector(self, idx: c_int) -> bool {
        unsafe { ffi::lua_isvector(self, idx) != 0 }
    }
    fn is_table(self, idx: c_int) -> bool {
        unsafe { ffi::lua_istable(self, idx) != 0 }
    }
    fn is_function(self, idx: c_int) -> bool {
        unsafe { ffi::lua_isfunction(self, idx) != 0 }
    }
    fn is_cfunction(self, idx: c_int) -> bool {
        unsafe { ffi::lua_iscfunction(self, idx) != 0 }
    }
    fn is_lfunction(self, idx: c_int) -> bool {
        unsafe { ffi::lua_isLfunction(self, idx) != 0 }
    }
    fn is_userdata(self, idx: c_int) -> bool {
        unsafe { ffi::lua_isuserdata(self, idx) != 0 }
    }
    fn is_lightuserdata(self, idx: c_int) -> bool {
        unsafe { ffi::lua_islightuserdata(self, idx) != 0 }
    }
    fn is_thread(self, idx: c_int) -> bool {
        unsafe { ffi::lua_isthread(self, idx) != 0 }
    }
    fn is_buffer(self, idx: c_int) -> bool {
        unsafe { ffi::lua_isbuffer(self, idx) != 0 }
    }
    fn is_nil(self, idx: c_int) -> bool {
        unsafe { ffi::lua_isnil(self, idx) != 0 }
    }
    fn is_none(self, idx: c_int) -> bool {
        unsafe { ffi::lua_isnone(self, idx) != 0 }
    }
    fn is_none_or_nil(self, idx: c_int) -> bool {
        unsafe { ffi::lua_isnoneornil(self, idx) != 0 }
    }
    fn is_readonly(self, idx: c_int) -> bool {
        unsafe { ffi::lua_getreadonly(self, idx) != 0 }
    }
    fn is_thread_reset(self) -> bool {
        unsafe { ffi::lua_isthreadreset(self) != 0 }
    }
    fn is_yieldable(self) -> bool {
        unsafe { ffi::lua_isyieldable(self) != 0 }
    }
    fn type_tag(self, idx: c_int) -> c_int {
        unsafe { ffi::lua_type(self, idx) }
    }
    fn type_name(self, idx: c_int) -> BString {
        unsafe {
            let ptr = ffi::luaL_typename(self, idx);
            BString::from(std::ffi::CStr::from_ptr(ptr).to_bytes())
        }
    }
    fn top(self) -> c_int {
        unsafe { ffi::lua_gettop(self) }
    }
    unsafe fn pop(self, n: u32) {
        unsafe { ffi::lua_pop(self, n as c_int) }
    }
    unsafe fn to_boolean(self, idx: c_int) -> bool {
        unsafe { ffi::lua_toboolean(self, idx) != 0 }
    }
    unsafe fn get_vector(self, idx: c_int) -> (f32, f32, f32) {
        let ptr = unsafe { ffi::lua_tovector(self, idx) };
        unsafe { (*ptr, *ptr.add(1), *ptr.add(2)) }
    }
    fn push_nil(self) {
        unsafe { ffi::lua_pushnil(self) }
    }
    fn push_boolean(self, b: bool) {
        unsafe { ffi::lua_pushboolean(self, b as c_int) }
    }
    fn push_integer(self, n: c_int) {
        unsafe { ffi::lua_pushinteger(self, n) }
    }
    fn push_vector(self, x: f32, y: f32, z: f32) {
        unsafe { ffi::lua_pushvector(self, x, y, z) }
    }
    unsafe fn get_field(self, object_idx: c_int, field: &CStr) -> c_int {
        unsafe { ffi::lua_getfield(self, object_idx, field.as_ptr()) }
    }
    unsafe fn set_field(self, object_idx: c_int, field: &CStr) {
        unsafe { ffi::lua_setfield(self, object_idx, field.as_ptr()) }
    }
    unsafe fn get_string(self, idx: c_int) -> BString {
        unsafe { BString::clone_lstring_from_stack(self, idx) }
    }
    unsafe fn get_cstring(self, idx: c_int) -> BString {
        unsafe { BString::clone_cstring_from_stack(self, idx) }
    }
    fn push_str(self, s: impl AsRef<str>) {
        let s = s.as_ref();
        unsafe { ffi::lua_pushlstring(self, s.as_ptr() as *const c_char, s.len()) }
    }
    fn push_lstring(self, s: &[u8]) {
        unsafe { ffi::lua_pushlstring(self, s.as_ptr() as *const c_char, s.len()) }
    }
    fn push_buffer(self, data: &[u8]) {
        let ptr = unsafe { ffi::lua_newbuffer(self, data.len()) } as *mut u8;
        unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, data.len()) };
    }
    unsafe fn get_buffer(self, idx: c_int) -> Vec<u8> {
        let mut len: usize = 0;
        let ptr = unsafe { ffi::lua_tobuffer(self, idx, &mut len) } as *const u8;
        unsafe { std::slice::from_raw_parts(ptr, len).to_vec() }
    }
    unsafe fn get_buffer_mut(self, idx: c_int) -> &'static mut [u8] {
        let mut len: usize = 0;
        let ptr = unsafe { ffi::lua_tobuffer(self, idx, &mut len) } as *mut u8;
        unsafe { std::slice::from_raw_parts_mut(ptr, len) }
    }
    unsafe fn push_value(self, idx: c_int) {
        unsafe { ffi::lua_pushvalue(self, idx) }
    }
    unsafe fn to_integer(self, idx: c_int) -> i32 {
        let mut isnum: c_int = 0;
        unsafe { ffi::lua_tointegerx(self, idx, &mut isnum) }
    }
    unsafe fn to_number(self, idx: c_int) -> f64 {
        let mut isnum: c_int = 0;
        unsafe { ffi::lua_tonumberx(self, idx, &mut isnum) as f64 }
    }
    unsafe fn remove(self, idx: c_int) {
        unsafe { ffi::lua_remove(self, idx) }
    }
    unsafe fn insert(self, idx: c_int) {
        unsafe { ffi::lua_insert(self, idx) }
    }
    unsafe fn ensure_stack(self, n: c_int, msg: *const c_char) {
        unsafe { ffi::luaL_checkstack(self, n, msg) }
    }
    unsafe fn object_len(self, idx: c_int) -> usize {
        unsafe { ffi::lua_objlen(self, idx) }
    }
    unsafe fn get_named_registry_value(self, name: &CStr) -> c_int {
        unsafe { ffi::lua_getfield(self, ffi::LUA_REGISTRYINDEX(), name.as_ptr()) }
    }
    unsafe fn set_named_registry_value(self, name: &CStr) {
        unsafe { ffi::lua_setfield(self, ffi::LUA_REGISTRYINDEX(), name.as_ptr()) }
    }
    unsafe fn get_global(self, name: &CStr) -> c_int {
        unsafe { ffi::lua_getglobal(self, name.as_ptr()) }
    }
    unsafe fn set_global(self, name: &CStr) {
        unsafe { ffi::lua_setglobal(self, name.as_ptr()) }
    }
    unsafe fn call(self, nargs: c_int, nresults: c_int) {
        unsafe { ffi::lua_call(self, nargs, nresults) }
    }
    unsafe fn pcall(self, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int {
        unsafe { ffi::lua_pcall(self, nargs, nresults, errfunc) }
    }
    fn newtable(self) {
        unsafe { ffi::lua_newtable(self) }
    }
    fn create_table(self, narr: u32, nrec: u32) {
        let narr = narr.min(MAX_TABLE_SIZE as u32) as c_int;
        let nrec = nrec.min(MAX_TABLE_SIZE as u32) as c_int;
        unsafe { ffi::lua_createtable(self, narr, nrec) }
    }
    unsafe fn setmetatable(self, idx: c_int) -> bool {
        unsafe { ffi::lua_setmetatable(self, idx) != 0 }
    }
    unsafe fn getmetatable(self, idx: c_int) -> bool {
        unsafe { ffi::lua_getmetatable(self, idx) != 0 }
    }
    unsafe fn rawgeti(self, idx: c_int, n: c_int) -> c_int {
        unsafe { ffi::lua_rawgeti(self, idx, n) }
    }
    unsafe fn rawseti(self, idx: c_int, n: c_int) {
        unsafe { ffi::lua_rawseti(self, idx, n) }
    }
    #[track_caller]
    fn stack_changes<'a>(self, delta: c_int) -> LuauStackGuard<'a> {
        LuauStackGuard::new(self, delta)
    }
    #[track_caller]
    fn stack_returns<'a>(self, n: c_int) -> LuauStackGuard<'a> {
        LuauStackGuard::returning(self, n)
    }
    #[track_caller]
    fn stack_balanced<'a>(self) -> LuauStackGuard<'a> {
        LuauStackGuard::balanced(self)
    }
    #[track_caller]
    fn stack_returns_none_or_errs<'a>(self) -> LuauStackGuard<'a> {
        LuauStackGuard::none_or_error(self)
    }
    #[track_caller]
    fn stack_returns_or_errs<'a>(self, n: c_int) -> LuauStackGuard<'a> {
        LuauStackGuard::returning_or_error(self, n)
    }
    fn to_seal(self, idx: c_int) -> crate::utils::value::SealValue {
        unsafe { crate::utils::value::create(self, idx) }
    }
    fn push_wrapped_error(self, msg: impl AsRef<str>) -> c_int {
        crate::push_wrapped_error(self, msg.as_ref())
    }
    fn is_wrapped_error(self, idx: c_int) -> bool {
        if self.type_tag(idx) != ffi::LUA_TUSERDATA() {
            return false;
        }
        if unsafe { ffi::lua_getmetatable(self, idx) } == 0 {
            return false;
        }
        unsafe { ffi::lua_getfield(self, -1, c"__type".as_ptr()) };
        let result = matches!(self.to_seal(-1), crate::utils::value::SealValue::String(ref s) if s == b"error");
        unsafe { ffi::lua_pop(self, 2) };
        result
    }
    unsafe fn push_wrapped_c_function(self, func: ffi::lua_CFunction, debug_name: Option<&CStr>) {
        unsafe { crate::push_wrapped_c_function(self, func, debug_name) }
    }
    unsafe fn set_wrapped_function(self, name: &CStr, f: ffi::lua_CFunction, signature: &CStr) {
        unsafe {
            ffi::lua_getglobal(self, c"ecall".as_ptr());
            ffi::lua_pushcfunctiond(self, f, signature.as_ptr());
            ffi::lua_call(self, 1, 1);
            ffi::lua_setfield(self, -2, name.as_ptr());
        }
    }
}
