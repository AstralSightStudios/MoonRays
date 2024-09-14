using Serilog;
using Silk.NET.Vulkan;
using Semaphore = Silk.NET.Vulkan.Semaphore;

namespace MoonRays.Renderer.vk;

public static class VkSyncObjects
{
    public static Semaphore ImageAvailableSemaphore;
    public static Semaphore RenderFinishedSemaphore;
    public static Fence InFlightFence;
    
    public static unsafe void Create()
    {
        var semaphoreInfo = new SemaphoreCreateInfo()
        {
            SType = StructureType.SemaphoreCreateInfo
        };
        var fenceInfo = new FenceCreateInfo()
        {
            SType = StructureType.FenceCreateInfo,
            Flags = FenceCreateFlags.SignaledBit
        };
        
        VulkanRenderer.VkApi().CreateSemaphore(VulkanRenderer.Device, &semaphoreInfo, null, out ImageAvailableSemaphore);
        VulkanRenderer.VkApi().CreateSemaphore(VulkanRenderer.Device, &semaphoreInfo, null, out RenderFinishedSemaphore);
        VulkanRenderer.VkApi().CreateFence(VulkanRenderer.Device, &fenceInfo, null, out InFlightFence);
        
        Log.Information("Created Sync Objects");
    }
}