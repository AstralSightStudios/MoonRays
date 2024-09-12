using MoonRays.Renderer.vk;
using MoonRays.Renderer.vk.GraphicsPipeline;
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
    public static SwapchainKHR SwapchainKHR;
    public static KhrSwapchain SwapchainInstance;
    public static List<Image> SwapchainImages;
    public static List<ImageView> SwapchainImageViews = new();
    public static RenderPass RenderPass;
    public static Pipeline GraphicsPipeline;
    
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
        GetApi().TryGetDeviceExtension(Instance, Device, out SwapchainInstance);
        VkSwapChain.Create();
        VkImageViews.Create();
        VkPipeline.Create();
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