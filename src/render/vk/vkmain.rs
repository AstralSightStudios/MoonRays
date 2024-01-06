/*
    vkmain.rs
    作为Vulkan图形API的主入口，做一些基础操作例如创建Instance等等
*/

use std::{ptr::null, ffi::{CString, CStr}, process::exit};
use ash::{self, vk::{self, PhysicalDevice, QueueFlags, SurfaceKHR, SurfaceFormatKHR, PresentModeKHR, Extent2D, SurfaceCapabilitiesKHR, DeviceQueueCreateInfo, SwapchainKHR}, Entry, Instance, Device, extensions::{khr::{Surface, Swapchain}, self}};
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
#[path="./vkdebugger.rs"]
mod VkDebugger;

pub fn LoadVK() -> ((Window, EventLoop<()>), (Entry, Instance), PhysicalDevice, (ash::Device, (u32, u32)), (SurfaceKHR, Surface)){
    let VkWindow = VkWindowTools::CreateWinitWindow();
    let mut InstanceExts = ash_window::enumerate_required_extensions(VkWindow.1.raw_display_handle()).unwrap().to_vec();
    InstanceExts.push(
        extensions::ext::DebugUtils::name().as_ptr()
    );
    let mut InstanceLayers: Vec<*const i8> = vec![
        #[cfg(debug_assertions)]{
            // 对于Debug编译，启用Vulkan验证层
            "VK_LAYER_KHRONOS_validation".as_ptr() as *const i8
        }
    ];
    let VkReturn = CreateInstance(InstanceExts,InstanceLayers);
    let VkDebuggerReturn = VkDebugger::GetVKDebugger(&VkReturn.1, &VkReturn.0);
    let VkPhysicalDevice = GetPhysicalDevice(&VkReturn.1);
    let mut DeviceExts: Vec<*const i8> = vec![];
    DeviceExts.push(
        extensions::khr::Swapchain::name().as_ptr()
    );
    let VkSurface = VkSurfaceTools::GetSurface(&VkReturn.1, &VkReturn.0, &VkWindow.0);
    let VkDevice = GetVkDevice(&VkReturn.1, VkPhysicalDevice, &VkSurface.0, &VkSurface.1, DeviceExts);
    let VkSwapChainSettings = GetSwapChainSettings(&VkPhysicalDevice, &VkSurface.0, &VkSurface.1, &VkWindow.0);
    let VkSwapChain = GetSwapChain(&VkReturn.1, &VkDevice.0, &VkSurface.0, &VkSwapChainSettings, &VkDevice.1);

    return (VkWindow, VkReturn, VkPhysicalDevice, VkDevice, VkSurface.clone());
}

pub fn CreateInstance(VK_INSTANCE_CREATE_INFO_ENABLE_EXTENSION: Vec<*const i8>, VK_INSTANCE_CREATE_INFO_ENABLE_LAYERS: Vec<*const i8>) -> (Entry, Instance){
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
        enabled_layer_count: VK_INSTANCE_CREATE_INFO_ENABLE_LAYERS.len() as u32,
        pp_enabled_layer_names: VK_INSTANCE_CREATE_INFO_ENABLE_LAYERS.as_ptr(),
        ..Default::default()
    };

    let mut entry = unsafe { Entry::load().unwrap() };

    // 苹果的所有操作系统不原生支持Vulkan，因此通过官方提供的MoltenVK创建特有Entry以实时将Vulkan转译为苹果官方的Metal
    // 该操作及其所用的依赖（ash_molten）已被标记为只对MacOS和iOS启用与编译
    #[cfg(any(target_os = "macos", target_os = "ios"))]{
        entry = ash_molten::load();
    }

    log::info!("Available layers:");

    for LayerProp in entry.enumerate_instance_layer_properties().unwrap(){
        let LayerName = unsafe { CStr::from_ptr(LayerProp.layer_name.as_ptr()).to_str().unwrap() };
        log::info!("   - {}", LayerName);
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
                log::error!("An unknown error occurred while creating a Vulkan instance. Details: {}", err);
                msgbox::create("MoonRaysEngine ERROR", ("An unknown error occurred while creating a Vulkan instance. \nDetails: ".to_string() + &err.to_string()).as_str(), msgbox::IconType::Error).unwrap();
            }
            exit(900000001);
        },
    }
}

pub fn GetPhysicalDevice(VkInstance: &Instance) -> PhysicalDevice{
    let device_list = unsafe { VkInstance.enumerate_physical_devices().unwrap() };
    let mut available_devices: Vec<PhysicalDevice> = vec![];
    let mut available_devices_prop: Vec<vk::PhysicalDeviceProperties> = vec![];
    // TODO: 通过配置文件定义此选项 让用户自己决定用哪张卡跑游戏
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

pub fn GetVkDevice(VkInstance: &Instance, VkPhysicalDevice: PhysicalDevice,VkSurface: &SurfaceKHR, SurfaceFN: &Surface, VK_DEVICE_CREATE_INFO_ENABLE_EXTENSION: Vec<*const i8>) -> (ash::Device,(u32, u32)){
    let PhysicalDevicesQueueFamilyPropertiesVec = unsafe { VkInstance.get_physical_device_queue_family_properties(VkPhysicalDevice) };
    let mut QueueFamilyPropIndexGraphics = 0;
    let mut QueueFamilyPropIndexPresent = 0;
    let mut QueueFamilyPropCountGraphics = 0;
    let mut QueueFamilyPropCountPresent = 0;

    let mut ScannedGraphicsQueue = false;
    let mut ScannedPresentQueue = false;

    for QueueFamilyProp in PhysicalDevicesQueueFamilyPropertiesVec{
        if(QueueFamilyProp.queue_flags.contains(QueueFlags::GRAPHICS)){
            if(!ScannedGraphicsQueue){
                log::info!("The family of queues located at index {} supports graphical features, so it has been selected. QueueCount={}", QueueFamilyPropIndexGraphics, &QueueFamilyProp.queue_count);
                QueueFamilyPropCountGraphics = QueueFamilyProp.queue_count;
                ScannedGraphicsQueue = true;
            }
        }
        if(!ScannedGraphicsQueue){
            QueueFamilyPropIndexGraphics += 1;
        }

        if(unsafe { SurfaceFN.get_physical_device_surface_support(VkPhysicalDevice, QueueFamilyPropIndexPresent, *VkSurface).unwrap() }){
            if(!ScannedPresentQueue){
                log::info!("The family of queues located at index {} supports surface, so it has been selected. QueueCount={}", QueueFamilyPropIndexPresent, &QueueFamilyProp.queue_count);
                QueueFamilyPropCountPresent = QueueFamilyProp.queue_count;
                ScannedPresentQueue = true;
            }
        }
        if(!ScannedPresentQueue){
            QueueFamilyPropIndexPresent += 1;
        }
    }
    
    // TODO: 实现自动分配优先级以获得更好的性能
    let GraphicsQueuePriorities: Vec<f32> = vec![1.0; QueueFamilyPropCountGraphics.try_into().unwrap()];
    let PresentQueuePriorities: Vec<f32> = vec![0.9; QueueFamilyPropCountPresent.try_into().unwrap()];

    // TODO: 使用循环实现批量创建
    let VK_DEVICE_QUEUE_CREATE_INFO_DEVICE:Vec<DeviceQueueCreateInfo> = vec![
        vk::DeviceQueueCreateInfo{
            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            queue_family_index: QueueFamilyPropIndexGraphics,
            queue_count: QueueFamilyPropCountGraphics,
            p_queue_priorities: GraphicsQueuePriorities.as_ptr(),
            ..Default::default()
        },
        vk::DeviceQueueCreateInfo{
            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            queue_family_index: QueueFamilyPropIndexPresent,
            queue_count: QueueFamilyPropCountPresent,
            p_queue_priorities: PresentQueuePriorities.as_ptr(),
            ..Default::default()
        },
    ];

    let VK_DEVICE_CREATE_INFO = vk::DeviceCreateInfo{
        s_type: vk::StructureType::DEVICE_CREATE_INFO,
        queue_create_info_count: 1,
        p_queue_create_infos: VK_DEVICE_QUEUE_CREATE_INFO_DEVICE.as_ptr(),
        pp_enabled_extension_names: VK_DEVICE_CREATE_INFO_ENABLE_EXTENSION.as_ptr(),
        enabled_extension_count: VK_DEVICE_CREATE_INFO_ENABLE_EXTENSION.len() as u32,
        ..Default::default()
    };

    let VkDevice = unsafe { VkInstance.create_device(VkPhysicalDevice, &VK_DEVICE_CREATE_INFO, None).unwrap() };

    return (VkDevice, (QueueFamilyPropIndexGraphics, QueueFamilyPropIndexPresent));
}

pub fn GetSwapChainSettings(Device: &PhysicalDevice, VkSurface: &SurfaceKHR, SurfaceFN: &Surface, window: &Window) -> (SurfaceCapabilitiesKHR, Extent2D, SurfaceFormatKHR, PresentModeKHR){
    unsafe{
        let SurfaceCapabilities = SurfaceFN.get_physical_device_surface_capabilities(*Device, *VkSurface).unwrap();
        let DeviceSupportedSurfaceFormats = SurfaceFN.get_physical_device_surface_formats(*Device, *VkSurface).unwrap();
        let DeviceSupportedSurfacePresentModes = SurfaceFN.get_physical_device_surface_present_modes(*Device, *VkSurface).unwrap();

        // 两个变量默认都为支持的第一个选项 会在下面的循环中修改为最优值 因为如果一个显卡连SRGB都不支持 那接下来也只是能跑就行了
        let mut SelectedSurfaceFormat: SurfaceFormatKHR = DeviceSupportedSurfaceFormats[0];
        let mut SelectedSurfacePresentMode: PresentModeKHR = DeviceSupportedSurfacePresentModes[0];

        let SelectedSwapExtent: Extent2D = ChooseSwapExtent(&SurfaceCapabilities, window);

        if (DeviceSupportedSurfaceFormats.len() < 1 || DeviceSupportedSurfacePresentModes.len() < 1){
            log::error!("The device does not support any SurfaceFormat or SurfacePresentMode and the app will most likely fail to start!");
            msgbox::create("MoonRaysEngine ERROR", "The device does not support any SurfaceFormat or SurfacePresentMode and the app will most likely fail to start!\nYou may need to update your graphics card driver or upgrade your graphics card.", msgbox::IconType::Error).unwrap();
        }

        for SurfaceFormat in DeviceSupportedSurfaceFormats{
            // 默认优先选择SRGB
            // TODO: 加入HDR支持
            if(SurfaceFormat.format == vk::Format::B8G8R8A8_SRGB && SurfaceFormat.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR){
                log::info!("Chose SurfaceFormat=B8G8R8A8_SRGB ColorSpace=SRGB");
                SelectedSurfaceFormat = SurfaceFormat;
            }
        }

        for SurfacePresentMode in DeviceSupportedSurfacePresentModes{
            // 默认使用VK_PRESENT_MODE_FIFO_KHR（也就是 垂直同步）
            // TODO: 通过配置文件定义此选项 就像普通游戏让你选择是否开启垂直同步一样
            if(SurfacePresentMode == vk::PresentModeKHR::FIFO){
                log::info!("Chose SurfacePresentMode=FIFO");
                SelectedSurfacePresentMode = SurfacePresentMode;
            }
        }

        return (SurfaceCapabilities,SelectedSwapExtent, SelectedSurfaceFormat, SelectedSurfacePresentMode);
    }
}

pub fn ChooseSwapExtent(capabilities: &vk::SurfaceCapabilitiesKHR, window: &Window) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::MAX {
        capabilities.current_extent
    } else {
        let (width, height) = (window.inner_size().width, window.inner_size().height);

        let mut actual_extent = vk::Extent2D {
            width: width as u32,
            height: height as u32,
        };

        actual_extent.width = actual_extent.width.clamp(
            capabilities.min_image_extent.width,
            capabilities.max_image_extent.width,
        );
        actual_extent.height = actual_extent.height.clamp(
            capabilities.min_image_extent.height,
            capabilities.max_image_extent.height,
        );

        actual_extent
    }
}

pub fn GetSwapChain(instance: &Instance, device: &Device, VkSurface: &SurfaceKHR, SwapChainSettings: &(SurfaceCapabilitiesKHR, Extent2D, SurfaceFormatKHR, PresentModeKHR), _QueueFamilyIndices: &(u32, u32)) -> (SwapchainKHR, Swapchain){
    let mut swapchain_minImageCount = 4;
    if(SwapChainSettings.0.max_image_count != 0 && swapchain_minImageCount > SwapChainSettings.0.max_image_count){
        swapchain_minImageCount = SwapChainSettings.0.max_image_count;
    }

    // 类型转换
    // TODO: 未来这里应该直接就是一个Vec
    let QueueFamilyIndices = vec![_QueueFamilyIndices.0, _QueueFamilyIndices.1];

    // 为了防止指针指向的变量被释放 导致Vulkan不能访问 这里先存储一下
    let CreateInfo_ImageFormat = SwapChainSettings.2.format;
    let CreateInfo_ColorSpace = SwapChainSettings.2.color_space;
    let CreateInfo_Extent = SwapChainSettings.1;
    let mut CreateInfo_PreTransform;
    if (SwapChainSettings.0.supported_transforms.contains(vk::SurfaceTransformFlagsKHR::IDENTITY)){
        CreateInfo_PreTransform = vk::SurfaceTransformFlagsKHR::IDENTITY;
    }
    else {
        CreateInfo_PreTransform = SwapChainSettings.0.current_transform;
    }
    let CreateInfo_PresentMode = SwapChainSettings.3;

    log::info!("Prepare to create swapchain, QueueFamilyIndices[0]={} QueueFamilyIndices[1]={}", &QueueFamilyIndices[0], &QueueFamilyIndices[1]);

    let VK_SWAPCHAIN_CREATE_INFO_DEFAULT = vk::SwapchainCreateInfoKHR {
        s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
        surface: VkSurface.to_owned(),
        min_image_count: swapchain_minImageCount,
        image_format: CreateInfo_ImageFormat,
        image_color_space: CreateInfo_ColorSpace,
        image_extent: CreateInfo_Extent,
        image_array_layers: 1, // TODO: 对3D应用程序做出更好的适配
        image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        image_sharing_mode: vk::SharingMode::EXCLUSIVE,
        queue_family_index_count: 2,
        p_queue_family_indices: Box::into_raw(QueueFamilyIndices.into_boxed_slice()) as *const u32, // 使用Box::into_raw来获取一个不会被释放的指针
        pre_transform: CreateInfo_PreTransform,
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE, // TODO: 支持Alpha通道
        present_mode: CreateInfo_PresentMode,
        clipped: vk::TRUE,
        ..Default::default()
    };

    let VkSwapChainFN = extensions::khr::Swapchain::new(instance, device);
    log::info!("SwapChainFN was created");
    let VkSwapChain = unsafe {
        VkSwapChainFN
            .create_swapchain(&VK_SWAPCHAIN_CREATE_INFO_DEFAULT, None)
            .unwrap()
    };
    log::info!("SwapChain was created");

    return (VkSwapChain, VkSwapChainFN);
}