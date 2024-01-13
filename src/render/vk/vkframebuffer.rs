use ash::vk;

pub fn GetSwapChainFrameBuffers(VkDevice: &ash::Device, VkSwapChainImageViews: &Vec<vk::ImageView>, VkRenderPass: &vk::RenderPass, SwapChainSettings: &(vk::SurfaceCapabilitiesKHR, vk::Extent2D, vk::SurfaceFormatKHR, vk::PresentModeKHR)) -> Vec<vk::Framebuffer>{
    let mut VkSwapChainFrameBuffers: Vec<vk::Framebuffer> = vec![];

    for SwapChainImageView in VkSwapChainImageViews{
        let Attachments = vec![*SwapChainImageView];

        let VK_FRAME_BUFFER_CREATE_INFO = vk::FramebufferCreateInfo{
            s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
            render_pass: *VkRenderPass,
            attachment_count: Attachments.len() as u32,
            p_attachments: Attachments.as_ptr(),
            width: SwapChainSettings.1.width as u32,
            height: SwapChainSettings.1.height as u32,
            layers: 1,
            ..Default::default()
        };

        VkSwapChainFrameBuffers.push(unsafe { VkDevice.create_framebuffer(&VK_FRAME_BUFFER_CREATE_INFO, None).unwrap() });
    }

    return VkSwapChainFrameBuffers;
}