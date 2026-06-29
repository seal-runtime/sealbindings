use std::ffi::{CStr, c_char, c_int};

use bstr::BString;

use crate::ffi;

pub trait BStringFromPtr {
    /// Clones a NUL-terminated C string pointer into an owned `BString`.
    /// The bytes are copied so Luau retains ownership of the original allocation.
    ///
    /// # Safety
    /// - `ptr` must be a valid NUL-terminated C string
    unsafe fn clone_from_ptr(ptr: *const c_char) -> BString;

    /// Clones `len` bytes from a C string pointer into an owned `BString`.
    /// Use this over `clone_from_ptr` when Luau gives you an explicit length
    /// (e.g. from `lua_tolstring`), as the string may contain interior NUL bytes.
    ///
    /// # Safety
    /// - `ptr` must be valid for reads of `len` bytes
    unsafe fn clone_from_ptr_with_len(ptr: *const c_char, len: usize) -> BString;
}

impl BStringFromPtr for BString {
    unsafe fn clone_from_ptr(ptr: *const c_char) -> BString {
        // go through CStr first to respect the NUL terminator
        let cstr = unsafe { CStr::from_ptr(ptr) };
        // clone so we don't free bytes owned by Luau
        BString::from(cstr.to_bytes().to_owned())
    }

    unsafe fn clone_from_ptr_with_len(ptr: *const c_char, len: usize) -> BString {
        // SAFETY: caller ensures ptr is valid for len bytes (both come from Luau)
        let bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) };
        // clone so we don't free bytes owned by Luau
        BString::from(bytes.to_owned())
    }
}

pub trait BStringFromLuau {
    /// Clones a Luau string at stack `idx` into an owned `BString` using `lua_tostring`.
    /// Prefer `clone_lstring_from_stack` if the string may contain interior NUL bytes.
    ///
    /// # Safety
    /// - `state` must be non-null
    /// - Value at `idx` must be a Luau string
    /// - String must not contain interior NUL bytes
    unsafe fn clone_cstring_from_stack(state: *mut ffi::lua_State, idx: c_int) -> BString;

    /// Clones a Luau string at stack `idx` into an owned `BString` using `lua_tolstring`.
    /// Handles strings with interior NUL bytes correctly.
    ///
    /// # Safety
    /// - `state` must be non-null
    /// - Value at `idx` must be a Luau string
    unsafe fn clone_lstring_from_stack(state: *mut ffi::lua_State, idx: c_int) -> BString;
}

impl BStringFromLuau for BString {
    unsafe fn clone_cstring_from_stack(state: *mut ffi::lua_State, idx: c_int) -> BString {
        let ptr = unsafe { ffi::lua_tostring(state, idx) };
        unsafe { BString::clone_from_ptr(ptr) }
    }

    unsafe fn clone_lstring_from_stack(state: *mut ffi::lua_State, idx: c_int) -> BString {
        let mut len: usize = 0;
        let ptr = unsafe { ffi::lua_tolstring(state, idx, &mut len) };
        unsafe { BString::clone_from_ptr_with_len(ptr, len) }
    }
}
