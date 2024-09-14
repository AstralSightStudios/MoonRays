using Serilog;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk;

public static class VkFramebuffers
{
    public static unsafe void Create()
    {
        foreach (var swapchainImageView in VulkanRenderer.SwapchainImageViews)
        {
            var attachments = new []
            {
                swapchainImageView
            };
            
            fixed(ImageView* attachmentsPtr = attachments)
            {
                var createInfo = new FramebufferCreateInfo()
                {
                    SType = StructureType.FramebufferCreateInfo,
                    RenderPass = VulkanRenderer.RenderPass,
                    AttachmentCount = 1,
                    PAttachments = attachmentsPtr,
                    Width = VkSwapChain.extent.Width,
                    Height = VkSwapChain.extent.Height,
                    Layers = 1
                };
                
                Framebuffer framebuffer;
                VulkanRenderer.VkApi().CreateFramebuffer(VulkanRenderer.Device, &createInfo, null, &framebuffer);
                Log.Information("Created framebuffer");
                
                VulkanRenderer.SwapChainFramebuffers.Add(framebuffer);
            }
        }
    }
}