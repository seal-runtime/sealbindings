use std::ffi::c_int;

use bstr::BString;

use crate::ffi;
use super::strings::BStringFromPtr;

#[allow(unused, reason = "only needed for debugging")]
/// Returns the Luau type name of the value at `idx` as a `BString`.
///
/// # Safety
/// - `state` must be non-null
/// - `idx` must be a valid stack index
pub unsafe fn type_of(state: *mut ffi::lua_State, idx: c_int) -> BString {
    let ptr = unsafe { ffi::luaL_typename(state, idx) };
    unsafe { BString::clone_from_ptr(ptr) }
}
