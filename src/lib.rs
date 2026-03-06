pub mod ffi;
pub use ffi::api::LuauApi;

use std::ffi::CString;
use std::sync::OnceLock;

static LUAU_API: OnceLock<&'static LuauApi> = OnceLock::new();

/// takes in a pointer `*const LuauApi` that should be passed to
/// `seal_open_extern` and initialize the LUAU_API constant within this library.
/// 
/// After calling this function, `sealbindings::ffi*` functions should be safe to call.
/// 
/// # Safety
/// - You should call this function immediately in `seal_open_extern`
/// - Pass a valid pointer to an actual *const LuauApi
/// - Ensure the versioning of the API provided by the version of seal you're using
///   is supported by this library.
pub unsafe fn initialize(ptr: *const LuauApi) {
    assert!(!ptr.is_null(), "LuauApi pointer is null");
    unsafe {
        LUAU_API.set(&*ptr).ok();
    }
}

/// Pushes a wrapped error object from @std/err onto the Luau stack.
/// After this returns, the stack top is the wrapped error.
///
/// # Panics
/// Panics if `msg` contains interior NUL bytes
/// Throws a runtime error if the Luau stack cannot grow.
pub fn push_wrapped_error(state: *mut ffi::lua_State, msg: &str) {
    assert!(!state.is_null(), "Luau state is null, this shouldn't be possible");
    // just use seal's @std/err library to construct the error
    unsafe {
        ffi::luaL_checkstack(state, 4, c"need 4 or more slots on luau stack".as_ptr());

        // - push require to stack
        ffi::lua_getglobal(state, c"require".as_ptr());
        // stack: [ require ]
    
        // push "@std/err"
        ffi::lua_pushstring(state, c"@std/err".as_ptr());
        // stack: [ require, "@std/err" ]
    
        // Step 3: call require("@std/err")
        ffi::lua_call(state, 1, 1);
        // stack: [ err_table ]
    
        // Step 4: get err.wrap
        ffi::lua_getfield(state, -1, c"wrap".as_ptr());
        // stack: [ err_table, err.wrap ]
    
        let error_message = CString::new(msg).expect("error message contains internal NUL bytes");
        ffi::lua_pushstring(state, error_message.as_ptr());
        // stack: [ err_table, err.wrap, msg ]
    
        // Step 6: call wrap(msg)
        ffi::lua_call(state, 1, 1);
        // stack: [ err_table, wrapped_error ]
    
        // Step 7: remove err_table, leave wrapped_error
        ffi::lua_remove(state, -2);
        // stack: [ wrapped_error ]
    }
}

/// Pushes a C function wrapped by the seal global `ecall` to the Luau stack.
/// This allows wrapped errors returned by the C function to be thrown nominally like seal errors.
/// 
/// After this returns, the stack top is the wrapped function returned by ecall.
/// Caller should `return 1` or continue stack manipulation.
///
/// # Safety
/// - state must be a non-null pointer to a lua_State
/// - passed func should be a valid Luau CFunction
/// - Luau stack should have at least 3 empty slots
pub unsafe fn push_wrapped_c_function(
    state: *mut ffi::lua_State,
    func: ffi::lua_CFunction,
) {
    unsafe {
        // Step 1: push global ecall
        ffi::lua_getglobal(state, c"ecall".as_ptr());
        // stack: [ ecall ]
    
        // Step 2: push the C function to wrap
        ffi::lua_pushcfunction(state, func);
        // stack: [ ecall, func ]
    
        // Step 3: call ecall(func)
        // Pops ecall + func, pushes return value
        ffi::lua_call(state, 1, 1);
        // stack: [ wrapped_function ]
    }
}