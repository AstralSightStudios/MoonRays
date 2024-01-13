use ash::vk;

pub fn GetCommandPool(VkDevice: &ash::Device, QueueFamilyIndices: &(u32, u32)) -> vk::CommandPool{
    let VK_COMMAND_POOL_CREATE_INFO = vk::CommandPoolCreateInfo{
        s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
        queue_family_index: QueueFamilyIndices.0,
        ..Default::default()
    };

    let VkCommandPool = unsafe { VkDevice.create_command_pool(&VK_COMMAND_POOL_CREATE_INFO, None).unwrap() };
    log::info!("CommandPool was created");
    return VkCommandPool;
}

pub fn GetCommandBuffers(VkDevice: &ash::Device, VkSwapChainFrameBuffers: &Vec<vk::Framebuffer>, VkCommandPool: &vk::CommandPool) -> Vec<vk::CommandBuffer>{
    let AllocInfo = vk::CommandBufferAllocateInfo{
        s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        command_pool: *VkCommandPool,
        level: vk::CommandBufferLevel::PRIMARY,
        command_buffer_count: VkSwapChainFrameBuffers.len() as u32,
        ..Default::default()
    };

    let VkCommandBuffers = unsafe { VkDevice.allocate_command_buffers(&AllocInfo).unwrap() };
    log::info!("CommandBuffers was created.");
    return VkCommandBuffers;
}