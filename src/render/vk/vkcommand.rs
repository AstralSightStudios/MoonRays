use ash::vk;

pub fn GetCommandPool(VkDevice: &ash::Device, QueueFamilyIndices: &(u32, u32)) -> vk::CommandPool{
    let VK_COMMAND_POOL_CREATE_INFO = vk::CommandPoolCreateInfo{
        s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
        queue_family_index: QueueFamilyIndices.0,
        flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
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

pub fn SINGLE_BeginCommands(VkDevice: &ash::Device, VkCommandPool: &vk::CommandPool) -> vk::CommandBuffer{
    let AllocInfo = vk::CommandBufferAllocateInfo{
        s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        command_pool: *VkCommandPool,
        level: vk::CommandBufferLevel::PRIMARY,
        command_buffer_count: 1,
        ..Default::default()
    };

    let VkCommandBuffers = unsafe { VkDevice.allocate_command_buffers(&AllocInfo).unwrap() };
    unsafe { VkDevice.begin_command_buffer(VkCommandBuffers[0], &vk::CommandBufferBeginInfo{
        s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
        flags: vk::CommandBufferUsageFlags::SIMULTANEOUS_USE,
        ..Default::default()
    }).unwrap() };
    log::info!("[SINGLE CommandBuffer] Begin");
    return VkCommandBuffers[0];
}

pub fn SINGLE_EndCommandsAndSubmit(VkDevice: &ash::Device, VkGraphicsQueue: &vk::Queue, VkCommandPool: &vk::CommandPool, VkCommandBuffer: &vk::CommandBuffer){
    unsafe { VkDevice.end_command_buffer(*VkCommandBuffer).unwrap() };
    let VkCommandBuffer_S = vec![*VkCommandBuffer];

    let SubmitInfo = vec! [vk::SubmitInfo {
        s_type: vk::StructureType::SUBMIT_INFO,
        command_buffer_count: 1,
        p_command_buffers: VkCommandBuffer_S.as_ptr(),
        ..Default::default ()
    }];

    unsafe { VkDevice.queue_submit(*VkGraphicsQueue, &SubmitInfo, vk::Fence::null()).unwrap() };
    unsafe { VkDevice.queue_wait_idle(*VkGraphicsQueue).unwrap() };

    unsafe { VkDevice.free_command_buffers(*VkCommandPool, &[*VkCommandBuffer]) };

    log::info!("[SINGLE CommandBuffer] End");
}