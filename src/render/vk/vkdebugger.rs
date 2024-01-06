use std::{os::raw::c_void, ffi::CStr};

use ash::{Instance, Entry, vk::{self, DebugUtilsMessengerEXT}};

unsafe extern "system" fn VKDebuggerCallback(VkDbgServerityFlags: vk::DebugUtilsMessageSeverityFlagsEXT, VkDbgTypeFlags: vk::DebugUtilsMessageTypeFlagsEXT, VkDbgCallbackData: *const vk::DebugUtilsMessengerCallbackDataEXT, VkDbgUserData: *mut c_void) -> vk::Bool32{
    let binding = CStr::from_ptr(VkDbgCallbackData.read().p_message_id_name);
    let MessageIdName = binding.to_string_lossy();
    let binding = CStr::from_ptr(VkDbgCallbackData.read().p_message);
    let Message = binding.to_string_lossy();

    // 判断错误信息中是否包含错误类型关键字以用对应的方式输出log
    // 但事实上就算是error也无所谓 只要验证层没panic并且报错不影响效果都可以忽略
    if(Message.contains("Error") || Message.contains("error")){
        log::error!("Vulkan Callback: From {} > {}", MessageIdName, Message);
    }
    else if (Message.contains("Warning") || Message.contains("warning")) {
        log::warn!("Vulkan Callback: From {} > {}", MessageIdName, Message);
    }
    else{
        log::info!("Vulkan Callback: From {} > {}", MessageIdName, Message);
    }

    return vk::TRUE;
}

pub fn GetVKDebugger(VkInstance: &Instance, VkEnrty: &Entry) -> DebugUtilsMessengerEXT{
    let VkDebuggerFN = ash::extensions::ext::DebugUtils::new(VkEnrty, VkInstance);

    let DEBUG_MESSENGER_CREATE_INFO_DEFAULT = vk::DebugUtilsMessengerCreateInfoEXT{
        s_type: ash::vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        message_severity: (
            ash::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE | 
            ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO | 
            ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING |
            ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
        ),
        message_type: (
            ash::vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING |
            ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL |
            ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE |
            ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
        ),
        pfn_user_callback: Some(VKDebuggerCallback),
        ..Default::default()
    };

    let VkDebuggerMessenger = unsafe { VkDebuggerFN.create_debug_utils_messenger(&DEBUG_MESSENGER_CREATE_INFO_DEFAULT, None).unwrap() };

    log::info!("Vulkan DebuggerMessenger was created");

    return VkDebuggerMessenger;
}