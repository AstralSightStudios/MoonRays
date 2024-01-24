use std::ptr::null;

use ash::{self, vk::{self, SurfaceCapabilitiesKHR, Extent2D, SurfaceFormatKHR, PresentModeKHR, PipelineShaderStageCreateInfo, Rect2D}};

pub fn GetGraphicsPipeline(VkDevice: &ash::Device, ShaderStages: &Vec<PipelineShaderStageCreateInfo>, SwapChainSettings: &(SurfaceCapabilitiesKHR, Extent2D, SurfaceFormatKHR, PresentModeKHR)) -> (Vec<vk::Pipeline>, vk::RenderPass, Vec<vk::Viewport>, Vec<Rect2D>){
    let DYNAMIC_STATES = vec![
        vk::DynamicState::VIEWPORT,
        vk::DynamicState::SCISSOR,
        //vk::DynamicState::PRIMITIVE_TOPOLOGY
    ];

    let VK_PIPELINE_DYNAMIC_STATE_CREATE_INFO = vk::PipelineDynamicStateCreateInfo{
        s_type: vk::StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
        dynamic_state_count: DYNAMIC_STATES.len() as u32,
        p_dynamic_states: DYNAMIC_STATES.as_ptr(),
        ..Default::default()
    };

    let BindingDescription = vec![super::GlslVertex::GlslVertexBase::GetBindingDescription()];
    let AttributeDescription = super::GlslVertex::GlslVertexBase::GetAttributeDescriptions();

    let VK_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = vk::PipelineVertexInputStateCreateInfo{
        s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        vertex_binding_description_count: BindingDescription.len() as u32,
        p_vertex_binding_descriptions: BindingDescription.as_ptr(),
        vertex_attribute_description_count: AttributeDescription.len() as u32,
        p_vertex_attribute_descriptions: AttributeDescription.as_ptr(),
        ..Default::default()
    };

    let VK_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = vk::PipelineInputAssemblyStateCreateInfo{
        s_type: vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        primitive_restart_enable: vk::FALSE,
        ..Default::default()
    };

    let VkViewPort = vec![
        vk::Viewport{
            x: 0.0,
            y: 0.0,
            width: SwapChainSettings.1.width as f32,
            height: SwapChainSettings.1.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }
    ];

    let VkViewRect2D = vec![
        vk::Rect2D{
            offset: vk::Offset2D { x: (0), y: (0) },
            extent: SwapChainSettings.1,
        }
    ];

    let VK_PIPELINE_VIEWPORT_STATE_CREATE_INFO = vk::PipelineViewportStateCreateInfo{
        s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        viewport_count: VkViewPort.len() as u32,
        p_viewports: VkViewPort.as_ptr(),
        scissor_count: VkViewRect2D.len() as u32,
        p_scissors: VkViewRect2D.as_ptr(),
        ..Default::default()
    };

    let VK_PIPELINE_RASTERIZATION_STATE_CREATE_INFO = vk::PipelineRasterizationStateCreateInfo{
        s_type: vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        depth_clamp_enable: vk::TRUE,
        rasterizer_discard_enable: vk::FALSE,
        polygon_mode: vk::PolygonMode::FILL,
        cull_mode: vk::CullModeFlags::BACK,
        front_face: vk::FrontFace::CLOCKWISE,
        // TODO: 这个depthbias的实际用途有待发掘，据说是有时用于阴影斜射？
        depth_bias_enable: vk::FALSE,
        depth_bias_constant_factor: 0.0,
        depth_bias_clamp: 0.0,
        depth_bias_slope_factor: 0.0,
        line_width: 1.0,
        ..Default::default()
    };

    // TODO: 多重采样抗锯齿目前被禁用 有时间来做开启
    let VK_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = vk::PipelineMultisampleStateCreateInfo{
        s_type: vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        rasterization_samples: vk::SampleCountFlags::TYPE_1,
        sample_shading_enable: vk::FALSE,
        min_sample_shading: 1.0,
        p_sample_mask: null(),
        alpha_to_coverage_enable: vk::FALSE,
        alpha_to_one_enable: vk::FALSE,
        ..Default::default()
    };

    let VK_PIPELINE_COLOR_BLEND_ATTACHMENT_STATE = vk::PipelineColorBlendAttachmentState{
        blend_enable: vk::TRUE,
        src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
        dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        color_blend_op: vk::BlendOp::ADD,
        src_alpha_blend_factor: vk::BlendFactor::SRC_ALPHA,
        dst_alpha_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        alpha_blend_op: vk::BlendOp::ADD,
        color_write_mask: vk::ColorComponentFlags::RGBA,
    };

    let VK_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = vk::PipelineColorBlendStateCreateInfo{
        s_type: vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        logic_op_enable: vk::TRUE,
        logic_op: vk::LogicOp::COPY,
        attachment_count: 1,
        p_attachments: &VK_PIPELINE_COLOR_BLEND_ATTACHMENT_STATE,
        blend_constants: [0.0,0.0,0.0,0.0],
        ..Default::default()
    };

    let VK_PIPELINE_LAYOUT_CREATE_INFO = vk::PipelineLayoutCreateInfo{
        s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
        set_layout_count: 0,
        p_set_layouts: null(),
        push_constant_range_count: 0,
        p_push_constant_ranges: null(),
        ..Default::default()
    };

    let VkPipelineLayout = unsafe { VkDevice.create_pipeline_layout(&VK_PIPELINE_LAYOUT_CREATE_INFO, None).unwrap() };

    log::info!("PipelineLayout was created");

    let VkColorAttachment = vec![
        vk::AttachmentDescription{
            format: SwapChainSettings.2.format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            ..Default::default()
        }
    ];

    let VkColorAttachmentRef = vec![
        vk::AttachmentReference{
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }
    ];

    let VkSubPass = vec![
        vk::SubpassDescription{
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            color_attachment_count: VkColorAttachmentRef.len() as u32,
            p_color_attachments: VkColorAttachmentRef.as_ptr(),
            ..Default::default()
        }
    ];

    let VK_SUBPASS_DEPENDENCY = vec![
        vk::SubpassDependency{
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::NONE,
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            ..Default::default()
        }
    ];

    let VK_RENDER_PASS_CREATE_INFO = vk::RenderPassCreateInfo{
        s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
        attachment_count: VkColorAttachment.len() as u32,
        p_attachments: VkColorAttachment.as_ptr(),
        subpass_count: VkSubPass.len() as u32,
        p_subpasses: VkSubPass.as_ptr(),
        dependency_count: VK_SUBPASS_DEPENDENCY.len() as u32,
        p_dependencies: VK_SUBPASS_DEPENDENCY.as_ptr(),
        ..Default::default()
    };

    let VkRenderPass = unsafe { VkDevice.create_render_pass(&VK_RENDER_PASS_CREATE_INFO, None).unwrap()  };

    log::info!("RenderPass was created");

    let VK_GRAPHICS_PIPELINE_CREATE_INFO = vk::GraphicsPipelineCreateInfo{
        s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
        stage_count: 2,
        p_stages: ShaderStages.as_ptr(),
        p_vertex_input_state: &VK_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        p_input_assembly_state: &VK_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        p_viewport_state: &VK_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        p_rasterization_state: &VK_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        p_multisample_state: &VK_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        p_color_blend_state: &VK_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        p_dynamic_state: &VK_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
        layout: VkPipelineLayout,
        render_pass: VkRenderPass,
        subpass: 0,
        base_pipeline_handle: vk::Pipeline::null(),
        base_pipeline_index: -1,
        ..Default::default()
    };

    let VkGraphicsPipeline = unsafe { VkDevice.create_graphics_pipelines(vk::PipelineCache::null(), &[VK_GRAPHICS_PIPELINE_CREATE_INFO], None).unwrap() };

    log::info!("Graphics Pipeline was created");

    return (VkGraphicsPipeline, VkRenderPass, VkViewPort, VkViewRect2D);
}