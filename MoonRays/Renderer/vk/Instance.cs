using System.Text;
using MoonRays.Tools;
using Serilog;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk;

public static unsafe class VkInstance
{
    public static void LogAvailableLayers()
    {
        uint layerCount = 0;
        VulkanRenderer.VkApi().EnumerateInstanceLayerProperties(ref layerCount, null);
        
        LayerProperties[] availableLayers = new LayerProperties[layerCount];
        VulkanRenderer.VkApi().EnumerateInstanceLayerProperties(ref layerCount, ref availableLayers[0]);
        
        Log.Information("Available Layers ({LayerCount}) : ", layerCount);
        foreach (var layerPropertiese in availableLayers)
        {
            Log.Information("      - {LayerName}", NativeType.BytePtrToString(layerPropertiese.LayerName));
        }
    }

    public static List<String> GetWindowExtensions()
    {
        uint count = 0;
        var extensions = NativeType.BytePtrPtrToStringList(Window.Main.glfw.GetRequiredInstanceExtensions(out count), (int)count);
        
        Log.Information("Available Window Extensions :");
        extensions.ForEach(extension => Log.Information("      - {ExtensionName}", extension));
        
        return extensions;
    }
    public static void Create()
    {
        LogAvailableLayers();
        
        // 默认以窗口extensions初始化list
        List<String> enabledExtensions = GetWindowExtensions();
        enabledExtensions.AddRange(Config.Engine.Config.RendererSettings.VkEnabledExtensions);
        
        ApplicationInfo appInfo = AppInfo.GetAppInfo();
        try
        {
            fixed (Instance* instancePtr = &VulkanRenderer.Instance)
            {
                Log.Information("Creating Vk instance...");
                var createInfo = new InstanceCreateInfo()
                {
                    SType = StructureType.InstanceCreateInfo,
                    PApplicationInfo = &appInfo,
                    PpEnabledLayerNames =
                        NativeType.ConvertStringListToBytePointerArray(Config.Engine.Config.RendererSettings
                            .VkEnabledLayers),
                    EnabledLayerCount = (uint)Config.Engine.Config.RendererSettings.VkEnabledLayers.Count,
                    PpEnabledExtensionNames = NativeType.ConvertStringListToBytePointerArray(enabledExtensions),
                    EnabledExtensionCount = (uint)enabledExtensions.Count
                };
                if (Config.Engine.Config.RendererSettings.VkEnabledLayers.Contains("VK_LAYER_KHRONOS_validation"))
                {
                    Log.Information("validation enabled, binding debug callback...");
                    var dbgMessengerCreateInfo = DbgCallback.CreateInfoExt();
                    createInfo.PNext = &dbgMessengerCreateInfo;
                }
                VulkanRenderer.VkApi().CreateInstance(createInfo, null, instancePtr);
                Log.Information("Created Vk instance successfully");
            }
        }
        catch (Exception ex)
        {
            Log.Error("Failed to create Vk instance: {message}", ex.Message);
            throw;
        }
    }
}