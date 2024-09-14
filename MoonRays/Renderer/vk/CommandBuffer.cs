using Serilog;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk;

public class VkCommandBuffer
{
    public static unsafe void Allocate()
    {
        var allocInfo = new CommandBufferAllocateInfo()
        {
            SType = StructureType.CommandBufferAllocateInfo,
            CommandPool = VulkanRenderer.CommandPool,
            Level = CommandBufferLevel.Primary,
            CommandBufferCount = 1
        };
        
        VulkanRenderer.VkApi().AllocateCommandBuffers(VulkanRenderer.Device, &allocInfo, out VulkanRenderer.CommandBuffer);
        Log.Information("Allocated Vulkan CommandBuffer");
    }

    public static unsafe void Record(CommandBuffer commandBuffer, int imageIndex)
    {
        var beginInfo = new CommandBufferBeginInfo()
        {
            SType = StructureType.CommandBufferBeginInfo,
            Flags = 0,
            PInheritanceInfo = null
        };
        VulkanRenderer.VkApi().BeginCommandBuffer(commandBuffer, &beginInfo);

        var clearColor = new ClearValue(new ClearColorValue()
        {
            Float32_0 = 1.0f,
            Float32_1 = 1.0f,
            Float32_2 = 1.0f,
            Float32_3 = 1.0f,
        });
        var clearColors = (new List<ClearValue>() { clearColor }).ToArray();

        fixed (ClearValue* pClearColors = clearColors)
        {
            var renderPassBeginInfo = new RenderPassBeginInfo()
            {
                SType = StructureType.RenderPassBeginInfo,
                RenderPass = VulkanRenderer.RenderPass,
                Framebuffer = VulkanRenderer.SwapChainFramebuffers[imageIndex],
                RenderArea = new Rect2D()
                {
                    Offset = new Offset2D() { X = 0, Y = 0 },
                    Extent = VkSwapChain.extent
                },
                ClearValueCount = 1,
                PClearValues = pClearColors
            };
            
            VulkanRenderer.VkApi().CmdBeginRenderPass(commandBuffer, &renderPassBeginInfo, SubpassContents.Inline);
            VulkanRenderer.VkApi().CmdBindPipeline(commandBuffer, PipelineBindPoint.Graphics, VulkanRenderer.GraphicsPipeline);

            var viewPort = GraphicsPipeline.VkViewPort.Viewport;
            var scissor = GraphicsPipeline.VkViewPort.Scissor;
            VulkanRenderer.VkApi().CmdSetViewport(commandBuffer, 0, 1, &viewPort);
            VulkanRenderer.VkApi().CmdSetScissor(commandBuffer, 0, 1, &scissor);
            
            VulkanRenderer.VkApi().CmdDraw(commandBuffer, 3, 1, 0, 0);
            
            VulkanRenderer.VkApi().CmdEndRenderPass(commandBuffer);
            VulkanRenderer.VkApi().EndCommandBuffer(commandBuffer);
        }
    }
}