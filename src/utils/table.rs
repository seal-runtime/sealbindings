use std::ffi::c_int;

use crate::ffi;

/// Iterates over every key-value pair in the table at `table_idx`, calling `visit`
/// for each entry with the key at index `-2` and value at `-1` on the stack.
///
/// Uses `lua_rawiter` (Luau-specific) which bypasses `__next` metamethods and needs
/// no sentinel nil push, making it faster than the `lua_next` pattern.
///
/// The visitor must not pop the key or value — they are cleaned up automatically
/// after each call. Adding values to the stack inside the visitor is fine as long
/// as you pop them before returning.
///
/// # Example
/// ```ignore
/// // called from Luau as: my_func({ hello = "world", foo = "bar" })
/// unsafe extern "C-unwind" fn my_func(state: *mut lua_State) -> c_int {
///     for_each_in_table(state, 1, |state| {
///         // key at -2, value at -1 — both are strings in this example
///         let key   = BString::clone_lstring_from_stack(state, -2);
///         let value = BString::clone_lstring_from_stack(state, -1);
///         println!("{key} = {value}");
///     });
///     0
/// }
/// ```
///
/// # Safety
/// - `state` must be non-null
/// - Value at `table_idx` must be a table
pub unsafe fn for_each_in_table<F>(
    state: *mut ffi::lua_State,
    table_idx: c_int,
    mut visit: F,
) where
    F: FnMut(*mut ffi::lua_State),
{
    let mut iter: c_int = 0;
    loop {
        iter = unsafe { ffi::lua_rawiter(state, table_idx, iter) };
        if iter == -1 {
            break;
        }
        // key at -2, value at -1
        visit(state);
        unsafe { ffi::lua_pop(state, 2) };
    }
}
