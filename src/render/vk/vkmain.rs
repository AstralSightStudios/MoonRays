/*
    vkmain.rs
    作为Vulkan图形API的主入口，做一些基础操作例如创建Instance等等
*/

use std::{ptr::null, ffi::{CString, CStr}, process::exit};
use ash::{self, vk::{self, PhysicalDevice, QueueFlags, SurfaceKHR, DeviceQueueCreateInfo, PhysicalDeviceFeatures}, Entry, Instance, extensions::{khr::Surface, self}};
use raw_window_handle::HasRawDisplayHandle;
use winit::{event_loop::EventLoop, window::Window};

use self::{VkPipeline::GetGraphicsPipeline, VkShader::GetBaseShadersPipelineShaderStage};
#[path="../../base/sysinfo.rs"]
pub(crate) mod SysInfo;
#[path="../../hardwaretools/canrunchecker.rs"]
mod CanRunChecker;
#[path="./vksurface.rs"]
mod VkSurfaceTools;
#[path="./vkwindow.rs"]
mod VkWindowTools;
#[path="./vkdebugger.rs"]
mod VkDebugger;
#[path="./vkswapchain.rs"]
mod VkSwapChain;
#[path="./vkpipeline.rs"]
mod VkPipeline;
#[path="./vkshader.rs"]
mod VkShader;
#[path="./vkframebuffer.rs"]
mod VkFrameBuffer;
#[path="./vkcommand.rs"]
mod VkCommand;
#[path="./vkdrawer.rs"]
pub(crate) mod VkDrawer;
#[path="./vksemaphores.rs"]
mod VkSemaphores;
#[path="../spirv_compiler.rs"]
mod SpirvCompiler;

#[derive(PartialEq, Clone, Copy)]
pub struct QueueFamilyIndices{
    pub GraphicsQueueIndex: u32,
    pub PresentQueueIndex: u32
}

#[derive(Clone)]
pub struct RenderEngineVK{
    pub VkBase: (Entry, Instance),
    pub VkPhysicalDevice: PhysicalDevice,
    pub VkDevice: ash::Device,
    pub VkQueueFamilyIndicesGraphicsAndPresent: QueueFamilyIndices,
    pub VkSurface: (SurfaceKHR, Surface),
    pub VkSwapChainSettings: (vk::SurfaceCapabilitiesKHR, vk::Extent2D, vk::SurfaceFormatKHR, vk::PresentModeKHR),
    pub VkSwapChain: (vk::SwapchainKHR, extensions::khr::Swapchain),
    pub VkImages: Vec<vk::Image>,
    pub VkImageViews: Vec<vk::ImageView>,
    pub VkPipelineShaderStages: Vec<vk::PipelineShaderStageCreateInfo>,
    pub VkPipeline: Vec<vk::Pipeline>,
    pub VkRenderPass: vk::RenderPass,
    pub VkViewport: Vec<vk::Viewport>,
    pub VkRect2DRenderArea: Vec<vk::Rect2D>,
    pub VkFrameBuffers: Vec<vk::Framebuffer>,
    pub VkCommandPool: vk::CommandPool,
    pub VkCommandBuffers: Vec<vk::CommandBuffer>,
    pub VkSemaphoreImageAvailable: vk::Semaphore,
    pub VkSemaphoreRenderFinished: vk::Semaphore
}

impl RenderEngineVK{
    // 我们拒绝屎山，小子
    // 这个函数用于将LoadVKTuple得到的依托答辩存放进一个美观的别墅中
    pub fn LoadVK() -> (RenderEngineVK, (Window, EventLoop<()>)){
        let LoadResult = LoadVKTuple();
        return (RenderEngineVK{
            VkBase: LoadResult.1,
            VkPhysicalDevice: LoadResult.2,
            VkDevice: LoadResult.3.0,
            VkQueueFamilyIndicesGraphicsAndPresent: QueueFamilyIndices{
                GraphicsQueueIndex: LoadResult.3.1.0,
                PresentQueueIndex: LoadResult.3.1.1,
            },
            VkSurface: LoadResult.4,
            VkSwapChainSettings: LoadResult.5, // 需要在swapchain重建中重新创建
            VkSwapChain: LoadResult.6, // 需要在swapchain重建中重新创建
            VkImages: LoadResult.7, // 需要在swapchain重建中重新创建
            VkImageViews: LoadResult.8, // 需要在swapchain重建中重新创建
            VkPipelineShaderStages: LoadResult.9, // 需要在swapchain重建中重新创建
            VkPipeline: LoadResult.10.0, // 需要在swapchain重建中重新创建
            VkRenderPass: LoadResult.10.1, // 需要在swapchain重建中重新创建
            VkViewport: LoadResult.10.2, // 需要在swapchain重建中重新创建
            VkRect2DRenderArea: LoadResult.10.3, // 需要在swapchain重建中重新创建
            VkFrameBuffers: LoadResult.11, // 需要在swapchain重建中重新创建
            VkCommandPool: LoadResult.12,
            VkCommandBuffers: LoadResult.13,
            VkSemaphoreImageAvailable: LoadResult.14.0,
            VkSemaphoreRenderFinished: LoadResult.14.1
        }, LoadResult.0)
    }

    // 窗口大小更改了，重开吧牢底😭
    // 这个函数用来重创建Surface以及后续的所有东西
    pub fn RemakeSurface(&mut self, WinitWindow: &(Window, EventLoop<()>)){
        unsafe { 
            self.VkDevice.device_wait_idle().unwrap();
            for FrameBuffer in &self.VkFrameBuffers{
                self.VkDevice.destroy_framebuffer(*FrameBuffer, None);
            }
            for VkImageView in &self.VkImageViews{
                self.VkDevice.destroy_image_view(*VkImageView, None);
            }
            self.VkSwapChain.1.destroy_swapchain(self.VkSwapChain.0, None);
        }

        self.VkSwapChainSettings = VkSwapChain::GetSwapChainSettings(&self.VkPhysicalDevice, &self.VkSurface.0, &self.VkSurface.1, &WinitWindow.0);
        self.VkSwapChain = VkSwapChain::GetSwapChain(&self.VkBase.1, &self.VkDevice, &self.VkSurface.0, &self.VkSwapChainSettings, &(self.VkQueueFamilyIndicesGraphicsAndPresent.GraphicsQueueIndex, self.VkQueueFamilyIndicesGraphicsAndPresent.PresentQueueIndex));
        self.VkImages = VkSwapChain::GetSwapChainImages(&self.VkSwapChain.0, &self.VkSwapChain.1);
        self.VkImageViews = VkSwapChain::GetSwapChainImageViews(&self.VkDevice, &self.VkImages, &self.VkSwapChainSettings);
        //self.VkPipelineShaderStages = GetBaseShadersPipelineShaderStage(&self.VkDevice);
        //let Temp_PipelineCreateRet = GetGraphicsPipeline(&self.VkDevice, &self.VkPipelineShaderStages, &self.VkSwapChainSettings);
        //self.VkPipeline = Temp_PipelineCreateRet.0;
        //self.VkRenderPass = Temp_PipelineCreateRet.1;
        //self.VkViewport = Temp_PipelineCreateRet.2;
        //self.VkRect2DRenderArea = Temp_PipelineCreateRet.3;
        self.VkFrameBuffers = VkFrameBuffer::GetSwapChainFrameBuffers(&self.VkDevice, &self.VkImageViews, &self.VkRenderPass, &self.VkSwapChainSettings);
    }
}

fn LoadVKTuple() -> ((Window, EventLoop<()>), (Entry, Instance), PhysicalDevice, (ash::Device, (u32, u32)), (SurfaceKHR, Surface), (vk::SurfaceCapabilitiesKHR, vk::Extent2D, vk::SurfaceFormatKHR, vk::PresentModeKHR), (vk::SwapchainKHR, extensions::khr::Swapchain), Vec<vk::Image>, Vec<vk::ImageView>, Vec<vk::PipelineShaderStageCreateInfo>, (Vec<vk::Pipeline>, vk::RenderPass, Vec<vk::Viewport>, Vec<vk::Rect2D>), Vec<vk::Framebuffer>, vk::CommandPool, Vec<vk::CommandBuffer>, (vk::Semaphore, vk::Semaphore)){
    SpirvCompiler::CompileBaseShaders();

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
    let mut DeviceFeatures = vec![
        PhysicalDeviceFeatures{
            depth_clamp: 1,
            logic_op: 1,
            ..Default::default()
        }
    ];
    // 关于VkDevice返回的两个u32：第一个是Graphics在QueueFamily中的索引，第二个是Present的索引
    let VkDevice = GetVkDevice(&VkReturn.1, VkPhysicalDevice, &VkSurface.0, &VkSurface.1, DeviceExts, DeviceFeatures);
    let VkSwapChainSettings = VkSwapChain::GetSwapChainSettings(&VkPhysicalDevice, &VkSurface.0, &VkSurface.1, &VkWindow.0);
    let VkSwapChain = VkSwapChain::GetSwapChain(&VkReturn.1, &VkDevice.0, &VkSurface.0, &VkSwapChainSettings, &VkDevice.1);
    let VkSwapChainImages = VkSwapChain::GetSwapChainImages(&VkSwapChain.0, &VkSwapChain.1);
    let VkSwapChainImageViews = VkSwapChain::GetSwapChainImageViews(&VkDevice.0, &VkSwapChainImages, &VkSwapChainSettings);
    let VkBaseShaderStages = GetBaseShadersPipelineShaderStage(&VkDevice.0);
    let VkGraphicsPipeline = GetGraphicsPipeline(&VkDevice.0, &VkBaseShaderStages, &VkSwapChainSettings);
    let VkSwapChainFrameBuffers = VkFrameBuffer::GetSwapChainFrameBuffers(&VkDevice.0, &VkSwapChainImageViews, &VkGraphicsPipeline.1, &VkSwapChainSettings);
    let VkCommandPool = VkCommand::GetCommandPool(&VkDevice.0, &VkDevice.1);
    let VkCommandBuffers = VkCommand::GetCommandBuffers(&VkDevice.0, &VkSwapChainFrameBuffers, &VkCommandPool);
    let VkSemaphore = VkSemaphores::GetVkSemaphore(&VkDevice.0);

    VkDrawer::DoDrawTask(&VkDevice.0, &VkGraphicsPipeline.0, &VkGraphicsPipeline.2, &VkGraphicsPipeline.3, &VkGraphicsPipeline.1, &VkCommandBuffers, &VkSwapChainFrameBuffers, &VkSwapChainSettings, 0);

    return (VkWindow, VkReturn, VkPhysicalDevice, VkDevice, VkSurface.clone(), VkSwapChainSettings, VkSwapChain, VkSwapChainImages, VkSwapChainImageViews, VkBaseShaderStages, VkGraphicsPipeline, VkSwapChainFrameBuffers, VkCommandPool, VkCommandBuffers, VkSemaphore);
}

fn CreateInstance(VK_INSTANCE_CREATE_INFO_ENABLE_EXTENSION: Vec<*const i8>, VK_INSTANCE_CREATE_INFO_ENABLE_LAYERS: Vec<*const i8>) -> (Entry, Instance){
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

fn GetPhysicalDevice(VkInstance: &Instance) -> PhysicalDevice{
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
        let name = crate::Tools::veci8_to_string(available_device_prop.device_name.to_vec());

        log::info!("   - {}", name);

        // 显卡选择优先度：无法识别的用作显示输出的显卡 < AMD独显/Vega核显 < Intel Arc独显 < NVIDIA任意显卡
        if(name.contains("AMD") || name.contains("Radeon") || name.contains("RX") || name.contains("ATI") || name.contains("Vega")){
            return_device = unsafe { VkInstance.enumerate_physical_devices().unwrap() }[name_check_index];
        }
        if(name.contains("Arc") && name.contains("Intel")){
            return_device = unsafe { VkInstance.enumerate_physical_devices().unwrap() }[name_check_index];
        }
        if(name.contains("NVIDIA")){
            return_device = unsafe { VkInstance.enumerate_physical_devices().unwrap() }[name_check_index];
        }

        name_check_index += 1;
    }

    let return_device_name_binding = crate::Tools::veci8_to_string(unsafe { VkInstance.get_physical_device_properties(return_device).device_name.to_vec() });
    return_device_name = &return_device_name_binding;

    log::info!("The graphics card has been selected: {}", return_device_name);

    return return_device;
}

fn GetVkDevice(VkInstance: &Instance, VkPhysicalDevice: PhysicalDevice,VkSurface: &SurfaceKHR, SurfaceFN: &Surface, VK_DEVICE_CREATE_INFO_ENABLE_EXTENSION: Vec<*const i8>, VK_DEVICE_CREATE_INFO_ENABLE_FEATURES: Vec<PhysicalDeviceFeatures>) -> (ash::Device,(u32, u32)){
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
        p_enabled_features: VK_DEVICE_CREATE_INFO_ENABLE_FEATURES.as_ptr(),
        ..Default::default()
    };

    let VkDevice = unsafe { VkInstance.create_device(VkPhysicalDevice, &VK_DEVICE_CREATE_INFO, None).unwrap() };

    return (VkDevice, (QueueFamilyPropIndexGraphics, QueueFamilyPropIndexPresent));
}