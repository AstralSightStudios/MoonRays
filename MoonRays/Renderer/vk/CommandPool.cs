using Serilog;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk;

public static class VkCommandPool
{
    public static unsafe void Create()
    {
        var createInfo = new CommandPoolCreateInfo()
        {
            SType = StructureType.CommandPoolCreateInfo,
            Flags = CommandPoolCreateFlags.ResetCommandBufferBit,
            QueueFamilyIndex = (uint)VulkanRenderer.QueueFamilyIndices.GraphicsFamily
        };

        VulkanRenderer.VkApi().CreateCommandPool(VulkanRenderer.Device, &createInfo, null, out VulkanRenderer.CommandPool);
        Log.Information("CommandPool created");
    }
}