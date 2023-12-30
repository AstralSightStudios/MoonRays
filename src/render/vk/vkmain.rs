use std::{ptr::null, ffi::CString};
use ash::{self, vk, Entry};
#[path="../../base/sysinfo.rs"]
mod SysInfo;

pub fn Init(){
    let vk_application_name_cstr = CString::new("Unknown Game").unwrap();
    let vk_engine_name_cstr = CString::new("MoonRays Engine").unwrap();
    let VK_APPLICATION_NAME = vk_application_name_cstr.as_ptr();
    let VK_ENGINE_NAME = vk_engine_name_cstr.as_ptr();
    

    let VK_APPLICATION_INFO_DEFAULT: vk::ApplicationInfo = vk::ApplicationInfo{
        s_type: vk::StructureType::APPLICATION_INFO,
        p_next: null(),
        p_application_name: VK_APPLICATION_NAME,
        application_version: 1,
        p_engine_name: VK_ENGINE_NAME,
        engine_version: 1,
        api_version: vk::API_VERSION_1_0,
    };

    let VK_INSTANCE_CREATE_INFO_DEFAULT: vk::InstanceCreateInfo = vk::InstanceCreateInfo{
        s_type: vk::StructureType::INSTANCE_CREATE_INFO,
        p_next: null(),
        flags:  ash::vk::InstanceCreateFlags::default(),
        p_application_info: &VK_APPLICATION_INFO_DEFAULT,
        ..Default::default()
    };

    let mut entry = unsafe { Entry::load().unwrap() };

    #[cfg(any(target_os = "macos", target_os = "ios"))]{
        entry = ash_molten::load();
    }
    
    let VkInstance = unsafe { Entry::create_instance(&entry ,&VK_INSTANCE_CREATE_INFO_DEFAULT, None).unwrap() };
}