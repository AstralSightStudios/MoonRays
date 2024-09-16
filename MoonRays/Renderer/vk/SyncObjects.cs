using Serilog;
using Silk.NET.Vulkan;
using Semaphore = Silk.NET.Vulkan.Semaphore;

namespace MoonRays.Renderer.vk;

public static class VkSyncObjects
{
    public static List<Semaphore> ImageAvailableSemaphores = new ();
    public static List<Semaphore> RenderFinishedSemaphores = new ();
    public static List<Fence> InFlightFences = new();
    
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

        for (var i = 0; i < Config.Engine.Config.GraphicsSettings.MaxFramesInFlight; i++)
        {
            Semaphore imageAvailableSemaphoreTmp;
            Semaphore renderFinishedSemaphoreTmp;
            Fence inFlightFenceTmp;
            
            VulkanRenderer.VkApi()
                .CreateSemaphore(VulkanRenderer.Device, &semaphoreInfo, null, out imageAvailableSemaphoreTmp);
            VulkanRenderer.VkApi()
                .CreateSemaphore(VulkanRenderer.Device, &semaphoreInfo, null, out renderFinishedSemaphoreTmp);
            VulkanRenderer.VkApi().CreateFence(VulkanRenderer.Device, &fenceInfo, null, out inFlightFenceTmp);
            
            ImageAvailableSemaphores.Add(imageAvailableSemaphoreTmp);
            RenderFinishedSemaphores.Add(renderFinishedSemaphoreTmp);
            InFlightFences.Add(inFlightFenceTmp);
        }

        Log.Information("Created Sync Objects");
    }
}