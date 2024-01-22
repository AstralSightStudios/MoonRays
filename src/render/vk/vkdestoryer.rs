use ash::vk;

pub fn DestoryAndFreeBuffer(VkDevice: &ash::Device, buffer: vk::Buffer, bufferMemory: vk::DeviceMemory){
    unsafe { VkDevice.destroy_buffer(buffer, None) };
    unsafe { VkDevice.free_memory(bufferMemory, None) };
}

pub fn DestoryAndFreeImage(VkDevice: &ash::Device, image: vk::Image, imageMemory: vk::DeviceMemory){
    unsafe { VkDevice.destroy_image(image, None) };
    unsafe { VkDevice.free_memory(imageMemory, None) };
}