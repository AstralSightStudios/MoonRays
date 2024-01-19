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