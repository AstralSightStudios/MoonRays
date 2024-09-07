using MoonRays.Tools;
using Serilog;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk;

public static class VkPhysicalDevice
{
    private static unsafe List<PhysicalDevice> GetPhysicalDevices()
    {
        uint physicalDeviceCount = 0;
        VulkanRenderer.VkApi().EnumeratePhysicalDevices(VulkanRenderer.Instance, ref physicalDeviceCount, null);
        PhysicalDevice[] physicalDevices = new PhysicalDevice[physicalDeviceCount];
        VulkanRenderer.VkApi().EnumeratePhysicalDevices(VulkanRenderer.Instance, ref physicalDeviceCount, ref physicalDevices[0]);
        
        return new List<PhysicalDevice>(physicalDevices);
    }

    private static unsafe List<PhysicalDeviceProperties> GetPhysicalDeviceProperties(List<PhysicalDevice> physicalDevices)
    {
        List<PhysicalDeviceProperties> physicalDevicePropertiesList = new List<PhysicalDeviceProperties>();
        foreach (var physicalDevice in physicalDevices)
        {
            PhysicalDeviceProperties physicalDeviceProperties = new PhysicalDeviceProperties();
            VulkanRenderer.VkApi().GetPhysicalDeviceProperties(physicalDevice, out physicalDeviceProperties);
            physicalDevicePropertiesList.Add(physicalDeviceProperties);
        }
        
        return physicalDevicePropertiesList;
    }

    private static PhysicalDevice ChoseBestPhysicalDevice(List<PhysicalDevice> physicalDevices,
        List<PhysicalDeviceProperties> physicalDeviceProperties)
    {
        List<long> scores = new List<long>();
        foreach (var physicalDeviceProperty in physicalDeviceProperties)
        {
            long score = 0;
            score = score + physicalDeviceProperty.Limits.MaxVertexInputBindings + physicalDeviceProperty.Limits.MaxImageDimension3D + physicalDeviceProperty.Limits.MaxUniformBufferRange;
            if (physicalDeviceProperty.DeviceType == PhysicalDeviceType.DiscreteGpu)
            {
                score += 1000;
            }
            
            scores.Add(score);
        }

        return physicalDevices[Tools.ListUtils.IndexOfMax(scores)];
    }
    public static unsafe void Create()
    {
        var physicalDevices = GetPhysicalDevices();
        var physicalDevicePropertieses = GetPhysicalDeviceProperties(physicalDevices);
        
        Log.Information("Physical Devices: ");
        foreach (var physicalDevicePropertiese in physicalDevicePropertieses)
        {
            Log.Information($"      - {NativeType.BytePtrToString(physicalDevicePropertiese.DeviceName)} API_VERSION={physicalDevicePropertiese.ApiVersion}");
        }
        
        VulkanRenderer.PhysicalDevice = ChoseBestPhysicalDevice(physicalDevices, physicalDevicePropertieses);
    }
}