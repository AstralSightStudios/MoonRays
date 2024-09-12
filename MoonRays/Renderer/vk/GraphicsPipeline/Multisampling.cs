using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk.GraphicsPipeline;

public static class VkMultisampling
{
    public static PipelineMultisampleStateCreateInfo BuildMultisampleStateCreateInfo()
    {
        return new PipelineMultisampleStateCreateInfo()
        {
            SType = StructureType.PipelineMultisampleStateCreateInfo,
            SampleShadingEnable = false,
            RasterizationSamples = Config.Engine.Config.GraphicsSettings.MultisampleRasterizationSamples,
        };
    }
}