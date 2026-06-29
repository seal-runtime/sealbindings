use std::ffi::c_int;
use crate::ffi;

/// Checks if the value at the top of the stack is a seal `@std/err` WrappedError.
/// Peeks at the metatable's `__type` field; leaves the stack unchanged.
fn wrapped_error_at_top(state: *mut ffi::lua_State) -> bool {
    unsafe {
        if ffi::lua_type(state, -1) != ffi::LUA_TUSERDATA() {
            return false;
        }
        if ffi::lua_getmetatable(state, -1) == 0 {
            return false;
        }
        ffi::lua_getfield(state, -1, c"__type".as_ptr());
        let mut len: usize = 0;
        let ptr = ffi::lua_tolstring(state, -1, &mut len);
        let is_error = !ptr.is_null()
            && std::slice::from_raw_parts(ptr as *const u8, len) == b"error";
        ffi::lua_pop(state, 2);
        is_error
    }
}

/// RAII guard that asserts the Luau stack has the expected net change when dropped.
///
/// Useful for catching stack imbalances during development — accidentally pushing or
/// leaving values on the stack is one of the most common bugs when working with the
/// Lua C API. Only checks in debug builds (`debug_assert`), so it's zero-cost in release.
///
/// # Example
/// ```ignore
/// unsafe extern "C-unwind" fn my_func(state: *mut lua_State) -> c_int {
///     let _g = LuauStackGuard::returning(state, 1);
///     ffi::lua_pushinteger(state, 42);
///     1
/// }
/// ```
pub struct LuauStackGuard<'a> {
    caller: std::panic::Location<'a>,
    state: *mut ffi::lua_State,
    expected_top: c_int,
    /// If `Some`, also accept this top when the value there is a seal WrappedError.
    error_top: Option<c_int>,
}

impl<'a> LuauStackGuard<'a> {
    /// Asserts on drop that the stack top changed by exactly `expected_delta`.
    /// Use a positive delta for pushed values, 0 for balanced, negative for net pops.
    ///
    /// # Panics
    /// Panics if `sealbindings::initialize()` has not been called yet.
    #[track_caller]
    pub fn new(state: *mut ffi::lua_State, expected_delta: c_int) -> Self {
        let caller = std::panic::Location::caller();
        let message = format!("sealbindings::initialize() must be called before using LuauStackGuard (called from {})", caller);
        crate::LUAU_API.get().expect(&message);
        let top = unsafe { ffi::lua_gettop(state) };
        Self {
            caller: *caller,
            state,
            expected_top: top + expected_delta,
            error_top: None,
        }
    }

    /// Asserts on drop that the stack is the same height as when the guard was created.
    ///
    /// # Panics
    /// Panics if `sealbindings::initialize()` has not been called yet.
    #[track_caller]
    pub fn balanced(state: *mut ffi::lua_State) -> Self {
        Self::new(state, 0)
    }

    /// Asserts on drop that exactly `n` values were pushed net onto the stack.
    ///
    /// # Panics
    /// Panics if `sealbindings::initialize()` has not been called yet.
    #[track_caller]
    pub fn returning(state: *mut ffi::lua_State, n: c_int) -> Self {
        Self::new(state, n)
    }

    /// Asserts on drop that either 0 values were pushed, or exactly 1 value was pushed
    /// and it is a seal `@std/err` WrappedError.
    ///
    /// Use this for functions that return nothing on success and push an error on failure.
    ///
    /// # Panics
    /// Panics if `sealbindings::initialize()` has not been called yet.
    #[track_caller]
    pub fn none_or_error(state: *mut ffi::lua_State) -> Self {
        let caller = std::panic::Location::caller();
        let message = format!("sealbindings::initialize() must be called before using LuauStackGuard (called from {})", caller);
        crate::LUAU_API.get().expect(&message);
        let top = unsafe { ffi::lua_gettop(state) };
        Self {
            caller: *caller,
            state,
            expected_top: top,
            error_top: Some(top + 1),
        }
    }

    /// Asserts on drop that either `n` values were pushed, or exactly 1 value was pushed
    /// and it is a seal `@std/err` WrappedError.
    ///
    /// Use this for functions that return `n` values on success and push an error on failure.
    ///
    /// # Panics
    /// Panics if `sealbindings::initialize()` has not been called yet.
    #[track_caller]
    pub fn returning_or_error(state: *mut ffi::lua_State, n: c_int) -> Self {
        let caller = std::panic::Location::caller();
        let message = format!("sealbindings::initialize() must be called before using LuauStackGuard (called from {})", caller);
        crate::LUAU_API.get().expect(&message);
        let top = unsafe { ffi::lua_gettop(state) };
        Self {
            caller: *caller,
            state,
            expected_top: top + n,
            error_top: Some(top + 1),
        }
    }
}

impl<'a> Drop for LuauStackGuard<'a> {
    fn drop(&mut self) {
        let actual = unsafe { ffi::lua_gettop(self.state) };
        let ok = actual == self.expected_top
            || self.error_top.is_some_and(|et| actual == et && wrapped_error_at_top(self.state));
        debug_assert!(
            ok,
            "\nLuauStackGuard violation in {}\nLuau stack imbalance: expected top {} (or error at {}), got {} (delta off by {})",
            self.caller,
            self.expected_top,
            self.error_top.unwrap_or(self.expected_top),
            actual,
            actual - self.expected_top,
        );
    }
}
