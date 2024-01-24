use ash::vk;

pub fn FindMemoryType(
    VkInstance: &ash::Instance,
    VkPhysicalDevice: &ash::vk::PhysicalDevice,
    type_filter: u32,
    properties: vk::MemoryPropertyFlags,
) -> u32 {
    let mem_properties = unsafe { VkInstance.get_physical_device_memory_properties(*VkPhysicalDevice) };
    for i in 0..mem_properties.memory_type_count {
        if (type_filter & (1 << i)) != 0
            && (mem_properties.memory_types[i as usize].property_flags & properties) == properties
        {
            return i;
        }
    }
    log::error!("failed to find suitable memory type!");
    panic!("failed to find suitable memory type!");
}

pub fn GetVertexBuffer(VkInstance: &ash::Instance, VkPhysicalDevice: &ash::vk::PhysicalDevice ,VkDevice: &ash::Device , Vertices: &Vec<super::GlslVertex::GlslVertexBase>) -> vk::Buffer{
    let VK_BUFFER_CREATE_INFO_DEFAULT = vk::BufferCreateInfo{
        s_type: vk::StructureType::BUFFER_CREATE_INFO,
        size: (std::mem::size_of::<super::GlslVertex::GlslVertexBase>() * (std::mem::size_of::<super::GlslVertex::GlslVertexBase>() * Vertices.len())) as u64,
        usage: vk::BufferUsageFlags::VERTEX_BUFFER,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        ..Default::default()
    };

    let VkVertexBuffer = unsafe { VkDevice.create_buffer(&VK_BUFFER_CREATE_INFO_DEFAULT, None).unwrap() };

    let VkVertexBufferMemoryRequirements = unsafe { VkDevice.get_buffer_memory_requirements(VkVertexBuffer) };

    let VK_MEMORY_ALLOC_VK_VERTEX_BUFFER = vk::MemoryAllocateInfo{
        s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
        allocation_size: VkVertexBufferMemoryRequirements.size,
        // 0b10 = VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT
        memory_type_index: FindMemoryType(VkInstance, VkPhysicalDevice, VkVertexBufferMemoryRequirements.memory_type_bits, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT),
        ..Default::default()
    };
    let VkVertexBufferMemory = unsafe { VkDevice.allocate_memory(&VK_MEMORY_ALLOC_VK_VERTEX_BUFFER, None).unwrap() };
    unsafe { VkDevice.bind_buffer_memory(VkVertexBuffer, VkVertexBufferMemory, 0).unwrap() };

    let MemData = unsafe { VkDevice.map_memory(VkVertexBufferMemory, 0, VK_BUFFER_CREATE_INFO_DEFAULT.size, vk::MemoryMapFlags::empty()).unwrap() };

    unsafe {
        std::ptr::copy_nonoverlapping(
            Vertices.as_ptr() as *const u8,
            MemData as *mut u8,
            VK_BUFFER_CREATE_INFO_DEFAULT.size as usize,
        );
    }

    unsafe { VkDevice.unmap_memory(VkVertexBufferMemory) };

    return VkVertexBuffer;
}

pub fn CreateGeneralBuffer(VkInstance: &ash::Instance,VkPhysicalDevice: &ash::vk::PhysicalDevice, VkDevice: &ash::Device, VkDeviceSize: vk::DeviceSize, VkBufferUsage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags) -> (vk::Buffer, vk::DeviceMemory){
    let VK_BUFFER_CREATE_INFO_DEFAULT = vk::BufferCreateInfo{
        s_type: vk::StructureType::BUFFER_CREATE_INFO,
        size: VkDeviceSize,
        usage: VkBufferUsage,
        ..Default::default()
    };

    let CreatedBuffer = unsafe { VkDevice.create_buffer(&VK_BUFFER_CREATE_INFO_DEFAULT, None).unwrap() };
    let CreatedBufferVRAMRequirements = unsafe { VkDevice.get_buffer_memory_requirements(CreatedBuffer) };

    let VK_MEMORY_ALLOC_INFO_DEFAULT = vk::MemoryAllocateInfo{
        s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
        allocation_size: CreatedBufferVRAMRequirements.size,
        memory_type_index: FindMemoryType(VkInstance, VkPhysicalDevice, CreatedBufferVRAMRequirements.memory_type_bits, properties),
        ..Default::default()
    };

    let AllocedMemory = unsafe { VkDevice.allocate_memory(&VK_MEMORY_ALLOC_INFO_DEFAULT, None).unwrap() };

    unsafe { VkDevice.bind_buffer_memory(CreatedBuffer, AllocedMemory, 0).unwrap() };

    return (CreatedBuffer, AllocedMemory)
}

pub fn CopyBuffer(VkDevice: &ash::Device, VkCommandPool: &vk::CommandPool, VkGraphicsQueue: &vk::Queue, src: vk::Buffer, dst: vk::Buffer, size: vk::DeviceSize){
    let VkCommandBuffer = super::VkCommand::SINGLE_BeginCommands(VkDevice, VkCommandPool);

    let CopyRegion = vk::BufferCopy{
        size,
        ..Default::default()
    };

    unsafe { VkDevice.cmd_copy_buffer(VkCommandBuffer, src, dst, &[CopyRegion]) };

    super::VkCommand::SINGLE_EndCommandsAndSubmit(VkDevice, VkGraphicsQueue, VkCommandPool, &VkCommandBuffer);
}

pub fn TransitionImageLayout(VkDevice: &ash::Device, VkCommandPool: &vk::CommandPool, VkGraphicsQueue: &vk::Queue, image: vk::Image, format: vk::Format, old_layout: vk::ImageLayout, new_layout: vk::ImageLayout){
    let VkCommandBuffer = super::VkCommand::SINGLE_BeginCommands(VkDevice, VkCommandPool);

    let mut barrier = vk::ImageMemoryBarrier{
        s_type: vk::StructureType::IMAGE_MEMORY_BARRIER,
        old_layout,
        new_layout,
        src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
        dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
        image,
        subresource_range: vk::ImageSubresourceRange{
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        },
        src_access_mask: vk::AccessFlags::empty(),
        dst_access_mask: vk::AccessFlags::empty(),
        ..Default::default()
    };

    log::info!("barrier created");

    // 定义源和目标管线阶段变量
    unsafe {
            let mut source_stage: vk::PipelineStageFlags = vk::PipelineStageFlags::empty();
            let mut destination_stage: vk::PipelineStageFlags = vk::PipelineStageFlags::empty();

        // 根据旧的和新的图像布局，设置屏障的访问掩码和管线阶段
        if old_layout == vk::ImageLayout::UNDEFINED && new_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL {
            log::info!("old_layout == vk::ImageLayout::UNDEFINED && new_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL");
            // 如果从未定义布局转换到传输目标优化布局
            barrier.src_access_mask = vk::AccessFlags::empty(); // 源访问掩码为空
            barrier.dst_access_mask = vk::AccessFlags::TRANSFER_WRITE; // 目标访问掩码为传输写入
            source_stage = vk::PipelineStageFlags::TOP_OF_PIPE; // 源管线阶段为管线顶端
            destination_stage = vk::PipelineStageFlags::TRANSFER; // 目标管线阶段为传输
        } else if old_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL && new_layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL {
            log::info!("old_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL && new_layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL");
            // 如果从传输目标优化布局转换到着色器只读优化布局
            barrier.src_access_mask = vk::AccessFlags::TRANSFER_WRITE; // 源访问掩码为传输写入
            barrier.dst_access_mask = vk::AccessFlags::SHADER_READ; // 目标访问掩码为着色器读取
            source_stage = vk::PipelineStageFlags::TRANSFER; // 源管线阶段为传输
            destination_stage = vk::PipelineStageFlags::FRAGMENT_SHADER; // 目标管线阶段为片段着色器
        } else {
            // 如果不支持的布局转换，抛出异常
            log::error!("unsupported layout transition!");
        }

        log::info!("source_stage={:?} destination_stage={:?}", source_stage, destination_stage);

        VkDevice.cmd_pipeline_barrier(
            VkCommandBuffer, 
            source_stage, 
            destination_stage, 
            vk::DependencyFlags::empty(), 
            
            &[], 
            
            &[], 
            &[barrier]
        ) 
    }

    super::VkCommand::SINGLE_EndCommandsAndSubmit(VkDevice, VkGraphicsQueue, VkCommandPool, &VkCommandBuffer);
}

pub fn CopyBufferToImage(VkDevice: &ash::Device, VkCommandPool: &vk::CommandPool, VkGraphicsQueue: &vk::Queue, buffer: vk::Buffer, image: vk::Image, width: u32, height: u32){
    let VkCommandBuffer = super::VkCommand::SINGLE_BeginCommands(VkDevice, VkCommandPool);

    let region = vk::BufferImageCopy{
        buffer_offset: 0,
        buffer_row_length: 0,
        buffer_image_height: 0,
        image_subresource: vk::ImageSubresourceLayers{
            aspect_mask: vk::ImageAspectFlags::COLOR,
            mip_level: 0,
            base_array_layer: 0,
            layer_count: 1
        },
        image_offset: vk::Offset3D{
            x: 0,
            y: 0,
            z: 0
        },
        image_extent: vk::Extent3D{
            width,
            height,
            depth: 1
        },
    };

    unsafe { VkDevice.cmd_copy_buffer_to_image(
        VkCommandBuffer, 
        buffer, 
        image, 
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        &[region]
    ) };

    super::VkCommand::SINGLE_EndCommandsAndSubmit(VkDevice, VkGraphicsQueue, VkCommandPool, &VkCommandBuffer);
}

pub fn CreateIndexBuffer(VkInstance: &ash::Instance,VkPhysicalDevice: &ash::vk::PhysicalDevice, VkDevice: &ash::Device,VkCommandPool: &vk::CommandPool,VkGraphicsQueue: &vk::Queue, indices: Vec<u32>) -> vk::Buffer{
    let VkBufferSize = std::mem::size_of::<u32>() * indices.len();
    let VkBufferCreateRet = CreateGeneralBuffer(VkInstance, VkPhysicalDevice, VkDevice, VkBufferSize as u64, vk::BufferUsageFlags::TRANSFER_SRC, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);
    let stagingBuffer = VkBufferCreateRet.0;
    let stagingBufferMemory = VkBufferCreateRet.1;

    let MemData = unsafe { VkDevice.map_memory(stagingBufferMemory, 0, VkBufferSize as u64, vk::MemoryMapFlags::empty()).unwrap() };
    unsafe {
        std::ptr::copy_nonoverlapping(
            indices.as_ptr() as *const u8,
            MemData as *mut u8,
            VkBufferSize,
        );
    }
    unsafe { VkDevice.unmap_memory(stagingBufferMemory) };

    let VkIndexBufferCreateRet = CreateGeneralBuffer(VkInstance, VkPhysicalDevice, VkDevice, VkBufferSize as u64, vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER, vk::MemoryPropertyFlags::DEVICE_LOCAL);
    let VkIndexBuffer = VkIndexBufferCreateRet.0;
    let VkIndexBufferMemory = VkIndexBufferCreateRet.1;

    CopyBuffer(VkDevice, VkCommandPool, VkGraphicsQueue, stagingBuffer, VkIndexBuffer, VkBufferSize as u64);

    super::VkDestoryer::DestoryAndFreeBuffer(VkDevice, stagingBuffer, stagingBufferMemory);

    return VkIndexBuffer;
}