pub mod ffi;
pub mod utils;
pub use ffi::api::LuauApi;
pub use ffi::types::LuauState;
pub use utils::*;

use std::ffi::{c_int, CString};
use std::sync::OnceLock;

static LUAU_API: OnceLock<&'static LuauApi> = OnceLock::new();

#[allow(non_snake_case)]
/// Version-sensitive Luau constants copied from the LuauApi during `initialize()`.
pub(crate) struct LuauConstants {
    // Stack pseudo-indices
    pub LUA_REGISTRYINDEX: c_int,
    pub LUA_ENVIRONINDEX:  c_int,
    pub LUA_GLOBALSINDEX:  c_int,
    pub LUAI_MAXCSTACK:    c_int,
    // Thread status (lua_Status)
    pub LUA_OK:       c_int,
    pub LUA_YIELD:    c_int,
    pub LUA_ERRRUN:   c_int,
    pub LUA_ERRSYNTAX: c_int,
    pub LUA_ERRMEM:   c_int,
    pub LUA_ERRERR:   c_int,
    // GC operations (lua_GCOp)
    pub LUA_GCSTOP:        c_int,
    pub LUA_GCRESTART:     c_int,
    pub LUA_GCCOLLECT:     c_int,
    pub LUA_GCCOUNT:       c_int,
    pub LUA_GCCOUNTB:      c_int,
    pub LUA_GCISRUNNING:   c_int,
    pub LUA_GCSTEP:        c_int,
    pub LUA_GCSETGOAL:     c_int,
    pub LUA_GCSETSTEPMUL:  c_int,
    pub LUA_GCSETSTEPSIZE: c_int,
    // Type tags (LUA_T*)
    pub LUA_TNONE:          c_int,
    pub LUA_TNIL:           c_int,
    pub LUA_TBOOLEAN:       c_int,
    pub LUA_TLIGHTUSERDATA: c_int,
    pub LUA_TNUMBER:        c_int,
    pub LUA_TINTEGER:       c_int,
    pub LUA_TVECTOR:        c_int,
    pub LUA_TSTRING:        c_int,
    pub LUA_TTABLE:         c_int,
    pub LUA_TFUNCTION:      c_int,
    pub LUA_TUSERDATA:      c_int,
    pub LUA_TTHREAD:        c_int,
    pub LUA_TBUFFER:        c_int,
}

pub(crate) static LUAU_CONSTANTS: OnceLock<LuauConstants> = OnceLock::new();

/// Initializes sealbindings from the `*const LuauApi` provided by seal, then calls `f`
/// to set up the plugin's Luau exports. Returns the result of `f`, or a pushed
/// `@std/err` wrapped error if `f` panics.
///
/// Pass the entire body of `seal_open_extern` as the closure so that any panic during
/// initialization is caught and surfaced as a seal error instead of crashing the runtime.
/// Any values pushed to the stack before the panic are cleaned up automatically.
///
/// # Example
/// ```ignore
/// #[unsafe(no_mangle)]
/// pub unsafe extern "C-unwind" fn seal_open_extern(
///     state: *mut LuauState,
///     api: *const LuauApi,
/// ) -> c_int {
///     unsafe { sealbindings::initialize(state, api, |state| {
///         state.create_table(0, 1);
///         state.set_wrapped_function(c"hello", hello, c"hello() -> ()");
///         1
///     })}
/// }
/// ```
///
/// # Safety
/// - `state` must be a valid non-null Luau state
/// - `api` must be a valid non-null pointer to the `LuauApi` passed by seal
pub unsafe fn initialize<F>(state: *mut ffi::lua_State, api: *const LuauApi, f: F) -> c_int
where
    F: FnOnce(*mut ffi::lua_State) -> c_int,
{
    assert!(!api.is_null(), "LuauApi pointer is null");
    unsafe {
        let api_ref = &*api;
        LUAU_API.set(api_ref).ok();
        LUAU_CONSTANTS.set(LuauConstants {
            LUA_REGISTRYINDEX: api_ref.LUA_REGISTRYINDEX,
            LUA_ENVIRONINDEX:  api_ref.LUA_ENVIRONINDEX,
            LUA_GLOBALSINDEX:  api_ref.LUA_GLOBALSINDEX,
            LUAI_MAXCSTACK:    api_ref.LUAI_MAXCSTACK,
            LUA_OK:            api_ref.LUA_OK,
            LUA_YIELD:         api_ref.LUA_YIELD,
            LUA_ERRRUN:        api_ref.LUA_ERRRUN,
            LUA_ERRSYNTAX:     api_ref.LUA_ERRSYNTAX,
            LUA_ERRMEM:        api_ref.LUA_ERRMEM,
            LUA_ERRERR:        api_ref.LUA_ERRERR,
            LUA_GCSTOP:        api_ref.LUA_GCSTOP,
            LUA_GCRESTART:     api_ref.LUA_GCRESTART,
            LUA_GCCOLLECT:     api_ref.LUA_GCCOLLECT,
            LUA_GCCOUNT:       api_ref.LUA_GCCOUNT,
            LUA_GCCOUNTB:      api_ref.LUA_GCCOUNTB,
            LUA_GCISRUNNING:   api_ref.LUA_GCISRUNNING,
            LUA_GCSTEP:        api_ref.LUA_GCSTEP,
            LUA_GCSETGOAL:     api_ref.LUA_GCSETGOAL,
            LUA_GCSETSTEPMUL:  api_ref.LUA_GCSETSTEPMUL,
            LUA_GCSETSTEPSIZE: api_ref.LUA_GCSETSTEPSIZE,
            LUA_TNONE:         api_ref.LUA_TNONE,
            LUA_TNIL:          api_ref.LUA_TNIL,
            LUA_TBOOLEAN:      api_ref.LUA_TBOOLEAN,
            LUA_TLIGHTUSERDATA: api_ref.LUA_TLIGHTUSERDATA,
            LUA_TNUMBER:       api_ref.LUA_TNUMBER,
            LUA_TINTEGER:      api_ref.LUA_TINTEGER,
            LUA_TVECTOR:       api_ref.LUA_TVECTOR,
            LUA_TSTRING:       api_ref.LUA_TSTRING,
            LUA_TTABLE:        api_ref.LUA_TTABLE,
            LUA_TFUNCTION:     api_ref.LUA_TFUNCTION,
            LUA_TUSERDATA:     api_ref.LUA_TUSERDATA,
            LUA_TTHREAD:       api_ref.LUA_TTHREAD,
            LUA_TBUFFER:       api_ref.LUA_TBUFFER,
        }).ok();
    }

    std::panic::set_hook(Box::new(|info| {
        let location = info.location()
            .expect("rust should give us a panic location");

        eprintln!(
            "seal extern plugin (dynamic library) panicked at runtime:\n  message: {}\n  panic at: {}:{}",
            info.payload_as_str().unwrap_or("<unknown message>"), location.file(), location.line()
        );
    }));

    let top_before = unsafe { ffi::lua_gettop(state) };
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(state))) {
        Ok(n) => n,
        Err(payload) => {
            unsafe { ffi::lua_settop(state, top_before) };
            let msg = payload
                .downcast_ref::<String>().map(|s| s.as_str())
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("unknown panic payload");
            push_wrapped_error(state, &format!("plugin panicked: {}", msg))
        }
    }
}

/// Pushes a wrapped error object from @std/err onto the Luau stack.
/// After this returns, the stack top is the wrapped error.
///
/// # Panics
/// Panics if `msg` contains interior NUL bytes
/// Throws a runtime error if the Luau stack cannot grow.
pub fn push_wrapped_error(state: *mut ffi::lua_State, msg: &str) -> c_int {
    assert!(!state.is_null(), "Luau state is null, this shouldn't be possible");
    unsafe {
        ffi::luaL_checkstack(state, 2, c"need 2 or more slots on luau stack".as_ptr());

        // get err.wrap pinned in the registry by seal's runtime init
        ffi::lua_getfield(state, ffi::LUA_REGISTRYINDEX(), c"@std/err:wrap".as_ptr());
        // stack: [ err.wrap ]

        let error_message = CString::new(msg).expect("error message contains internal NUL bytes");
        ffi::lua_pushstring(state, error_message.as_ptr());
        // stack: [ err.wrap, msg ]

        ffi::lua_call(state, 1, 1);
        // stack: [ wrapped_error ]
    }
    1
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
pub unsafe fn push_wrapped_c_function(state: *mut ffi::lua_State, func: ffi::lua_CFunction, debugname: Option<&std::ffi::CStr>) {
    unsafe {
        // Step 1: push global ecall
        ffi::lua_getglobal(state, c"ecall".as_ptr());
        // stack: [ ecall ]

        // Step 2: push the C function to wrap, with optional debug name
        match debugname {
            Some(name) => ffi::lua_pushcfunctiond(state, func, name.as_ptr()),
            None       => ffi::lua_pushcfunction(state, func),
        }
        // stack: [ ecall, func ]

        // Step 3: call ecall(func)
        // Pops ecall + func, pushes return value
        ffi::lua_call(state, 1, 1);
        // stack: [ wrapped_function ]
    }
}

