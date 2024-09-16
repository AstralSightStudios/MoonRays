using Serilog;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk;

public static class VkDescriptorPool
{
    public static unsafe void Create()
    {
        var poolSize = new DescriptorPoolSize()
        {
            Type = DescriptorType.UniformBuffer,
            DescriptorCount = (uint)Config.Engine.Config.GraphicsSettings.MaxFramesInFlight
        };
        var poolInfo = new DescriptorPoolCreateInfo()
        {
            SType = StructureType.DescriptorPoolCreateInfo,
            PoolSizeCount = 1,
            PPoolSizes = &poolSize,
            MaxSets = (uint)Config.Engine.Config.GraphicsSettings.MaxFramesInFlight
        };

        VulkanRenderer.VkApi()
            .CreateDescriptorPool(VulkanRenderer.Device, &poolInfo, null, out VulkanRenderer.DescriptorPool);
        
        Log.Information("Created DescriptorPool");
    }
}