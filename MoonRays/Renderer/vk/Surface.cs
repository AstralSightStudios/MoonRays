using Silk.NET.Core.Native;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk;

public static class VkSurface
{
    public static unsafe void Create()
    {
        VkNonDispatchableHandle surfaceHandle;
        Window.Main.glfw.CreateWindowSurface(VulkanRenderer.Instance.ToHandle(), Window.Main.window, null, &surfaceHandle);

        VulkanRenderer.SurfaceKHR = surfaceHandle.ToSurface();
    }
}