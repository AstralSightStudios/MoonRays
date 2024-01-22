use ash::vk;

pub fn CreateImage(VkInstance: &ash::Instance, VkPhysicalDevice: &vk::PhysicalDevice, VkDevice: &ash::Device ,width: u32, height: u32, VkImageType: &vk::ImageType, VkImageFormat: &vk::Format, VkImageTiling: &vk::ImageTiling, VkImageUsage: &vk::ImageUsageFlags, VkMemoryPropertyFlags: &vk::MemoryPropertyFlags) -> (vk::Image, vk::DeviceMemory){
    let VK_IMAGE_CREATE_INFO_DEFAULT = vk::ImageCreateInfo{
        s_type: vk::StructureType::IMAGE_CREATE_INFO,
        image_type: *VkImageType,
        extent: vk::Extent3D{
            width,
            height,
            depth: 1,
        },
        mip_levels: 1,
        array_layers: 1,
        format: *VkImageFormat,
        tiling: *VkImageTiling,
        initial_layout: vk::ImageLayout::UNDEFINED,
        usage: *VkImageUsage,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        samples: vk::SampleCountFlags::TYPE_1,
        ..Default::default()
    };

    let VkImage = unsafe { VkDevice.create_image(&VK_IMAGE_CREATE_INFO_DEFAULT, None).unwrap() };

    log::info!("Image was Created");

    let VkImageMemoryRequirements = unsafe { VkDevice.get_image_memory_requirements(VkImage) };

    let VK_IMAGE_MEMORY_ALLOC = vk::MemoryAllocateInfo{
        s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
        allocation_size: VkImageMemoryRequirements.size,
        memory_type_index: super::VkBuffer::FindMemoryType(VkInstance, VkPhysicalDevice, VkImageMemoryRequirements.memory_type_bits, *VkMemoryPropertyFlags),
        ..Default::default()
    };

    let VkImageMemory = unsafe { VkDevice.allocate_memory(&VK_IMAGE_MEMORY_ALLOC, None).unwrap() };

    unsafe { VkDevice.bind_image_memory(VkImage, VkImageMemory, 0).unwrap() }

    return (VkImage, VkImageMemory)
}

pub fn GetTextureImage(VkInstance: &ash::Instance, VkPhysicalDevice: &vk::PhysicalDevice, VkDevice: &ash::Device, VkGraphicsQueue: &vk::Queue, VkCommandPool: &vk::CommandPool, ImagePath: &String){
    let TargetImageDynamic = image::io::Reader::open(ImagePath).unwrap().with_guessed_format().unwrap().decode().unwrap();
    log::info!("image file loaded");
    let TargetImage = TargetImageDynamic.clone().into_rgba8().into_raw();
    //log::info!("{}", TargetImage.len());
    let TargetImageSize: vk::DeviceSize = (TargetImageDynamic.width() * TargetImageDynamic.height() * 4).into();
    let BufferCreateRet = super::VkBuffer::CreateGeneralBuffer(VkInstance, VkPhysicalDevice, VkDevice, TargetImageSize, vk::BufferUsageFlags::TRANSFER_SRC, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);
    let TargetImageBufferMemory = BufferCreateRet.1;
    let TargetImageBuffer = BufferCreateRet.0;
    let MemData = unsafe { VkDevice.map_memory(TargetImageBufferMemory, 0, TargetImageSize, vk::MemoryMapFlags::empty()).unwrap() };
    log::info!("TargetImageBuffer Memory Map");
    unsafe {
        std::ptr::copy_nonoverlapping(
            TargetImage.as_ptr() as *const u8,
            MemData as *mut u8,
            TargetImageSize as usize,
        );
    }
    log::info!("TargetImage was copiled to MemData");

    unsafe { VkDevice.unmap_memory(TargetImageBufferMemory) };

    log::info!("Target Image inited");

    let ImageCreateRet = CreateImage(VkInstance, VkPhysicalDevice, VkDevice, TargetImageDynamic.width(), TargetImageDynamic.height(), &vk::ImageType::TYPE_2D, &vk::Format::R8G8B8A8_UNORM, &vk::ImageTiling::OPTIMAL, &(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED), &vk::MemoryPropertyFlags::DEVICE_LOCAL);
    let TextureImage = ImageCreateRet.0;

    log::info!("Image inited");
    
    super::VkBuffer::TransitionImageLayout(VkDevice, VkCommandPool, VkGraphicsQueue, TextureImage, vk::Format::R8G8B8A8_SRGB, vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
    super::VkBuffer::CopyBufferToImage(VkDevice, VkCommandPool, VkGraphicsQueue, TargetImageBuffer, TextureImage, TargetImageDynamic.width(), TargetImageDynamic.height());
    super::VkBuffer::TransitionImageLayout(VkDevice, VkCommandPool, VkGraphicsQueue, TextureImage, vk::Format::R8G8B8A8_SRGB, vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);

    super::VkDestoryer::DestoryAndFreeBuffer(VkDevice, TargetImageBuffer, TargetImageBufferMemory);
}