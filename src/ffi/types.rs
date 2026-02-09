#![allow(nonstandard_style)]

use std::marker::{PhantomData, PhantomPinned};
use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};

//
// Core opaque state
//

#[repr(C)]
pub struct lua_State {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

//
// Basic numeric types
//

pub type lua_Number = c_double;

#[cfg(target_pointer_width = "32")]
pub type lua_Integer = i32;
#[cfg(target_pointer_width = "64")]
pub type lua_Integer = i64;

pub type lua_Unsigned = c_uint;

//
// Function pointer types
//

pub type lua_CFunction = unsafe extern "C-unwind" fn(L: *mut lua_State) -> c_int;
pub type lua_Continuation =
    unsafe extern "C-unwind" fn(L: *mut lua_State, status: c_int) -> c_int;

/// Userdata destructor (no unwinding)
pub type lua_Destructor = unsafe extern "C" fn(L: *mut lua_State, ud: *mut c_void);

/// Allocator (no unwinding)
pub type lua_Alloc = unsafe extern "C" fn(
    ud: *mut c_void,
    ptr: *mut c_void,
    osize: usize,
    nsize: usize,
) -> *mut c_void;

//
// Debug / hook types
//

pub type lua_Hook = unsafe extern "C-unwind" fn(L: *mut lua_State, ar: *mut lua_Debug);

pub type lua_Coverage = unsafe extern "C-unwind" fn(
    context: *mut c_void,
    function: *const c_char,
    linedefined: c_int,
    depth: c_int,
    hits: *const c_int,
    size: usize,
);

//
// lua_Debug
//

const LUA_IDSIZE: usize = 256;

#[repr(C)]
pub struct lua_Debug {
    pub name: *const c_char,
    pub what: *const c_char,
    pub source: *const c_char,
    pub short_src: *const c_char,
    pub linedefined: c_int,
    pub currentline: c_int,
    pub nupvals: u8,
    pub nparams: u8,
    pub isvararg: c_char,
    pub userdata: *mut c_void,
    pub ssbuf: [c_char; LUA_IDSIZE],
}

//
// lua_Callbacks
//

#[repr(C)]
#[non_exhaustive]
pub struct lua_Callbacks {
    /// arbitrary userdata pointer that is never overwritten by Luau
    pub userdata: *mut c_void,

    /// gets called at safepoints (loop back edges, call/ret, gc) if set
    pub interrupt: Option<unsafe extern "C-unwind" fn(L: *mut lua_State, gc: c_int)>,
    /// gets called when an unprotected error is raised (if longjmp is used)
    pub panic: Option<unsafe extern "C-unwind" fn(L: *mut lua_State, errcode: c_int)>,

    /// gets called when L is created (LP == parent) or destroyed (LP == NULL)
    pub userthread: Option<unsafe extern "C-unwind" fn(LP: *mut lua_State, L: *mut lua_State)>,
    /// gets called when a string is created; returned atom can be retrieved via tostringatom
    pub useratom: Option<unsafe extern "C-unwind" fn(s: *const c_char, l: usize) -> i16>,

    /// gets called when BREAK instruction is encountered
    pub debugbreak: Option<unsafe extern "C-unwind" fn(L: *mut lua_State, ar: *mut lua_Debug)>,
    /// gets called after each instruction in single step mode
    pub debugstep: Option<unsafe extern "C-unwind" fn(L: *mut lua_State, ar: *mut lua_Debug)>,
    /// gets called when thread execution is interrupted by break in another thread
    pub debuginterrupt: Option<unsafe extern "C-unwind" fn(L: *mut lua_State, ar: *mut lua_Debug)>,
    /// gets called when protected call results in an error
    pub debugprotectederror: Option<unsafe extern "C-unwind" fn(L: *mut lua_State)>,

    /// gets called when memory is allocated
    pub onallocate: Option<unsafe extern "C-unwind" fn(L: *mut lua_State, osize: usize, nsize: usize)>,
}

//
// luau_try callback types
//

#[repr(C)]
pub struct RustCallbackRet {
    pub status: c_int,
    pub ret: *mut c_void,
}

pub type RustCallback =
    unsafe extern "C-unwind" fn(L: *mut lua_State, data: *mut c_void) -> *mut c_void;

#[repr(C)]
pub struct luaL_Strbuf {
    _data: [u8; 0],
    _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct luaL_Reg {
    pub name: *const c_char,
    pub func: lua_CFunction,
}
