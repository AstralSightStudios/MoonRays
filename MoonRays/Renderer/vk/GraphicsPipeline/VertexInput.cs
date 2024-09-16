using MoonRays.Renderer.vk.Shader;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk.GraphicsPipeline;

public static unsafe class VkVertexInput
{
    public static PipelineVertexInputStateCreateInfo BuildVertexInputStateCreateInfo()
    {
        Shader.Vertex[] vertices = new Vertex[]
        {
            new Vertex() { pos = { x = 0.0f, y = -0.5f }, color = { r = 1.0f, g = 0.0f, b = 0.0f } },
            new Vertex() { pos = { x = 0.5f, y = 0.5f }, color = { r = 0.0f, g = 1.0f, b = 0.0f } },
            new Vertex() { pos = { x = -0.5f, y = 0.5f }, color = { r = 0.0f, g = 0.0f, b = 1.0f } }
        };

        VertexInputBindingDescription[] bindingDescriptions = new VertexInputBindingDescription[]{ VertexTools.GetBindingDescription() };
        var attributeDescriptions = VertexTools.GetAttributeDescriptions().ToArray();
        
        fixed(VertexInputAttributeDescription* attributeDescriptionsPtr = attributeDescriptions)
        fixed(VertexInputBindingDescription* bindingDescriptionsPtr = bindingDescriptions)
            return new PipelineVertexInputStateCreateInfo()
            {
                SType = StructureType.PipelineVertexInputStateCreateInfo,
                VertexBindingDescriptionCount = 1,
                PVertexBindingDescriptions = bindingDescriptionsPtr,
                VertexAttributeDescriptionCount = (uint)attributeDescriptions.Length,
                PVertexAttributeDescriptions = attributeDescriptionsPtr,
            };
    }
}