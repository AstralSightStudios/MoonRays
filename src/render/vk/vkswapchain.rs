use ash::{Instance, Device,vk::{self,Image, ImageView, PhysicalDevice, SurfaceKHR, SurfaceFormatKHR, PresentModeKHR, SurfaceCapabilitiesKHR ,Extent2D, SwapchainKHR}, extensions::khr::Surface, extensions::khr::Swapchain};
use winit::window::Window;
use ash::extensions;

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

pub fn GetSwapChainImages(VkSwapChain: &SwapchainKHR, VkSwapChainFN: &Swapchain) -> Vec<Image>{
    let VkSwapChainImages = unsafe { VkSwapChainFN.get_swapchain_images(*VkSwapChain).unwrap() };
    return VkSwapChainImages;
}

pub fn GetSwapChainImageViews(VkDevice: &Device, VkSwapChainImages: &Vec<Image>, SwapChainSettings: &(SurfaceCapabilitiesKHR, Extent2D, SurfaceFormatKHR, PresentModeKHR)) -> Vec<ImageView>{
    let mut VkSwapChainImageViews: Vec<ImageView> = vec![];

    for VkImage in VkSwapChainImages{
        VkSwapChainImageViews.push(super::VkTexture::CreateImageView(VkDevice, VkImage, &SwapChainSettings.2.format));
    }

    return VkSwapChainImageViews;
}