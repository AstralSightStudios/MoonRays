using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk.GraphicsPipeline;

public static class VkVertexInput
{
    public static PipelineVertexInputStateCreateInfo BuildVertexInputStateCreateInfo()
    {
        return new PipelineVertexInputStateCreateInfo()
        {
            SType = StructureType.PipelineVertexInputStateCreateInfo,
            VertexBindingDescriptionCount = 0,
            PVertexBindingDescriptions = null,
            VertexAttributeDescriptionCount = 0,
            PVertexAttributeDescriptions = null
        };
    }
}