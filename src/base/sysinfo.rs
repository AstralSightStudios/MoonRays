/*
    sysinfo.rs
    获取系统在软件层面上的一些信息
*/

use std::{any::type_name, ffi::CString, os::raw::c_void};

use windows::Win32::Foundation::HMODULE;

pub fn get_system_name() -> String {
    return std::env::consts::OS.to_string();
}

pub fn WIN32_ONLY_get_current_module_handle() -> *const c_void {
    let hmodule = unsafe { windows::Win32::System::LibraryLoader::GetModuleHandleA(None).unwrap() };
    return &hmodule as *const HMODULE as *const c_void;
}