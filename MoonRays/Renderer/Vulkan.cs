using MoonRays.Renderer.vk;
using Silk.NET.Vulkan;
using Silk.NET.Vulkan.Extensions.KHR;
using static Silk.NET.Vulkan.Vk;
using PhysicalDevice = Silk.NET.Vulkan.PhysicalDevice;

namespace MoonRays.Renderer;

public static class VulkanRenderer
{
    private static Vk? _vkApi;
    public static Instance Instance;
    public static PhysicalDevice PhysicalDevice;
    public static QueueFamilyIndices QueueFamilyIndices;
    public static Device Device;
    public static DeviceQueues DeviceQueues;
    public static SurfaceKHR SurfaceKHR;
    public static KhrSurface SurfaceInstance;
    
    public static void Init()
    {
        _vkApi = Silk.NET.Vulkan.Vk.GetApi();
        VkInstance.Create();
        GetApi().TryGetInstanceExtension(Instance, out SurfaceInstance);
        VkPhysicalDevice.Create();
        VkSurface.Create();
        VkQueueFamily.Find();
        var queueCreateInfos = VkQueueFamily.GetQueueCreateInfos();
        VkDevice.Create(queueCreateInfos);
        VkDevice.GetDeviceQueues();
    }

    public static Vk VkApi()
    {
        if (_vkApi != null)
        {
            return _vkApi;
        }

        throw new NullReferenceException("Vk API is not initialized");
    }
}