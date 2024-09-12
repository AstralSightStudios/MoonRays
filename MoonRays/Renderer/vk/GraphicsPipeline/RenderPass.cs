using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk.GraphicsPipeline;

public static class VkRenderPass
{
    public static unsafe void Create()
    {
        var colorAttachment = new AttachmentDescription()
        {
            Format = VkSwapChain.surfaceFormat.Format,
            Samples = Config.Engine.Config.GraphicsSettings.RenderPassColorSamples,
            LoadOp = AttachmentLoadOp.Clear,
            StoreOp = AttachmentStoreOp.Store,
            StencilLoadOp = AttachmentLoadOp.DontCare,
            StencilStoreOp = AttachmentStoreOp.DontCare,
            InitialLayout = ImageLayout.Undefined,
            FinalLayout = ImageLayout.PresentSrcKhr
        };

        var colorAttachmentRef = new AttachmentReference()
        {
            Attachment = 0,
            Layout = ImageLayout.ColorAttachmentOptimal,
        };

        var subpass = new SubpassDescription()
        {
            PipelineBindPoint = PipelineBindPoint.Graphics,
            ColorAttachmentCount = 1,
            PColorAttachments = &colorAttachmentRef,
        };

        var renderPassCreateInfo = new RenderPassCreateInfo()
        {
            SType = StructureType.RenderPassCreateInfo,
            AttachmentCount = 1,
            PAttachments = &colorAttachment,
            SubpassCount = 1,
            PSubpasses = &subpass,
        };
        
        VulkanRenderer.VkApi().CreateRenderPass(VulkanRenderer.Device, &renderPassCreateInfo, null, out VulkanRenderer.RenderPass);
    }
}