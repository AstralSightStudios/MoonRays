using Serilog;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk;

public static class VkImageViews
{
    public static unsafe void Create()
    {
        foreach (var swapchainImage in VulkanRenderer.SwapchainImages)
        {
            var createInfo = new ImageViewCreateInfo()
            {
                SType = StructureType.ImageViewCreateInfo,
                Image = swapchainImage,
                ViewType = ImageViewType.Type2D,
                Format = VkSwapChain.surfaceFormat.Format,
                Components = new ComponentMapping()
                {
                    R = ComponentSwizzle.Identity,
                    G = ComponentSwizzle.Identity,
                    B = ComponentSwizzle.Identity,
                    A = ComponentSwizzle.Identity,
                },
                SubresourceRange = new ImageSubresourceRange()
                {
                    AspectMask = ImageAspectFlags.ColorBit,
                    BaseMipLevel = 0,
                    LevelCount = 1,
                    BaseArrayLayer = 0,
                    LayerCount = 1
                }
            };

            ImageView imageView = new ImageView();
            VulkanRenderer.VkApi().CreateImageView(VulkanRenderer.Device, &createInfo, null, out imageView);
            VulkanRenderer.SwapchainImageViews.Add(imageView);
        }
        Log.Information("Created image views");
    }
}