pub mod ffi;
use ffi::api::LuauApi;

use std::sync::OnceLock;

static LUAU_API: OnceLock<&'static LuauApi> = OnceLock::new();

/// Calls the function `seal_get_ffi_api` from the seal executable and initializes
/// the static LUAU_API.
/// 
/// After calling this function, you may call any function from `sealbindings::ffi`.
///
/// # Safety
/// - Must be called exactly once, from `seal_open_extern`.
/// - The seal executable must properly expose a function `seal_get_ffi_api`;
///   older versions of *seal* may not have this function.
pub unsafe fn initialize() {
    #[cfg(unix)]
    let lib = {
        use libloading::os::unix::Library as UnixLib;
        UnixLib::this()
    };

    #[cfg(windows)]
    let lib = {
        use libloading::os::windows::Library as WinLib;
        WinLib::this().expect("GetModuleHandle(NULL) failed")
    };

    let func = unsafe {
        lib.get::<unsafe extern "C-unwind" fn() -> *const LuauApi>(c"seal_get_ffi_api")
            .expect("seal_get_ffi_api not found in seal executable; make sure you're on the latest seal!")
    };

    let ptr = unsafe {func() };
    assert!(!ptr.is_null(), "seal_get_ffi_api returned null");
    unsafe {
        LUAU_API.set(&*ptr).ok();
    }

}