/*
    vkmain.rs
    作为Vulkan图形API的主入口，做一些基础操作例如创建Instance等等
*/

use std::{ptr::null, ffi::CString, process::exit};
use ash::{self, vk::{self, PhysicalDevice, QueueFlags, SurfaceKHR}, Entry, Instance, Device, extensions::khr::Surface};
use raw_window_handle::HasRawDisplayHandle;
use winit::{event_loop::EventLoop, window::Window};
#[path="../../base/sysinfo.rs"]
pub(crate) mod SysInfo;
#[path="../../hardwaretools/canrunchecker.rs"]
mod CanRunChecker;
#[path="../../tools.rs"]
mod Tools;
#[path="./vksurface.rs"]
mod VkSurfaceTools;
#[path="./vkwindow.rs"]
mod VkWindowTools;

pub fn LoadVK() -> ((Window, EventLoop<()>), (Entry, Instance), PhysicalDevice, Device, (SurfaceKHR, Surface)){
    let VkWindow = VkWindowTools::CreateWinitWindow();
    let InstanceExts = ash_window::enumerate_required_extensions(VkWindow.1.raw_display_handle()).unwrap();
    let VkReturn = CreateInstance(InstanceExts);
    let VkPhysicalDevice = GetPhysicalDevice(&VkReturn.1);
    let VkDevice = GetVkDevice(&VkReturn.1, VkPhysicalDevice);
    let VkSurface = VkSurfaceTools::GetSurface(&VkReturn.1, &VkReturn.0, &VkWindow.0);

    return (VkWindow, VkReturn, VkPhysicalDevice, VkDevice, VkSurface);
}

pub fn CreateInstance(VK_INSTANCE_CREATE_INFO_ENABLE_EXTENSION: &[*const i8]) -> (Entry, Instance){
    let vk_application_name_cstr = CString::new(crate::GAME_NAME).unwrap();
    let vk_engine_name_cstr = CString::new("MoonRays Engine").unwrap();
    let VK_APPLICATION_NAME = vk_application_name_cstr.as_ptr();
    let VK_ENGINE_NAME = vk_engine_name_cstr.as_ptr();

    let VK_APPLICATION_INFO_DEFAULT: vk::ApplicationInfo = vk::ApplicationInfo{
        s_type: vk::StructureType::APPLICATION_INFO,
        p_next: null(),
        p_application_name: VK_APPLICATION_NAME,
        application_version: crate::GAME_VERSION,
        p_engine_name: VK_ENGINE_NAME,
        engine_version: crate::ENGINE_VERSION,
        api_version: vk::API_VERSION_1_3,
    };

    let VK_INSTANCE_CREATE_INFO_DEFAULT: vk::InstanceCreateInfo = vk::InstanceCreateInfo{
        s_type: vk::StructureType::INSTANCE_CREATE_INFO,
        flags:  ash::vk::InstanceCreateFlags::default(),
        p_application_info: &VK_APPLICATION_INFO_DEFAULT,
        pp_enabled_extension_names: VK_INSTANCE_CREATE_INFO_ENABLE_EXTENSION.as_ptr(),
        enabled_extension_count: VK_INSTANCE_CREATE_INFO_ENABLE_EXTENSION.len() as u32,
        ..Default::default()
    };

    let mut entry = unsafe { Entry::load().unwrap() };

    // 苹果的所有操作系统不原生支持Vulkan，因此通过官方提供的MoltenVK创建特有Entry以实时将Vulkan转译为苹果官方的Metal
    // 该操作及其所用的依赖（ash_molten）已被标记为只对MacOS和iOS启用与编译
    #[cfg(any(target_os = "macos", target_os = "ios"))]{
        entry = ash_molten::load();
    }
    
    let Match_VkInstance = unsafe { Entry::create_instance(&entry ,&VK_INSTANCE_CREATE_INFO_DEFAULT, None) };

    log::info!("Vulkan Instance Created");

    match Match_VkInstance {
        Ok(VkInstance) => return (entry, VkInstance),
        Err(err) => {
            // vulkan抛出INCOMPATIBLE_DRIVER通常意味着驱动程序不兼容程序使用的Vulkan版本
            // 因此返回更详细的错误信息给用户
            if (err.to_string().contains("INCOMPATIBLE_DRIVER")){
                log::error!("Unable to create a Vulkan instance because the driver does not support it.");
                msgbox::create("MoonRaysEngine ERROR", "Unable to create a Vulkan instance because the driver does not support it. You may need to update your graphics card drivers and if that still doesn't work, it may mean that your graphics card does not support Vulkan and cannot run this application.", msgbox::IconType::Error).unwrap();
            }
            // TODO：这里未来还能添加更多错误处理
            else{
                log::error!("An unknown error occurred while creating a Vulkan instance. Details:{}", err);
                msgbox::create("MoonRaysEngine ERROR", ("An unknown error occurred while creating a Vulkan instance. \nDetails:".to_string() + &err.to_string()).as_str(), msgbox::IconType::Error).unwrap();
            }
            exit(900000001);
        },
    }
}

pub fn GetPhysicalDevice(VkInstance: &Instance) -> PhysicalDevice{
    let device_list = unsafe { VkInstance.enumerate_physical_devices().unwrap() };
    let mut available_devices: Vec<PhysicalDevice> = vec![];
    let mut available_devices_prop: Vec<vk::PhysicalDeviceProperties> = vec![];
    let mut return_device: PhysicalDevice = device_list[0];
    let mut return_device_name: &String = &"".to_string();
    for device in device_list {
        let device_prop = unsafe { VkInstance.get_physical_device_properties(device) };
        if (device_prop.api_version >= vk::make_api_version(0, 1, 3, 0)){
            available_devices.push(device);
            available_devices_prop.push(device_prop);
        }
    }

    log::info!("Available devices:");
    let mut name_check_index = 0;
    for available_device_prop in available_devices_prop {
        let name = Tools::veci8_to_string(available_device_prop.device_name.to_vec());

        log::info!("   - {}", name);

        // 显卡选择优先度：无法识别的用作显示输出的显卡 < AMD独显/Vega核显 < Intel Arc独显 < NVIDIA任意显卡
        if(name.contains("AMD") || name.contains("Radeon") || name.contains("RX") || name.contains("ATI") || name.contains("Vega")){
            return_device = unsafe { VkInstance.enumerate_physical_devices().unwrap() }[name_check_index];
        }
        if(name.contains("Arc") || name.contains("Intel")){
            return_device = unsafe { VkInstance.enumerate_physical_devices().unwrap() }[name_check_index];
        }
        if(name.contains("NVIDIA")){
            return_device = unsafe { VkInstance.enumerate_physical_devices().unwrap() }[name_check_index];
        }

        name_check_index += 1;
    }

    let return_device_name_binding = Tools::veci8_to_string(unsafe { VkInstance.get_physical_device_properties(return_device).device_name.to_vec() });
    return_device_name = &return_device_name_binding;

    log::info!("The graphics card has been selected: {}", return_device_name);

    return return_device;
}

pub fn GetVkDevice(VkInstance: &Instance, VkPhysicalDevice: PhysicalDevice) -> ash::Device{
    let PhysicalDevicesQueueFamilyPropertiesVec = unsafe { VkInstance.get_physical_device_queue_family_properties(VkPhysicalDevice) };
    let mut QueueFamilyPropIndex = 0;

    for QueueFamilyProp in PhysicalDevicesQueueFamilyPropertiesVec{
        if(QueueFamilyProp.queue_flags.contains(QueueFlags::GRAPHICS)){
            log::info!("The family of queues located at index {} supports graphical features, so it has been selected", QueueFamilyPropIndex);
            break;
        }
        QueueFamilyPropIndex += 1;
    }
    
    let VK_DEVICE_QUEUE_CREATE_INFO_DEVICE = vk::DeviceQueueCreateInfo{
        s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
        queue_family_index: QueueFamilyPropIndex,
        ..Default::default()
    };

    let VK_DEVICE_CREATE_INFO = vk::DeviceCreateInfo{
        s_type: vk::StructureType::DEVICE_CREATE_INFO,
        queue_create_info_count: 1,
        p_queue_create_infos: &VK_DEVICE_QUEUE_CREATE_INFO_DEVICE,
        ..Default::default()
    };

    let VkDevice = unsafe { VkInstance.create_device(VkPhysicalDevice, &VK_DEVICE_CREATE_INFO, None).unwrap() };

    return VkDevice;
}