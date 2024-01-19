/*
    sysinfo.rs
    获取系统在软件层面上的一些信息
*/

use std::os::raw::c_void;

pub fn get_system_name() -> String {
    return std::env::consts::OS.to_string();
}

pub fn WIN32_ONLY_get_current_module_handle() -> *const c_void {
    #[cfg(any(target_os = "windows"))]{
        use windows::Win32::Foundation::HMODULE;
        let hmodule = unsafe { windows::Win32::System::LibraryLoader::GetModuleHandleA(None).unwrap() };
        return &hmodule as *const HMODULE as *const c_void;
    }

    return std::ptr::null();
}