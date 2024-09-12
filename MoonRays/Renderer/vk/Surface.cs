using Silk.NET.Core.Native;
using Silk.NET.SDL;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk;

public static class VkSurface
{
    public static unsafe void Create()
    {
        VkNonDispatchableHandle surfaceHandle;
        var result = Window.Main.sdl.VulkanCreateSurface(Window.Main.window, VulkanRenderer.Instance.ToHandle(), &surfaceHandle);
        
        if (result == SdlBool.False)
        {
            throw new Exception($"Failed to create surface, error info: {Tools.NativeType.BytePtrToString(Window.Main.sdl.GetError())}");
        }

        VulkanRenderer.SurfaceKHR = surfaceHandle.ToSurface();
    }
}