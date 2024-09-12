using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk.GraphicsPipeline;

public static class VkRasterizer
{
    public static PipelineRasterizationStateCreateInfo BuildRasterizationStateCreateInfo()
    {
        return new PipelineRasterizationStateCreateInfo()
        {
            SType = StructureType.PipelineRasterizationStateCreateInfo,
            DepthClampEnable = false,
            RasterizerDiscardEnable = false,
            PolygonMode = PolygonMode.Fill,
            LineWidth = 1.0f,
            CullMode = CullModeFlags.BackBit,
            FrontFace = FrontFace.Clockwise,
            DepthBiasEnable = false,
        };
    }
}