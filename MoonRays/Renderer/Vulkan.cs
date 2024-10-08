﻿using MoonRays.Renderer.vk;
using MoonRays.Renderer.vk.GraphicsPipeline;
using MoonRays.UI.dev;
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
    public static List<Framebuffer> SwapChainFramebuffers = new();
    public static CommandPool CommandPool;
    public static CommandBuffer[] CommandBuffers = new CommandBuffer[Config.Engine.Config.GraphicsSettings.MaxFramesInFlight];
    
    public static DescriptorPool DescriptorPool;
    
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
        VkFramebuffers.Create();
        VkCommandPool.Create();
        VkCommandBuffer.Allocate();
        VkSyncObjects.Create();

        VkDescriptorPool.Create();
        
        if (Config.Feature.EnableImGui)
        {
            ImGui.Init();
        }
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