#![allow(nonstandard_style)]

pub mod api;
pub mod types;

pub use types::*;
use api::LuauApi;

use std::ffi::{c_double, c_float, c_void, c_char, c_int};

use crate::LUAU_API;

/// Returns the initialized Luau API vtable.
///
/// # Panics
/// Panics if `initialize_luau_api` has not been called.
pub fn luau_api() -> &'static LuauApi {
    LUAU_API
        .get()
        .expect("Luau API not initialized; call ffi::initialize first")
}

// --- luau constants (from lua.h) ---

pub const LUA_MULTRET: c_int = -1;

pub const LUA_UTAG_LIMIT: c_int = 128;
pub const LUA_LUTAG_LIMIT: c_int = 128;

pub const LUA_REGISTRYINDEX: c_int = -1000000 - 2000;
pub const LUA_ENVIRONINDEX: c_int = -1000000 - 2001;
pub const LUA_GLOBALSINDEX: c_int = -1000000 - 2002;

pub const LUA_OK: c_int = 0;
pub const LUA_YIELD: c_int = 1;
pub const LUA_ERRRUN: c_int = 2;
pub const LUA_ERRSYNTAX: c_int = 3;
pub const LUA_ERRMEM: c_int = 4;
pub const LUA_ERRERR: c_int = 5;

pub const LUA_TNONE: c_int = -1;
pub const LUA_TNIL: c_int = 0;
pub const LUA_TBOOLEAN: c_int = 1;
pub const LUA_TLIGHTUSERDATA: c_int = 2;
pub const LUA_TNUMBER: c_int = 3;
pub const LUA_TVECTOR: c_int = 4;
pub const LUA_TSTRING: c_int = 5;
pub const LUA_TTABLE: c_int = 6;
pub const LUA_TFUNCTION: c_int = 7;
pub const LUA_TUSERDATA: c_int = 8;
pub const LUA_TTHREAD: c_int = 9;
pub const LUA_TBUFFER: c_int = 10;

pub const LUA_MINSTACK: c_int = 20;

pub const LUA_GCSTOP: c_int = 0;
pub const LUA_GCRESTART: c_int = 1;
pub const LUA_GCCOLLECT: c_int = 2;
pub const LUA_GCCOUNT: c_int = 3;
pub const LUA_GCCOUNTB: c_int = 4;
pub const LUA_GCISRUNNING: c_int = 5;
pub const LUA_GCSTEP: c_int = 6;
pub const LUA_GCSETGOAL: c_int = 7;
pub const LUA_GCSETSTEPMUL: c_int = 8;
pub const LUA_GCSETSTEPSIZE: c_int = 9;

pub const LUA_NOREF: c_int = -1;
pub const LUA_REFNIL: c_int = 0;

pub const LUA_IDSIZE: usize = 256;

//
// State manipulation
//

/// Creates a new Luau state using the given allocator.
///
/// # Safety
/// - `f` must be a valid allocator function.
/// - `ud` must be valid for the allocator’s expectations.
pub unsafe fn lua_newstate(f: lua_Alloc, ud: *mut c_void) -> *mut lua_State {
    unsafe { (luau_api().lua_newstate)(f, ud) }
}

/// Closes a Luau state and frees all associated resources.
///
/// # Safety
/// - `state` must be a valid, live Luau state created by `lua_newstate`.
pub unsafe fn lua_close(state: *mut lua_State) {
    unsafe { (luau_api().lua_close)(state) }
}

/// Creates a new thread (coroutine) from `state`.
///
/// # Safety
/// - `state` must be a valid Luau state.
pub unsafe fn lua_newthread(state: *mut lua_State) -> *mut lua_State {
    unsafe { (luau_api().lua_newthread)(state) }
}

/// Returns the main thread associated with `state`.
///
/// # Safety
/// - `state` must be a valid Luau state.
pub unsafe fn lua_mainthread(state: *mut lua_State) -> *mut lua_State {
    unsafe { (luau_api().lua_mainthread)(state) }
}

/// Resets a Luau thread to its initial state.
///
/// # Safety
/// - `state` must be a valid Luau thread.
pub unsafe fn lua_resetthread(state: *mut lua_State) {
    unsafe { (luau_api().lua_resetthread)(state) }
}

/// Returns non‑zero if the thread is in a reset state.
///
/// # Safety
/// - `state` must be a valid Luau thread.
pub unsafe fn lua_isthreadreset(state: *mut lua_State) -> c_int {
    unsafe { (luau_api().lua_isthreadreset)(state) }
}

//
// Basic stack manipulation
//

/// Converts a stack index to an absolute index.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid or pseudo index.
pub unsafe fn lua_absindex(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_absindex)(state, idx) }
}

/// Returns the index of the top element in the stack.
///
/// # Safety
/// - `state` must be valid.
pub unsafe fn lua_gettop(state: *mut lua_State) -> c_int {
    unsafe { (luau_api().lua_gettop)(state) }
}

/// Sets the stack top to `idx`, discarding or growing as needed.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index or 0.
pub unsafe fn lua_settop(state: *mut lua_State, idx: c_int) {
    unsafe { (luau_api().lua_settop)(state, idx) }
}

/// Pushes a copy of the value at `idx` onto the stack.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must refer to a valid stack index.
pub unsafe fn lua_pushvalue(state: *mut lua_State, idx: c_int) {
    unsafe { (luau_api().lua_pushvalue)(state, idx) }
}

/// Removes the value at `idx`, shifting higher elements down.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must refer to a valid stack index.
pub unsafe fn lua_remove(state: *mut lua_State, idx: c_int) {
    unsafe { (luau_api().lua_remove)(state, idx) }
}

/// Inserts the value at the top into position `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must refer to a valid stack index.
pub unsafe fn lua_insert(state: *mut lua_State, idx: c_int) {
    unsafe { (luau_api().lua_insert)(state, idx) }
}

/// Replaces the value at `idx` with the top value and pops it.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must refer to a valid stack index.
pub unsafe fn lua_replace(state: *mut lua_State, idx: c_int) {
    unsafe { (luau_api().lua_replace)(state, idx) }
}

/// Ensures the stack has at least `sz` free slots.
///
/// # Safety
/// - `state` must be valid.
pub unsafe fn lua_checkstack(state: *mut lua_State, sz: c_int) -> c_int {
    unsafe { (luau_api().lua_checkstack)(state, sz) }
}

/// Ensures raw stack space without invoking hooks.
///
/// # Safety
/// - `state` must be valid.
pub unsafe fn lua_rawcheckstack(state: *mut lua_State, sz: c_int) {
    unsafe { (luau_api().lua_rawcheckstack)(state, sz) }
}

/// Moves `n` values from `from` to `to`.
///
/// # Safety
/// - Both states must be valid and share the same global state.
/// - Stack indices must be valid in `from`.
pub unsafe fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int) {
    unsafe { (luau_api().lua_xmove)(from, to, n) }
}

/// Pushes the value at `idx` in `from` onto `to`.
///
/// # Safety
/// - Both states must be valid and compatible.
/// - `idx` must be valid in `from`.
pub unsafe fn lua_xpush(from: *mut lua_State, to: *mut lua_State, idx: c_int) {
    unsafe { (luau_api().lua_xpush)(from, to, idx) }
}

//
// Access functions (stack -> C)
//

/// Returns non‑zero if the value at `idx` is a number or convertible.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
pub unsafe fn lua_isnumber(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_isnumber)(state, idx) }
}

/// Returns non‑zero if the value at `idx` is a string or convertible.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
pub unsafe fn lua_isstring(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_isstring)(state, idx) }
}

/// Returns non‑zero if the value at `idx` is a C function.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
pub unsafe fn lua_iscfunction(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_iscfunction)(state, idx) }
}

/// Returns non‑zero if the value at `idx` is a Luau function.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
pub unsafe fn lua_isLfunction(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_isLfunction)(state, idx) }
}

/// Returns non‑zero if the value at `idx` is userdata.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
pub unsafe fn lua_isuserdata(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_isuserdata)(state, idx) }
}

/// Returns the type of the value at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
pub unsafe fn lua_type(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_type)(state, idx) }
}

/// Returns the name of a Luau type, or "userdata" for any userdata.
///
/// # Safety
/// - `state` must be valid.
/// - `tp` must be a valid type tag.
/// - Returned pointer is owned by Luau.
pub unsafe fn lua_typename(state: *mut lua_State, tp: c_int) -> *const c_char {
    unsafe { (luau_api().lua_typename)(state, tp) }
}

/// Compares two values using Luau equality.
///
/// # Safety
/// - `state` must be valid.
/// - `idx1` and `idx2` must be valid stack indices.
pub unsafe fn lua_equal(state: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int {
    unsafe { (luau_api().lua_equal)(state, idx1, idx2) }
}

/// Compares two values using raw equality.
///
/// # Safety
/// - `state` must be valid.
/// - `idx1` and `idx2` must be valid stack indices.
pub unsafe fn lua_rawequal(state: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int {
    unsafe { (luau_api().lua_rawequal)(state, idx1, idx2) }
}

/// Returns non‑zero if value at `idx1` is less than value at `idx2`.
///
/// # Safety
/// - `state` must be valid.
/// - Indices must be valid and comparable.
pub unsafe fn lua_lessthan(state: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int {
    unsafe { (luau_api().lua_lessthan)(state, idx1, idx2) }
}

/// Converts value at `idx` to a number, optionally reporting success.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
/// - `isnum` may be null or a valid pointer.
pub unsafe fn lua_tonumberx(
    state: *mut lua_State,
    idx: c_int,
    isnum: *mut c_int,
) -> lua_Number {
    unsafe { (luau_api().lua_tonumberx)(state, idx, isnum) }
}

/// Converts value at `idx` to an integer, optionally reporting success.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
/// - `isnum` may be null or a valid pointer.
pub unsafe fn lua_tointegerx_(
    state: *mut lua_State,
    idx: c_int,
    isnum: *mut c_int,
) -> c_int {
    unsafe { (luau_api().lua_tointegerx_)(state, idx, isnum) }
}

/// Converts value at `idx` to an unsigned integer.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
/// - `isnum` may be null or a valid pointer.
pub unsafe fn lua_tounsignedx(
    state: *mut lua_State,
    idx: c_int,
    isnum: *mut c_int,
) -> lua_Unsigned {
    unsafe { (luau_api().lua_tounsignedx)(state, idx, isnum) }
}

/// Returns a pointer to a vector value at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
/// - Returned pointer is owned by Luau.
pub unsafe fn lua_tovector(state: *mut lua_State, idx: c_int) -> *const c_float {
    unsafe { (luau_api().lua_tovector)(state, idx) }
}

/// Converts value at `idx` to a boolean.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
pub unsafe fn lua_toboolean(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_toboolean)(state, idx) }
}

/// Converts value at `idx` to a string and returns pointer + length.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
/// - `len` must be a valid pointer.
/// - Returned pointer is owned by Luau.
pub unsafe fn lua_tolstring(
    state: *mut lua_State,
    idx: c_int,
    len: *mut usize,
) -> *const c_char {
    unsafe { (luau_api().lua_tolstring)(state, idx, len) }
}

/// Returns string and atom for value at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` and `atom` must be valid.
/// - Returned pointer is owned by Luau.
pub unsafe fn lua_tostringatom(
    state: *mut lua_State,
    idx: c_int,
    atom: *mut c_int,
) -> *const c_char {
    unsafe { (luau_api().lua_tostringatom)(state, idx, atom) }
}

/// Returns the namecall atom string for `atom`.
///
/// # Safety
/// - `state` must be valid.
/// - `atom` must be a valid pointer.
/// - Returned pointer is owned by Luau.
pub unsafe fn lua_namecallatom(
    state: *mut lua_State,
    atom: *mut c_int,
) -> *const c_char {
    unsafe { (luau_api().lua_namecallatom)(state, atom) }
}

/// Returns the length of the value at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
pub unsafe fn lua_objlen(state: *mut lua_State, idx: c_int) -> usize {
    unsafe { (luau_api().lua_objlen)(state, idx) }
}

/// Returns the C function at `idx`, if any.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
pub unsafe fn lua_tocfunction(
    state: *mut lua_State,
    idx: c_int,
) -> Option<lua_CFunction> {
    unsafe { (luau_api().lua_tocfunction)(state, idx) }
}

/// Returns a light userdata pointer at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be a valid stack index.
pub unsafe fn lua_tolightuserdata(state: *mut lua_State, idx: c_int) -> *mut c_void {
    unsafe { (luau_api().lua_tolightuserdata)(state, idx) }
}

/// Returns a tagged light userdata pointer at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
/// - Tag must match how the value was created.
pub unsafe fn lua_tolightuserdatatagged(
    state: *mut lua_State,
    idx: c_int,
    tag: c_int,
) -> *mut c_void {
    unsafe { (luau_api().lua_tolightuserdatatagged)(state, idx, tag) }
}

/// Returns a userdata pointer at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_touserdata(state: *mut lua_State, idx: c_int) -> *mut c_void {
    unsafe { (luau_api().lua_touserdata)(state, idx) }
}

/// Returns a tagged userdata pointer at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
/// - Tag must match how the value was created.
pub unsafe fn lua_touserdatatagged(
    state: *mut lua_State,
    idx: c_int,
    tag: c_int,
) -> *mut c_void {
    unsafe { (luau_api().lua_touserdatatagged)(state, idx, tag) }
}

/// Returns the userdata tag at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_userdatatag(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_userdatatag)(state, idx) }
}

/// Returns the light userdata tag at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_lightuserdatatag(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_lightuserdatatag)(state, idx) }
}

/// Returns the thread at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_tothread(state: *mut lua_State, idx: c_int) -> *mut lua_State {
    unsafe { (luau_api().lua_tothread)(state, idx) }
}

/// Returns a buffer pointer and length at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` and `len` must be valid.
/// - Returned pointer is owned by Luau.
pub unsafe fn lua_tobuffer(
    state: *mut lua_State,
    idx: c_int,
    len: *mut usize,
) -> *mut c_void {
    unsafe { (luau_api().lua_tobuffer)(state, idx, len) }
}

/// Returns a raw pointer identifying the value at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
/// - Pointer is only meaningful for identity comparisons.
pub unsafe fn lua_topointer(state: *mut lua_State, idx: c_int) -> *const c_void {
    unsafe { (luau_api().lua_topointer)(state, idx) }
}

//
// Push functions (C -> stack)
//

/// Pushes nil onto the stack.
///
/// # Safety
/// - `state` must be valid.
/// - Stack must have space for one value.
pub unsafe fn lua_pushnil(state: *mut lua_State) {
    unsafe { (luau_api().lua_pushnil)(state) }
}

/// Pushes a number onto the stack.
///
/// # Safety
/// - `state` must be valid.
/// - Stack must have space for one value.
pub unsafe fn lua_pushnumber(state: *mut lua_State, n: lua_Number) {
    unsafe { (luau_api().lua_pushnumber)(state, n) }
}

/// Pushes an integer onto the stack.
///
/// # Safety
/// - `state` must be valid.
/// - Stack must have space for one value.
pub unsafe fn lua_pushinteger_(state: *mut lua_State, n: c_int) {
    unsafe { (luau_api().lua_pushinteger_)(state, n) }
}

/// Pushes an unsigned integer onto the stack.
///
/// # Safety
/// - `state` must be valid.
/// - Stack must have space for one value.
pub unsafe fn lua_pushunsigned(state: *mut lua_State, n: lua_Unsigned) {
    unsafe { (luau_api().lua_pushunsigned)(state, n) }
}

/// Pushes a vector value onto the stack.
///
/// # Safety
/// - `state` must be valid.
/// - Stack must have space for one value.
pub unsafe fn lua_pushvector(state: *mut lua_State, x: c_float, y: c_float, z: c_float) {
    unsafe { (luau_api().lua_pushvector)(state, x, y, z) }
}

/// Pushes a string from raw bytes and length.
///
/// # Safety
/// - `state` must be valid.
/// - `s` must point to `l` bytes of readable memory.
pub unsafe fn lua_pushlstring_(state: *mut lua_State, s: *const c_char, l: usize) {
    unsafe { (luau_api().lua_pushlstring_)(state, s, l) }
}

/// Pushes a string from a NUL‑terminated C string.
///
/// # Safety
/// - `state` must be valid.
/// - `s` must be a valid, NUL‑terminated C string.
pub unsafe fn lua_pushstring(state: *mut lua_State, s: *const c_char) {
    unsafe { (luau_api().lua_pushstring_)(state, s) }
}

/// Pushes a C closure with upvalues and optional continuation.
///
/// # Safety
/// - `state` must be valid.
/// - `f` and `cont` must be valid function pointers.
/// - Stack must contain `nup` upvalues.
pub unsafe fn lua_pushcclosurek(
    state: *mut lua_State,
    f: lua_CFunction,
    debugname: *const c_char,
    nup: c_int,
    cont: Option<lua_Continuation>,
) {
    unsafe { (luau_api().lua_pushcclosurek)(state, f, debugname, nup, cont) }
}

/// Pushes a boolean onto the stack.
///
/// # Safety
/// - `state` must be valid.
/// - Stack must have space for one value.
pub unsafe fn lua_pushboolean(state: *mut lua_State, b: c_int) {
    unsafe { (luau_api().lua_pushboolean)(state, b) }
}

/// Pushes the current thread onto the stack.
///
/// # Safety
/// - `state` must be valid.
/// - Stack must have space for one value.
pub unsafe fn lua_pushthread(state: *mut lua_State) -> c_int {
    unsafe { (luau_api().lua_pushthread)(state) }
}

/// Pushes a tagged light userdata pointer.
///
/// # Safety
/// - `state` must be valid.
/// - `p` must be a valid pointer for your usage.
/// - Tag must be consistent with your protocol.
pub unsafe fn lua_pushlightuserdatatagged(
    state: *mut lua_State,
    p: *mut c_void,
    tag: c_int,
) {
    unsafe { (luau_api().lua_pushlightuserdatatagged)(state, p, tag) }
}

/// Allocates tagged userdata of size `sz`.
///
/// # Safety
/// - `state` must be valid.
/// - `sz` must be reasonable; Luau will allocate that many bytes.
pub unsafe fn lua_newuserdatatagged(
    state: *mut lua_State,
    sz: usize,
    tag: c_int,
) -> *mut c_void {
    unsafe { (luau_api().lua_newuserdatatagged)(state, sz, tag) }
}

/// Allocates tagged userdata with metatable.
///
/// # Safety
/// - There must be a metatable for `tag`; set one via `ffi::lua_setuserdatametatable`
/// - `state` must be valid.
/// - `sz` must be reasonable.
pub unsafe fn lua_newuserdatataggedwithmetatable(
    state: *mut lua_State,
    sz: usize,
    tag: c_int,
) -> *mut c_void {
    unsafe { (luau_api().lua_newuserdatataggedwithmetatable)(state, sz, tag) }
}

/// Allocates userdata with destructor.
///
/// # Safety
/// - `state` must be valid.
/// - `sz` must be reasonable.
/// - `dtor` must be a valid destructor function.
pub unsafe fn lua_newuserdatadtor(
    state: *mut lua_State,
    sz: usize,
    dtor: lua_Destructor,
) -> *mut c_void {
    unsafe { (luau_api().lua_newuserdatadtor)(state, sz, dtor) }
}

/// Allocates a new buffer of size `sz`.
///
/// # Safety
/// - `state` must be valid.
/// - `sz` must be reasonable.
pub unsafe fn lua_newbuffer(state: *mut lua_State, sz: usize) -> *mut c_void {
    unsafe { (luau_api().lua_newbuffer)(state, sz) }
}

//
// Get functions (Luau -> stack)
//

/// Performs `t[k]` and pushes the result.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` and key on stack must be valid.
pub unsafe fn lua_gettable(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_gettable)(state, idx) }
}

/// Pushes `t[k]` where `t` is at `idx` and `k` is a string.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
/// - `k` must be a valid C string.
pub unsafe fn lua_getfield(state: *mut lua_State, idx: c_int, k: *const c_char) -> c_int {
    unsafe { (luau_api().lua_getfield)(state, idx, k) }
}

/// Raw field access without metamethods.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
/// - `k` must be a valid C string.
pub unsafe fn lua_rawgetfield(state: *mut lua_State, idx: c_int, k: *const c_char) -> c_int {
    unsafe { (luau_api().lua_rawgetfield)(state, idx, k) }
}

/// Raw table access using key on stack.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` and key on stack must be valid.
pub unsafe fn lua_rawget(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_rawget)(state, idx) }
}

/// Raw integer index access.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_rawgeti_(state: *mut lua_State, idx: c_int, n: c_int) -> c_int {
    unsafe { (luau_api().lua_rawgeti_)(state, idx, n) }
}

/// Raw pointer‑keyed access with tag.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
/// - `p` and `tag` must match how entries were stored.
pub unsafe fn lua_rawgetptagged(
    state: *mut lua_State,
    idx: c_int,
    p: *const c_void,
    tag: c_int,
) -> c_int {
    unsafe { (luau_api().lua_rawgetptagged)(state, idx, p, tag) }
}

/// Creates a new table with array/hash slots.
///
/// # Safety
/// - `state` must be valid.
/// - Stack must have space for one value.
pub unsafe fn lua_createtable(state: *mut lua_State, narr: c_int, nrec: c_int) {
    unsafe { (luau_api().lua_createtable)(state, narr, nrec) }
}

/// Sets read‑only flag on value at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_setreadonly(state: *mut lua_State, idx: c_int, enabled: c_int) {
    unsafe { (luau_api().lua_setreadonly)(state, idx, enabled) }
}

/// Gets read‑only flag on value at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_getreadonly(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_getreadonly)(state, idx) }
}

/// Sets safe environment flag on value at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_setsafeenv(state: *mut lua_State, idx: c_int, enabled: c_int) {
    unsafe { (luau_api().lua_setsafeenv)(state, idx, enabled) }
}

/// Gets metatable of value at `objindex`.
///
/// # Safety
/// - `state` must be valid.
/// - `objindex` must be valid.
pub unsafe fn lua_getmetatable(state: *mut lua_State, objindex: c_int) -> c_int {
    unsafe { (luau_api().lua_getmetatable)(state, objindex) }
}

/// Gets environment table of value at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_getfenv(state: *mut lua_State, idx: c_int) {
    unsafe { (luau_api().lua_getfenv)(state, idx) }
}

//
// Set functions (stack -> Luau)
//

/// Performs `t[k] = v` with table at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` and stack values must be valid.
pub unsafe fn lua_settable(state: *mut lua_State, idx: c_int) {
    unsafe { (luau_api().lua_settable)(state, idx) }
}

/// Sets `t[k] = v` where `t` is at `idx` and `k` is a string.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
/// - `k` must be a valid C string.
pub unsafe fn lua_setfield(state: *mut lua_State, idx: c_int, k: *const c_char) {
    unsafe { (luau_api().lua_setfield)(state, idx, k) }
}

/// Raw table set using key and value on stack.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` and stack values must be valid.
pub unsafe fn lua_rawset(state: *mut lua_State, idx: c_int) {
    unsafe { (luau_api().lua_rawset)(state, idx) }
}

/// Raw integer index set.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` and value on stack must be valid.
pub unsafe fn lua_rawseti_(state: *mut lua_State, idx: c_int, n: c_int) {
    unsafe { (luau_api().lua_rawseti_)(state, idx, n) }
}

/// Raw pointer‑keyed set with tag.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` and value on stack must be valid.
/// - `p` and `tag` must match your protocol.
pub unsafe fn lua_rawsetptagged(
    state: *mut lua_State,
    idx: c_int,
    p: *const c_void,
    tag: c_int,
) {
    unsafe { (luau_api().lua_rawsetptagged)(state, idx, p, tag) }
}

/// Sets metatable of value at `objindex`.
///
/// # Safety
/// - `state` must be valid.
/// - `objindex` and metatable on stack must be valid.
pub unsafe fn lua_setmetatable(state: *mut lua_State, objindex: c_int) -> c_int {
    unsafe { (luau_api().lua_setmetatable)(state, objindex) }
}

/// Sets environment table of value at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` and table on stack must be valid.
pub unsafe fn lua_setfenv(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_setfenv)(state, idx) }
}

//
// load / call
//

/// Loads a Luau chunk from memory.
///
/// # Safety
/// - `state` must be valid.
/// - `data` must point to `size` bytes of readable memory.
/// - `chunkname` must be a valid C string.
pub unsafe fn luau_load(
    state: *mut lua_State,
    chunkname: *const c_char,
    data: *const c_char,
    size: usize,
    env: c_int,
) -> c_int {
    unsafe { (luau_api().luau_load)(state, chunkname, data, size, env) }
}

/// Calls a Luau function with `nargs` arguments and `nresults` results.
///
/// # Safety
/// - `state` must be valid.
/// - Stack must contain function and arguments as expected by Luau.
pub unsafe fn lua_call(state: *mut lua_State, nargs: c_int, nresults: c_int) {
    unsafe { (luau_api().lua_call)(state, nargs, nresults) }
}

/// Protected call of a Luau function.
///
/// # Safety
/// - `state` must be valid.
/// - Stack layout must match Luau’s expectations.
pub unsafe fn lua_pcall(
    state: *mut lua_State,
    nargs: c_int,
    nresults: c_int,
    errfunc: c_int,
) -> c_int {
    unsafe { (luau_api().lua_pcall)(state, nargs, nresults, errfunc) }
}

/// Calls a C function in protected mode.
///
/// # Safety
/// - `state` must be valid.
/// - `f` must be a valid C function.
/// - `ud` must be valid for `f`.
pub unsafe fn lua_cpcall(
    state: *mut lua_State,
    f: lua_CFunction,
    ud: *mut c_void,
) -> c_int {
    unsafe { (luau_api().lua_cpcall)(state, f, ud) }
}

//
// Coroutine
//

/// Yields from the current coroutine with `nresults` results.
///
/// # Safety
/// - `state` must be a yieldable Luau thread.
pub unsafe fn lua_yield(state: *mut lua_State, nresults: c_int) -> c_int {
    unsafe { (luau_api().lua_yield)(state, nresults) }
}

/// Triggers a breakpoint‑style yield.
///
/// # Safety
/// - `state` must be a valid Luau thread.
pub unsafe fn lua_break(state: *mut lua_State) -> c_int {
    unsafe { (luau_api().lua_break)(state) }
}

/// Resumes a coroutine from `from` with `narg` arguments.
///
/// # Safety
/// - Both states must be valid and related.
/// - Stack layout must match Luau’s expectations.
pub unsafe fn lua_resume_(state: *mut lua_State, from: *mut lua_State, narg: c_int) -> c_int {
    unsafe { (luau_api().lua_resume_)(state, from, narg) }
}

/// Resumes a coroutine with an error.
///
/// # Safety
/// - Both states must be valid and related.
pub unsafe fn lua_resumeerror(state: *mut lua_State, from: *mut lua_State) -> c_int {
    unsafe { (luau_api().lua_resumeerror)(state, from) }
}

/// Returns the status of `state`.
///
/// # Safety
/// - `state` must be valid.
pub unsafe fn lua_status(state: *mut lua_State) -> c_int {
    unsafe { (luau_api().lua_status)(state) }
}

/// Returns non‑zero if `state` is yieldable.
///
/// # Safety
/// - `state` must be valid.
pub unsafe fn lua_isyieldable(state: *mut lua_State) -> c_int {
    unsafe { (luau_api().lua_isyieldable)(state) }
}

/// Returns thread‑local data pointer.
///
/// # Safety
/// - `state` must be valid.
pub unsafe fn lua_getthreaddata(state: *mut lua_State) -> *mut c_void {
    unsafe { (luau_api().lua_getthreaddata)(state) }
}

/// Sets thread‑local data pointer.
///
/// # Safety
/// - `state` must be valid.
/// - `data` must remain valid for the thread’s lifetime or until changed.
pub unsafe fn lua_setthreaddata(state: *mut lua_State, data: *mut c_void) {
    unsafe { (luau_api().lua_setthreaddata)(state, data) }
}

//
// GC
//

/// Performs a GC operation.
///
/// # Safety
/// - `state` must be valid.
/// - `what` and `data` must be valid for Luau’s GC API.
pub unsafe fn lua_gc(state: *mut lua_State, what: c_int, data: c_int) -> c_int {
    unsafe { (luau_api().lua_gc)(state, what, data) }
}

/// Returns the name of a GC state.
///
/// # Safety
/// - `state` must be a valid GC state enum value.
/// - Returned pointer is owned by Luau.
pub unsafe fn lua_gcstatename(state: c_int) -> *const c_char {
    unsafe { (luau_api().lua_gcstatename)(state) }
}

/// Returns the current GC allocation rate.
///
/// # Safety
/// - `state` must be valid.
pub unsafe fn lua_gcallocationrate(state: *mut lua_State) -> i64 {
    unsafe { (luau_api().lua_gcallocationrate)(state) }
}

//
// Memory stats
//

/// Sets the current memory category.
///
/// # Safety
/// - `state` must be valid.
/// - `category` must be a valid category id.
pub unsafe fn lua_setmemcat(state: *mut lua_State, category: c_int) {
    unsafe { (luau_api().lua_setmemcat)(state, category) }
}

/// Returns total bytes allocated for a category.
///
/// # Safety
/// - `state` must be valid.
/// - `category` must be valid.
pub unsafe fn lua_totalbytes(state: *mut lua_State, category: c_int) -> usize {
    unsafe { (luau_api().lua_totalbytes)(state, category) }
}

//
// Misc
//

/// Raises a Luau error using the value on top of the stack.
///
/// # Safety
/// - `state` must be valid.
/// - Top of stack must contain an error object or message.
pub unsafe fn lua_error(state: *mut lua_State) -> ! {
    unsafe { (luau_api().lua_error)(state) }
}

/// Iterates over a table at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` and key on stack must be valid.
pub unsafe fn lua_next(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_next)(state, idx) }
}

/// Raw iterator over table at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` and `iter` must be valid.
pub unsafe fn lua_rawiter(state: *mut lua_State, idx: c_int, iter: c_int) -> c_int {
    unsafe { (luau_api().lua_rawiter)(state, idx, iter) }
}

/// Concatenates `n` values on the stack.
///
/// # Safety
/// - `state` must be valid.
/// - Top `n` values must be valid for concatenation.
pub unsafe fn lua_concat(state: *mut lua_State, n: c_int) {
    unsafe { (luau_api().lua_concat)(state, n) }
}

/// Returns Luau’s internal clock value.
///
/// # Safety
/// - Always safe to call.
pub unsafe fn lua_clock() -> c_double {
    unsafe { (luau_api().lua_clock)() }
}

/// Sets the tag of userdata at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_setuserdatatag(state: *mut lua_State, idx: c_int, tag: c_int) {
    unsafe { (luau_api().lua_setuserdatatag)(state, idx, tag) }
}

/// Registers a destructor for a userdata tag.
///
/// # Safety
/// - `state` must be valid.
/// - `dtor` must be a valid function pointer or None.
pub unsafe fn lua_setuserdatadtor(
    state: *mut lua_State,
    tag: c_int,
    dtor: Option<lua_Destructor>,
) {
    unsafe { (luau_api().lua_setuserdatadtor)(state, tag, dtor) }
}

/// Gets the destructor for a userdata tag.
///
/// # Safety
/// - `state` must be valid.
pub unsafe fn lua_getuserdatadtor(state: *mut lua_State, tag: c_int) -> Option<lua_Destructor> {
    unsafe { (luau_api().lua_getuserdatadtor)(state, tag) }
}

/// Sets the metatable for a userdata tag.
///
/// # Safety
/// - `state` must be valid.
/// - Metatable must be on the stack.
pub unsafe fn lua_setuserdatametatable(state: *mut lua_State, tag: c_int) {
    unsafe { (luau_api().lua_setuserdatametatable)(state, tag) }
}

/// Gets the metatable for a userdata tag.
///
/// # Safety
/// - `state` must be valid.
pub unsafe fn lua_getuserdatametatable(state: *mut lua_State, tag: c_int) {
    unsafe { (luau_api().lua_getuserdatametatable)(state, tag) }
}

/// Sets a human‑readable name for a light userdata tag.
///
/// # Safety
/// - `state` must be valid.
/// - `name` must be a valid C string.
pub unsafe fn lua_setlightuserdataname(
    state: *mut lua_State,
    tag: c_int,
    name: *const c_char,
) {
    unsafe { (luau_api().lua_setlightuserdataname)(state, tag, name) }
}

/// Gets the name for a light userdata tag.
///
/// # Safety
/// - `state` must be valid.
/// - Returned pointer is owned by Luau.
pub unsafe fn lua_getlightuserdataname(state: *mut lua_State, tag: c_int) -> *const c_char {
    unsafe { (luau_api().lua_getlightuserdataname)(state, tag) }
}

/// Clones a function at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must refer to a function.
pub unsafe fn lua_clonefunction(state: *mut lua_State, idx: c_int) {
    unsafe { (luau_api().lua_clonefunction)(state, idx) }
}

/// Clears all entries from a table at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must refer to a table.
pub unsafe fn lua_cleartable(state: *mut lua_State, idx: c_int) {
    unsafe { (luau_api().lua_cleartable)(state, idx) }
}

/// Returns the allocator function and its userdata.
///
/// # Safety
/// - `state` must be valid.
/// - `ud` must be a valid pointer.
pub unsafe fn lua_getallocf(state: *mut lua_State, ud: *mut *mut c_void) -> lua_Alloc {
    unsafe { (luau_api().lua_getallocf)(state, ud) }
}

//
// Reference system
//

/// Creates a reference to the value at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_ref(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_ref)(state, idx) }
}

/// Releases a reference.
///
/// # Safety
/// - `state` must be valid.
/// - `ref` must be a reference previously returned by `lua_ref`.
pub unsafe fn lua_unref(state: *mut lua_State, r#ref: c_int) {
    unsafe { (luau_api().lua_unref)(state, r#ref) }
}

//
// Debug API
//

/// Returns the current stack depth.
///
/// # Safety
/// - `state` must be valid.
pub unsafe fn lua_stackdepth(state: *mut lua_State) -> c_int {
    unsafe { (luau_api().lua_stackdepth)(state) }
}

/// Fills `ar` with debug info for a given level.
///
/// # Safety
/// - `state` must be valid.
/// - `what` and `ar` must be valid pointers.
pub unsafe fn lua_getinfo(
    state: *mut lua_State,
    level: c_int,
    what: *const c_char,
    ar: *mut lua_Debug,
) -> c_int {
    unsafe { (luau_api().lua_getinfo)(state, level, what, ar) }
}

/// Pushes an argument from a given stack level.
///
/// # Safety
/// - `state` must be valid.
/// - `level` and `n` must be valid.
pub unsafe fn lua_getargument(state: *mut lua_State, level: c_int, n: c_int) -> c_int {
    unsafe { (luau_api().lua_getargument)(state, level, n) }
}

/// Gets a local variable name and pushes its value.
///
/// # Safety
/// - `state` must be valid.
/// - `level` and `n` must be valid.
/// - Returned pointer is owned by Luau.
pub unsafe fn lua_getlocal(state: *mut lua_State, level: c_int, n: c_int) -> *const c_char {
    unsafe { (luau_api().lua_getlocal)(state, level, n) }
}

/// Sets a local variable from the top of the stack.
///
/// # Safety
/// - `state` must be valid.
/// - `level` and `n` must be valid.
pub unsafe fn lua_setlocal(state: *mut lua_State, level: c_int, n: c_int) -> *const c_char {
    unsafe { (luau_api().lua_setlocal)(state, level, n) }
}

/// Gets an upvalue name and pushes its value.
///
/// # Safety
/// - `state` must be valid.
/// - `funcindex` and `n` must be valid.
pub unsafe fn lua_getupvalue(
    state: *mut lua_State,
    funcindex: c_int,
    n: c_int,
) -> *const c_char {
    unsafe { (luau_api().lua_getupvalue)(state, funcindex, n) }
}

/// Sets an upvalue from the top of the stack.
///
/// # Safety
/// - `state` must be valid.
/// - `funcindex` and `n` must be valid.
pub unsafe fn lua_setupvalue(
    state: *mut lua_State,
    funcindex: c_int,
    n: c_int,
) -> *const c_char {
    unsafe { (luau_api().lua_setupvalue)(state, funcindex, n) }
}

/// Enables or disables single‑step debugging.
///
/// # Safety
/// - `state` must be valid.
pub unsafe fn lua_singlestep(state: *mut lua_State, enabled: c_int) {
    unsafe { (luau_api().lua_singlestep)(state, enabled) }
}

/// Sets or clears a breakpoint.
///
/// # Safety
/// - `state` must be valid.
/// - `funcindex` and `line` must be valid.
pub unsafe fn lua_breakpoint(
    state: *mut lua_State,
    funcindex: c_int,
    line: c_int,
    enabled: c_int,
) -> c_int {
    unsafe { (luau_api().lua_breakpoint)(state, funcindex, line, enabled) }
}

/// Collects coverage information for a function.
///
/// # Safety
/// - `state` must be valid.
/// - `context` and `callback` must be valid.
pub unsafe fn lua_getcoverage(
    state: *mut lua_State,
    funcindex: c_int,
    context: *mut c_void,
    callback: lua_Coverage,
) {
    unsafe { (luau_api().lua_getcoverage)(state, funcindex, context, callback) }
}

/// Returns a debug trace string.
///
/// # Safety
/// - `state` must be valid.
/// - Returned pointer is owned by Luau.
pub unsafe fn lua_debugtrace(state: *mut lua_State) -> *const c_char {
    unsafe { (luau_api().lua_debugtrace)(state) }
}

//
// Callbacks
//

/// Returns a pointer to the callbacks structure for `state`.
///
/// # Safety
/// - `state` must be valid.
/// - Returned pointer is owned by Luau.
pub unsafe fn lua_callbacks(state: *mut lua_State) -> *mut lua_Callbacks {
    unsafe { (luau_api().lua_callbacks)(state) }
}

//
// Customization lib
//

/// Sets a Luau feature flag.
///
/// # Safety
/// - `name` must be a valid C string.
pub unsafe fn luau_setfflag(name: *const c_char, value: c_int) -> c_int {
    unsafe { (luau_api().luau_setfflag)(name, value) }
}

/// Returns a pointer to the metatable of value at `idx`.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_getmetatablepointer(state: *mut lua_State, idx: c_int) -> *const c_void {
    unsafe { (luau_api().lua_getmetatablepointer)(state, idx) }
}

/// Dumps GC info to `file`.
///
/// # Safety
/// - `state` must be valid.
/// - `file` and `category_name` must be valid for your usage.
pub unsafe fn lua_gcdump(
    state: *mut lua_State,
    file: *mut c_void,
    category_name: Option<unsafe extern "C" fn(state: *mut lua_State, memcat: u8) -> *const c_char>,
) {
    unsafe { (luau_api().lua_gcdump)(state, file, category_name) }
}

//
// Inline helpers / macros
//

/// Converts value at `idx` to a number.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_tonumber(state: *mut lua_State, idx: c_int) -> lua_Number {
    unsafe { (luau_api().lua_tonumber)(state, idx) }
}

/// Converts value at `idx` to an integer.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_tointeger_(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { (luau_api().lua_tointeger_)(state, idx) }
}

/// Converts value at `idx` to an unsigned integer.
///
/// # Safety
/// - `state` must be valid.
/// - `idx` must be valid.
pub unsafe fn lua_tounsigned(state: *mut lua_State, i: c_int) -> lua_Unsigned {
    unsafe { (luau_api().lua_tounsigned)(state, i) }
}

/// Pops `n` values from the stack.
///
/// # Safety
/// - `state` must be valid.
/// - `n` must not underflow the stack.
pub unsafe fn lua_pop(state: *mut lua_State, n: c_int) {
    unsafe { (luau_api().lua_pop)(state, n) }
}

/// Creates a new empty table.
///
/// # Safety
/// - `state` must be valid.
/// - Stack must have space for one value.
pub unsafe fn lua_newtable(state: *mut lua_State) {
    unsafe { (luau_api().lua_newtable)(state) }
}

/// Allocates userdata of size `sz`.
///
/// # Safety
/// - `state` must be valid.
/// - `sz` must be reasonable.
pub unsafe fn lua_newuserdata(state: *mut lua_State, sz: usize) -> *mut c_void {
    unsafe { (luau_api().lua_newuserdata)(state, sz) }
}

/// Allocates userdata for a value of type `c_void` (opaque).
///
/// # Safety
/// - `state` must be valid.
pub unsafe fn lua_newuserdata_t(state: *mut lua_State, data: c_void) -> *mut c_void {
    unsafe { (luau_api().lua_newuserdata_t)(state, data) }
}

/// Returns non‑zero if value at `n` is a function.
///
/// # Safety
/// - `state` must be valid.
/// - `n` must be valid.
pub unsafe fn lua_isfunction(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { (luau_api().lua_isfunction)(state, n) }
}

/// Returns non‑zero if value at `n` is a table.
///
/// # Safety
/// - `state` must be valid.
/// - `n` must be valid.
pub unsafe fn lua_istable(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { (luau_api().lua_istable)(state, n) }
}

/// Returns non‑zero if value at `n` is light userdata.
///
/// # Safety
/// - `state` must be valid.
/// - `n` must be valid.
pub unsafe fn lua_islightuserdata(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { (luau_api().lua_islightuserdata)(state, n) }
}

/// Returns non‑zero if value at `n` is nil.
///
/// # Safety
/// - `state` must be valid.
/// - `n` must be valid.
pub unsafe fn lua_isnil(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { (luau_api().lua_isnil)(state, n) }
}

/// Returns non‑zero if value at `n` is boolean.
///
/// # Safety
/// - `state` must be valid.
/// - `n` must be valid.
pub unsafe fn lua_isboolean(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { (luau_api().lua_isboolean)(state, n) }
}

/// Returns non‑zero if value at `n` is a vector.
///
/// # Safety
/// - `state` must be valid.
/// - `n` must be valid.
pub unsafe fn lua_isvector(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { (luau_api().lua_isvector)(state, n) }
}

/// Returns non‑zero if value at `n` is a thread.
///
/// # Safety
/// - `state` must be valid.
/// - `n` must be valid.
pub unsafe fn lua_isthread(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { (luau_api().lua_isthread)(state, n) }
}

/// Returns non‑zero if value at `n` is a buffer.
///
/// # Safety
/// - `state` must be valid.
/// - `n` must be valid.
pub unsafe fn lua_isbuffer(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { (luau_api().lua_isbuffer)(state, n) }
}

/// Returns non‑zero if value at `n` is none (no value).
///
/// # Safety
/// - `state` must be valid.
/// - `n` must be valid.
pub unsafe fn lua_isnone(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { (luau_api().lua_isnone)(state, n) }
}

/// Returns non‑zero if value at `n` is none or nil.
///
/// # Safety
/// - `state` must be valid.
/// - `n` must be valid.
pub unsafe fn lua_isnoneornil(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { (luau_api().lua_isnoneornil)(state, n) }
}

/// Pushes a literal C string.
///
/// # Safety
/// - `state` must be valid.
/// - `s` must be a valid C string.
pub unsafe fn lua_pushliteral(state: *mut lua_State, s: *const c_char) {
    unsafe { (luau_api().lua_pushliteral)(state, s) }
}

/// Pushes a C function.
///
/// # Safety
/// - `state` must be valid.
/// - `f` must be a valid function pointer.
pub unsafe fn lua_pushcfunction(state: *mut lua_State, f: lua_CFunction) {
    unsafe { (luau_api().lua_pushcfunction)(state, f) }
}

/// Pushes a C function with debug name.
///
/// # Safety
/// - `state` must be valid.
/// - `f` and `debugname` must be valid.
pub unsafe fn lua_pushcfunctiond(
    state: *mut lua_State,
    f: lua_CFunction,
    debugname: *const c_char,
) {
    unsafe { (luau_api().lua_pushcfunctiond)(state, f, debugname) }
}

/// Pushes a C closure with `nup` upvalues.
///
/// # Safety
/// - `state` must be valid.
/// - Stack must contain `nup` upvalues.
pub unsafe fn lua_pushcclosure(state: *mut lua_State, f: lua_CFunction, nup: c_int) {
    unsafe { (luau_api().lua_pushcclosure)(state, f, nup) }
}

/// Pushes a C closure with continuation.
///
/// # Safety
/// - `state` must be valid.
/// - `f` and `cont` must be valid.
/// - Stack must contain `nup` upvalues.
pub unsafe fn lua_pushcclosurec(
    state: *mut lua_State,
    f: lua_CFunction,
    cont: lua_Continuation,
    nup: c_int,
) {
    unsafe { (luau_api().lua_pushcclosurec)(state, f, cont, nup) }
}

/// Pushes a C closure with debug name.
///
/// # Safety
/// - `state` must be valid.
/// - `f` and `debugname` must be valid.
/// - Stack must contain `nup` upvalues.
pub unsafe fn lua_pushcclosured(
    state: *mut lua_State,
    f: lua_CFunction,
    debugname: *const c_char,
    nup: c_int,
) {
    unsafe { (luau_api().lua_pushcclosured)(state, f, debugname, nup) }
}

/// Pushes a light userdata pointer.
///
/// # Safety
/// - `state` must be valid.
/// - `p` must be valid for your usage.
pub unsafe fn lua_pushlightuserdata(state: *mut lua_State, p: *mut c_void) {
    unsafe { (luau_api().lua_pushlightuserdata)(state, p) }
}

/// Sets a global variable from the top of the stack.
///
/// # Safety
/// - `state` must be valid.
/// - `var` must be a valid C string.
pub unsafe fn lua_setglobal(state: *mut lua_State, var: *const c_char) {
    unsafe { (luau_api().lua_setglobal)(state, var) }
}

/// Pushes a global variable onto the stack.
///
/// # Safety
/// - `state` must be valid.
/// - `var` must be a valid C string.
pub unsafe fn lua_getglobal(state: *mut lua_State, var: *const c_char) -> c_int {
    unsafe { (luau_api().lua_getglobal)(state, var) }
}

/// Converts value at `i` to a string.
///
/// # Safety
/// - `state` must be valid.
/// - `i` must be valid.
/// - Returned pointer is owned by Luau.
pub unsafe fn lua_tostring(state: *mut lua_State, i: c_int) -> *const c_char {
    unsafe { (luau_api().lua_tostring)(state, i) }
}

//
// lauxlib additions
//

/// Registers a library of functions.
///
/// # Safety
/// - `L` must be valid.
/// - `libname` and `l` must be valid pointers.
pub unsafe fn luaL_register(L: *mut lua_State, libname: *const c_char, l: *const luaL_Reg) {
    unsafe { (luau_api().luaL_register)(L, libname, l) }
}

/// Gets a metafield from an object.
///
/// # Safety
/// - `L` must be valid.
/// - `obj` and `e` must be valid.
pub unsafe fn luaL_getmetafield_(L: *mut lua_State, obj: c_int, e: *const c_char) -> c_int {
    unsafe { (luau_api().luaL_getmetafield_)(L, obj, e) }
}

/// Calls a metamethod on an object.
///
/// # Safety
/// - `L` must be valid.
/// - `obj` and `e` must be valid.
pub unsafe fn luaL_callmeta(L: *mut lua_State, obj: c_int, e: *const c_char) -> c_int {
    unsafe { (luau_api().luaL_callmeta)(L, obj, e) }
}

/// Raises a type error for argument `narg`.
///
/// # Safety
/// - `L` must be valid.
/// - `tname` must be a valid C string.
pub unsafe fn luaL_typeerror(L: *mut lua_State, narg: c_int, tname: *const c_char) -> ! {
    unsafe { (luau_api().luaL_typeerror)(L, narg, tname) }
}

/// Raises an argument error for `narg`.
///
/// # Safety
/// - `L` must be valid.
/// - `extramsg` must be a valid C string.
pub unsafe fn luaL_argerror(L: *mut lua_State, narg: c_int, extramsg: *const c_char) -> ! {
    unsafe { (luau_api().luaL_argerror)(L, narg, extramsg) }
}

/// Checks and returns a string argument.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` and `l` must be valid.
pub unsafe fn luaL_checklstring(L: *mut lua_State, narg: c_int, l: *mut usize) -> *const c_char {
    unsafe { (luau_api().luaL_checklstring)(L, narg, l) }
}

/// Returns an optional string argument.
///
/// # Safety
/// - `L` must be valid.
/// - `narg`, `def`, and `l` must be valid.
pub unsafe fn luaL_optlstring(
    L: *mut lua_State,
    narg: c_int,
    def: *const c_char,
    l: *mut usize,
) -> *const c_char {
    unsafe { (luau_api().luaL_optlstring)(L, narg, def, l) }
}

/// Checks and returns a number argument.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` must be valid.
pub unsafe fn luaL_checknumber(L: *mut lua_State, narg: c_int) -> lua_Number {
    unsafe { (luau_api().luaL_checknumber)(L, narg) }
}

/// Returns an optional number argument.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` must be valid.
pub unsafe fn luaL_optnumber(L: *mut lua_State, narg: c_int, def: lua_Number) -> lua_Number {
    unsafe { (luau_api().luaL_optnumber)(L, narg, def) }
}

/// Checks and returns a boolean argument.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` must be valid.
pub unsafe fn luaL_checkboolean(L: *mut lua_State, narg: c_int) -> c_int {
    unsafe { (luau_api().luaL_checkboolean)(L, narg) }
}

/// Returns an optional boolean argument.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` must be valid.
pub unsafe fn luaL_optboolean(L: *mut lua_State, narg: c_int, def: c_int) -> c_int {
    unsafe { (luau_api().luaL_optboolean)(L, narg, def) }
}

/// Checks and returns an integer argument.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` must be valid.
pub unsafe fn luaL_checkinteger_(L: *mut lua_State, narg: c_int) -> c_int {
    unsafe { (luau_api().luaL_checkinteger_)(L, narg) }
}

/// Returns an optional integer argument.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` must be valid.
pub unsafe fn luaL_optinteger_(L: *mut lua_State, narg: c_int, def: c_int) -> c_int {
    unsafe { (luau_api().luaL_optinteger_)(L, narg, def) }
}

/// Checks and returns an unsigned argument.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` must be valid.
pub unsafe fn luaL_checkunsigned(L: *mut lua_State, narg: c_int) -> lua_Unsigned {
    unsafe { (luau_api().luaL_checkunsigned)(L, narg) }
}

/// Returns an optional unsigned argument.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` must be valid.
pub unsafe fn luaL_optunsigned(L: *mut lua_State, narg: c_int, def: lua_Unsigned) -> lua_Unsigned {
    unsafe { (luau_api().luaL_optunsigned)(L, narg, def) }
}

/// Checks and returns a vector argument.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` must be valid.
pub unsafe fn luaL_checkvector(L: *mut lua_State, narg: c_int) -> *const c_float {
    unsafe { (luau_api().luaL_checkvector)(L, narg) }
}

/// Returns an optional vector argument.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` must be valid.
pub unsafe fn luaL_optvector(
    L: *mut lua_State,
    narg: c_int,
    def: *const c_float,
) -> *const c_float {
    unsafe { (luau_api().luaL_optvector)(L, narg, def) }
}

/// Ensures the stack has at least `sz` free slots.
///
/// # Safety
/// - `L` must be valid.
/// - `msg` must be a valid C string.
pub unsafe fn luaL_checkstack(L: *mut lua_State, sz: c_int, msg: *const c_char) {
    unsafe { (luau_api().luaL_checkstack_)(L, sz, msg) }
}

/// Checks that argument `narg` has type `t`.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` and `t` must be valid.
pub unsafe fn luaL_checktype(L: *mut lua_State, narg: c_int, t: c_int) {
    unsafe { (luau_api().luaL_checktype)(L, narg, t) }
}

/// Ensures argument `narg` is not none.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` must be valid.
pub unsafe fn luaL_checkany(L: *mut lua_State, narg: c_int) {
    unsafe { (luau_api().luaL_checkany)(L, narg) }
}

/// Creates a new metatable.
///
/// # Safety
/// - `L` must be valid.
/// - `tname` must be a valid C string.
pub unsafe fn luaL_newmetatable_(L: *mut lua_State, tname: *const c_char) -> c_int {
    unsafe { (luau_api().luaL_newmetatable_)(L, tname) }
}

/// Checks and returns userdata of a given type.
///
/// # Safety
/// - `L` must be valid.
/// - `ud` and `tname` must be valid.
pub unsafe fn luaL_checkudata(L: *mut lua_State, ud: c_int, tname: *const c_char) -> *mut c_void {
    unsafe { (luau_api().luaL_checkudata)(L, ud, tname) }
}

/// Checks and returns a buffer argument.
///
/// # Safety
/// - `L` must be valid.
/// - `narg` and `len` must be valid.
pub unsafe fn luaL_checkbuffer(L: *mut lua_State, narg: c_int, len: *mut usize) -> *mut c_void {
    unsafe { (luau_api().luaL_checkbuffer)(L, narg, len) }
}

/// Pushes source location for error messages.
///
/// # Safety
/// - `L` must be valid.
/// - `lvl` must be valid.
pub unsafe fn luaL_where(L: *mut lua_State, lvl: c_int) {
    unsafe { (luau_api().luaL_where)(L, lvl) }
}

/// Raises a formatted error.
///
/// # Safety
/// - `L` must be valid.
/// - `fmt` must be a valid C string.
pub unsafe fn luaL_error(L: *mut lua_State, fmt: *const c_char) -> ! {
    unsafe { (luau_api().luaL_error)(L, fmt) }
}

/// Checks an option string against a list.
///
/// # Safety
/// - `L` must be valid.
/// - `def` and `lst` must be valid.
pub unsafe fn luaL_checkoption(
    L: *mut lua_State,
    narg: c_int,
    def: *const c_char,
    lst: *const *const c_char,
) -> c_int {
    unsafe { (luau_api().luaL_checkoption)(L, narg, def, lst) }
}

/// Converts value at `idx` to string and returns pointer + length.
///
/// # Safety
/// - `L` must be valid.
/// - `idx` and `len` must be valid.
pub unsafe fn luaL_tolstring_(L: *mut lua_State, idx: c_int, len: *mut usize) -> *const c_char {
    unsafe { (luau_api().luaL_tolstring_)(L, idx, len) }
}

/// Creates a new Luau state with default allocator.
///
/// # Safety
/// - Same as `lua_newstate`.
pub unsafe fn luaL_newstate() -> *mut lua_State {
    unsafe { (luau_api().luaL_newstate)() }
}

/// Finds or creates a table.
///
/// # Safety
/// - `L` must be valid.
/// - `fname` must be a valid C string.
pub unsafe fn luaL_findtable(
    L: *mut lua_State,
    idx: c_int,
    fname: *const c_char,
    szhint: c_int,
) -> *const c_char {
    unsafe { (luau_api().luaL_findtable)(L, idx, fname, szhint) }
}

/// Returns the userdata type name of value at `idx` or "no value" if unset.
///
/// Note: for userdata with a metatable that defines `__type`, this reports
/// that `__type` string instead of the raw Luau type name. This is convenient
/// but can be a footgun if you rely on it for low‑level type checks.
///
/// # Safety
/// - `L` must be valid.
/// - `idx` must be valid.
/// - Returned pointer is owned by Luau.
pub unsafe fn luaL_typename(L: *mut lua_State, idx: c_int) -> *const c_char {
    unsafe { (luau_api().luaL_typename)(L, idx) }
}

/// Calls a function in yieldable mode.
///
/// # Safety
/// - `L` must be valid.
/// - Stack layout must match expectations.
pub unsafe fn luaL_callyieldable(L: *mut lua_State, nargs: c_int, nresults: c_int) -> c_int {
    unsafe { (luau_api().luaL_callyieldable)(L, nargs, nresults) }
}

/// Applies sandboxing to the state.
///
/// # Safety
/// - `L` must be valid.
pub unsafe fn luaL_sandbox_(L: *mut lua_State) {
    unsafe { (luau_api().luaL_sandbox_)(L) }
}

/// Applies sandboxing to the current thread.
///
/// # Safety
/// - `L` must be valid.
pub unsafe fn luaL_sandboxthread(L: *mut lua_State) {
    unsafe { (luau_api().luaL_sandboxthread)(L) }
}

//
// Buffer API
//

/// Initializes a string buffer.
///
/// # Safety
/// - `L` and `B` must be valid.
pub unsafe fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Strbuf) {
    unsafe { (luau_api().luaL_buffinit)(L, B) }
}

/// Initializes a buffer with a given size.
///
/// # Safety
/// - `L` and `B` must be valid.
pub unsafe fn luaL_buffinitsize(L: *mut lua_State, B: *mut luaL_Strbuf, size: usize) -> *mut c_char {
    unsafe { (luau_api().luaL_buffinitsize)(L, B, size) }
}

/// Ensures buffer has at least `size` free bytes.
///
/// # Safety
/// - `B` must be valid.
pub unsafe fn luaL_prepbuffsize(B: *mut luaL_Strbuf, size: usize) -> *mut c_char {
    unsafe { (luau_api().luaL_prepbuffsize)(B, size) }
}

/// Adds `l` bytes from `s` to buffer.
///
/// # Safety
/// - `B` must be valid.
/// - `s` must point to `l` readable bytes.
pub unsafe fn luaL_addlstring(B: *mut luaL_Strbuf, s: *const c_char, l: usize) {
    unsafe { (luau_api().luaL_addlstring)(B, s, l) }
}

/// Adds top of stack value to buffer.
///
/// # Safety
/// - `B` must be valid.
/// - Associated Luau state must be valid.
pub unsafe fn luaL_addvalue(B: *mut luaL_Strbuf) {
    unsafe { (luau_api().luaL_addvalue)(B) }
}

/// Adds value at `idx` to buffer.
///
/// # Safety
/// - `B` must be valid.
/// - `idx` must be valid in associated state.
pub unsafe fn luaL_addvalueany(B: *mut luaL_Strbuf, idx: c_int) {
    unsafe { (luau_api().luaL_addvalueany)(B, idx) }
}

/// Pushes buffer contents as a string.
///
/// # Safety
/// - `B` must be valid.
pub unsafe fn luaL_pushresult(B: *mut luaL_Strbuf) {
    unsafe { (luau_api().luaL_pushresult)(B) }
}

/// Pushes buffer contents as a string with explicit size.
///
/// # Safety
/// - `B` must be valid.
pub unsafe fn luaL_pushresultsize(B: *mut luaL_Strbuf, size: usize) {
    unsafe { (luau_api().luaL_pushresultsize)(B, size) }
}
