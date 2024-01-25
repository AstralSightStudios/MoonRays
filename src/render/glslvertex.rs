#[derive(Clone, Copy)]
pub struct UniformBufferObject {
    pub(crate) Model: glm::Mat4,
    pub(crate) View: glm::Mat4,
    pub(crate) Proj: glm::Mat4
}

#[derive(Clone, Copy)]
pub struct GlslVertexBase{
    pub(crate) pos: glm::Vec2,
    pub(crate) color: glm::Vec3
}

impl GlslVertexBase{
    pub fn GetBindingDescription() -> ash::vk::VertexInputBindingDescription{
        let VERTEX_INPUT_BINDING_DESCRIPTION_DEFAULT = ash::vk::VertexInputBindingDescription{
            binding: 0,
            stride: std::mem::size_of::<GlslVertexBase>() as u32,
            input_rate: ash::vk::VertexInputRate::VERTEX,
        };
        
        return VERTEX_INPUT_BINDING_DESCRIPTION_DEFAULT;
    }

    pub fn GetAttributeDescriptions() -> Vec<ash::vk::VertexInputAttributeDescription>{
        let AttributesVec = vec![
            ash::vk::VertexInputAttributeDescription{
                location: 0, 
                binding: 0, 
                format: ash::vk::Format::R32G32_SFLOAT, 
                offset: memoffset::offset_of!(GlslVertexBase, pos) as u32
            },
            ash::vk::VertexInputAttributeDescription{
                location: 1, 
                binding: 0, 
                format: ash::vk::Format::R32G32B32_SFLOAT, 
                offset: memoffset::offset_of!(GlslVertexBase, color) as u32
            }
        ];

        return AttributesVec;
    }
}