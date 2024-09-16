using System.Runtime.InteropServices;
using MoonRays.Renderer;
using MoonRays.Tools;
using Serilog;
using Silk.NET.Vulkan;

namespace MoonRays.UI.dev;

public static class ImGui
{
    public static unsafe void Init()
    {
        ImGuiBinding.hello();
        
        var version = ImGuiBinding.ImGui_GetVersion();
        Log.Information($"Loaded ImGUI. Version: {NativeType.BytePtrToString(version)}");

        ImGuiBinding.ImGui_CreateContext();
        var ioPtr = ImGuiBinding.ImGui_GetIO();

        // 从 IntPtr 读取 ImGuiIO 结构体
        var io = (ImGuiStructures.ImGuiIO*)ioPtr;
        io->ConfigFlags |= (int)ImGuiEnums.ImGuiConfigFlags.NavEnableKeyboard;
        io->ConfigFlags |= (int)ImGuiEnums.ImGuiConfigFlags.NavEnableGamepad;

        ImGuiBinding.cImGui_ImplSDL2_InitForVulkan(Window.Main.window);
        
        var vkImguiInitInfo = new ImGuiStructures.ImGui_ImplVulkan_InitInfo()
        {
            Instance = VulkanRenderer.Instance.Handle,
            PhysicalDevice = VulkanRenderer.PhysicalDevice.Handle,
            Device = VulkanRenderer.Device.Handle,
            QueueFamily = (uint)VulkanRenderer.QueueFamilyIndices.GraphicsFamily,
            Queue = VulkanRenderer.DeviceQueues.Graphics.Handle,
        };
        
        //ImGuiBinding.cImGui_ImplVulkan_Init(ref vkImguiInitInfo);
    }
}