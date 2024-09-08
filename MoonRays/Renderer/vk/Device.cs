using MoonRays.Tools;
using Serilog;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk;

public class DeviceQueues
{
    public Queue Graphics;
    public Queue Compute;
    public Queue Transfer;
}

public static class VkDevice
{
    public static unsafe void Create(List<DeviceQueueCreateInfo> queueCreateInfos)
    {
        fixed (DeviceQueueCreateInfo* pQueueCreateInfos = queueCreateInfos.ToArray())
        {
            var createInfo = new DeviceCreateInfo()
            {
                SType = StructureType.DeviceCreateInfo,
                PQueueCreateInfos = pQueueCreateInfos,
                QueueCreateInfoCount = (uint)queueCreateInfos.Count,
                PpEnabledExtensionNames = NativeType.ConvertStringListToBytePointerArray(Config.Engine.Config.RendererSettings.VkDeviceEnabledExtensions),
                EnabledExtensionCount = (uint)Config.Engine.Config.RendererSettings.VkDeviceEnabledExtensions.Count,
            };

            fixed (Device* devicePtr = &VulkanRenderer.Device)
            {
                VulkanRenderer.VkApi().CreateDevice(VulkanRenderer.PhysicalDevice, &createInfo, null, devicePtr);
            }
        }
        
        Log.Information("Created Vulkan device");
    }

    public static void GetDeviceQueues()
    {
        var graphicsQueue = new Queue();
        VulkanRenderer.VkApi().GetDeviceQueue(VulkanRenderer.Device, (uint)VulkanRenderer.QueueFamilyIndices.GraphicsFamily, 0, out graphicsQueue);
        var computeQueue = new Queue();
        VulkanRenderer.VkApi().GetDeviceQueue(VulkanRenderer.Device, (uint)VulkanRenderer.QueueFamilyIndices.ComputeFamily, 0, out computeQueue);
        var transferQueue = new Queue();
        VulkanRenderer.VkApi().GetDeviceQueue(VulkanRenderer.Device, (uint)VulkanRenderer.QueueFamilyIndices.TransferFamily, 0, out transferQueue);

        VulkanRenderer.DeviceQueues = new DeviceQueues()
        {
            Graphics = graphicsQueue,
            Compute = computeQueue,
            Transfer = transferQueue
        };
    }
}