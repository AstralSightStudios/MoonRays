use ash::vk;

pub fn DoDrawTask(VkDevice: &ash::Device, VkGraphicsPipeline: &Vec<vk::Pipeline>, VkViewPort: &Vec<vk::Viewport>, VkViewRect2D: &Vec<vk::Rect2D>, VkRenderPass: &vk::RenderPass, VkCommandBuffers: &Vec<vk::CommandBuffer>, VkFrameBuffers: &Vec<vk::Framebuffer>, SwapChainSettings: &(vk::SurfaceCapabilitiesKHR, vk::Extent2D, vk::SurfaceFormatKHR, vk::PresentModeKHR), ImageIndex: usize){
    let mut LoopIndex = 0; //由于VkCommandBuffers与VkFrameBuffers长度相等，因此通用一个循环
    for VkCommandBuffer in VkCommandBuffers{
        let BeginInfo = vk::CommandBufferBeginInfo{
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            flags: vk::CommandBufferUsageFlags::SIMULTANEOUS_USE,
            ..Default::default()
        };

        unsafe { VkDevice.begin_command_buffer(*VkCommandBuffer, &BeginInfo).unwrap() };

        let RPBeginInfo = vk::RenderPassBeginInfo{
            s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
            render_pass: *VkRenderPass,
            framebuffer: VkFrameBuffers[ImageIndex],
            render_area: vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: SwapChainSettings.1 },
            clear_value_count: 1,
            p_clear_values: vec![crate::RENDER_VK_CLEAR_COLOR].as_ptr(),
            ..Default::default()
        };

        unsafe{
            VkDevice.cmd_begin_render_pass(*VkCommandBuffer, &RPBeginInfo, vk::SubpassContents::INLINE);
            VkDevice.cmd_bind_pipeline(*VkCommandBuffer, vk::PipelineBindPoint::GRAPHICS, VkGraphicsPipeline[LoopIndex]);

            VkDevice.cmd_set_viewport(*VkCommandBuffer, 0, &VkViewPort);
            VkDevice.cmd_set_scissor(*VkCommandBuffer, 0, &VkViewRect2D);

            VkDevice.cmd_draw(*VkCommandBuffer, 3, 1, 0, 0) ;

            VkDevice.cmd_end_render_pass(*VkCommandBuffer);
            VkDevice.end_command_buffer(*VkCommandBuffer).unwrap();

            //log::info!("Draw Command Sended");
        }
    }
}