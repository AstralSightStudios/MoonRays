use ash::vk;

pub fn GetVkSemaphore(VkDevice: &ash::Device) -> (vk::Semaphore, vk::Semaphore){
    let SemaphoreCreateInfo = vk::SemaphoreCreateInfo{
        s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
        ..Default::default()
    };

    let VkSemaphoreImageAvailable = unsafe { VkDevice.create_semaphore(&SemaphoreCreateInfo, None).unwrap() };
    let VkSemaphoreRenderFinished = unsafe { VkDevice.create_semaphore(&SemaphoreCreateInfo, None).unwrap() };

    log::info!("Semaphore was created");

    return (VkSemaphoreImageAvailable, VkSemaphoreRenderFinished);
}