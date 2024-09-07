using MoonRays.Tools;
using Serilog;
using Silk.NET.Core;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk;

public class QueueFamilyIndices
{
    public int? GraphicsFamily = null;
    public int? ComputeFamily = null;
    public int? TransferFamily = null;
    public int? PresentFamily = null;
}

public static class VkQueueFamily
{
    private static unsafe List<QueueFamilyProperties> GetQueueFamilyProperties(PhysicalDevice physicalDevice)
    {
        uint count = 0;
        VulkanRenderer.VkApi().GetPhysicalDeviceQueueFamilyProperties(physicalDevice, ref count, null);
        QueueFamilyProperties[] queueFamilyProperties = new QueueFamilyProperties[count];
        VulkanRenderer.VkApi()
            .GetPhysicalDeviceQueueFamilyProperties(physicalDevice, ref count, out queueFamilyProperties[0]);

        return new List<QueueFamilyProperties>(queueFamilyProperties);
    }
    
    public static void Find()
    {
        var queueFamilyIndices = new QueueFamilyIndices() {};
        var queueFamilyProperties = GetQueueFamilyProperties(VulkanRenderer.PhysicalDevice);

        var index = 0;
        foreach (var queueFamilyProperty in queueFamilyProperties)
        {
            if (queueFamilyProperty.QueueFlags.HasFlag(QueueFlags.GraphicsBit) && queueFamilyIndices.GraphicsFamily == null)
            {
                Log.Information($"[findQueueFamilyIndices] Found Graphics Queue, Index={index}");
                queueFamilyIndices.GraphicsFamily = index;
            }
            
            Bool32 presentSupported = false;
            VulkanRenderer.SurfaceInstance.GetPhysicalDeviceSurfaceSupport(VulkanRenderer.PhysicalDevice, (uint)index, VulkanRenderer.SurfaceKHR, out presentSupported);
            if (presentSupported && queueFamilyIndices.PresentFamily == null)
            {
                Log.Information($"[findQueueFamilyIndices] Found Present Queue, Index={index}");
                queueFamilyIndices.PresentFamily = index;
            }
            
            if (queueFamilyProperty.QueueFlags.HasFlag(QueueFlags.ComputeBit) && queueFamilyIndices.ComputeFamily == null && queueFamilyIndices.GraphicsFamily != index)
            {
                Log.Information($"[findQueueFamilyIndices] Found Compute Queue, Index={index}");
                queueFamilyIndices.ComputeFamily = index;
            }
            
            if (queueFamilyProperty.QueueFlags.HasFlag(QueueFlags.TransferBit) && queueFamilyIndices.TransferFamily == null && queueFamilyIndices.GraphicsFamily != index && queueFamilyIndices.ComputeFamily != index)
            {
                Log.Information($"[findQueueFamilyIndices] Found Transfer Queue, Index={index}");
                queueFamilyIndices.TransferFamily = index;
            }

            index++;
        }

        VulkanRenderer.QueueFamilyIndices = queueFamilyIndices;
    }

    public static unsafe List<DeviceQueueCreateInfo> GetQueueCreateInfos()
    {
        List<DeviceQueueCreateInfo> queueCreateInfos = new();
    
        float[] priorities = { 1.0f, 0.9f, 0.8f, 0.7f };
    
        void AddQueueCreateInfo(uint familyIndex, float* priorityPtr)
        {
            if (!queueCreateInfos.Any(q => q.QueueFamilyIndex == familyIndex))
            {
                queueCreateInfos.Add(new DeviceQueueCreateInfo()
                {
                    SType = StructureType.DeviceQueueCreateInfo,
                    QueueFamilyIndex = familyIndex,
                    QueueCount = 1,
                    PQueuePriorities = priorityPtr
                });
            }
        }
    
        fixed (float* pri1 = &priorities[0], pri09 = &priorities[1], pri08 = &priorities[2], pri07 = &priorities[3])
        {
            AddQueueCreateInfo((uint)VulkanRenderer.QueueFamilyIndices.GraphicsFamily, pri1);
            
            if (VulkanRenderer.QueueFamilyIndices.PresentFamily != VulkanRenderer.QueueFamilyIndices.GraphicsFamily)
            {
                AddQueueCreateInfo((uint)VulkanRenderer.QueueFamilyIndices.PresentFamily, pri09);
            }

            AddQueueCreateInfo((uint)VulkanRenderer.QueueFamilyIndices.ComputeFamily, pri08);
            AddQueueCreateInfo((uint)VulkanRenderer.QueueFamilyIndices.TransferFamily, pri07);
        }

        return queueCreateInfos;
    }

}