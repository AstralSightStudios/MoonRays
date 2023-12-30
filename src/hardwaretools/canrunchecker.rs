/*
    canrunchecker.rs
    主要作用是判断用户的电脑能否运行引擎以及制作出来的游戏，主要判断vk版本、硬件配置等
*/

use std::ffi::CString;

use ash::{Instance, Entry};

pub fn IsCurrentVulkanSupported(entry: &Entry, VkInstance: &Instance) -> bool{
    // 通过检查是否存在vkEnumerateInstanceVersion函数来判断是否版本在1.1或更高
    let targetcstring = CString::new("vkEnumerateInstanceVersion").unwrap();
    let addr = unsafe { Entry::get_instance_proc_addr(entry, VkInstance.to_owned().handle(), targetcstring.as_ptr()) };
    if addr == None {
        return false;
    }
    return true
}