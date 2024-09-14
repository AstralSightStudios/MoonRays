using Silk.NET.Vulkan;
using Semaphore = Silk.NET.Vulkan.Semaphore;

namespace MoonRays.Renderer.vk;

public static class Drawer
{
    public static unsafe void DrawFrame()
    {
        var inFlightFence = VkSyncObjects.InFlightFence;
        VulkanRenderer.VkApi().WaitForFences(VulkanRenderer.Device, 1, &inFlightFence, true, UInt64.MaxValue);
        VulkanRenderer.VkApi().ResetFences(VulkanRenderer.Device, 1, &inFlightFence);

        uint imageIndex;
        VulkanRenderer.SwapchainInstance.AcquireNextImage(VulkanRenderer.Device, VulkanRenderer.SwapchainKHR, UInt64.MaxValue, VkSyncObjects.ImageAvailableSemaphore, new Fence(), &imageIndex);

        VulkanRenderer.VkApi().ResetCommandBuffer(VulkanRenderer.CommandBuffer, 0);
        VkCommandBuffer.Record(VulkanRenderer.CommandBuffer, (int)imageIndex);

        var waitSemaphores = (new List<Semaphore>() { VkSyncObjects.ImageAvailableSemaphore }).ToArray();
        var waitStages = (new List<PipelineStageFlags> { PipelineStageFlags.ColorAttachmentOutputBit }).ToArray();
        var commandBuffers = (new List<CommandBuffer>() { VulkanRenderer.CommandBuffer }).ToArray();
        var signalSemaphores = (new List<Semaphore>() { VkSyncObjects.RenderFinishedSemaphore }).ToArray();
        var swapChains = (new List<SwapchainKHR>() { VulkanRenderer.SwapchainKHR }).ToArray();
        
        fixed (Semaphore* waitSemaphoresPtr = waitSemaphores)
        fixed (PipelineStageFlags* waitStagesPtr = waitStages)
        fixed (CommandBuffer* commandBuffersPtr = commandBuffers)
        fixed (Semaphore* signalSemaphoresPtr = signalSemaphores)
        fixed (SwapchainKHR* swapChainsPtr = swapChains)
        {
            var submitInfo = new SubmitInfo()
            {
                SType = StructureType.SubmitInfo,
                WaitSemaphoreCount = 1,
                PWaitSemaphores = waitSemaphoresPtr,
                PWaitDstStageMask = waitStagesPtr,
                CommandBufferCount = 1,
                PCommandBuffers = commandBuffersPtr,
                SignalSemaphoreCount = 1,
                PSignalSemaphores = signalSemaphoresPtr
            };
            
            VulkanRenderer.VkApi().QueueSubmit(VulkanRenderer.DeviceQueues.Graphics, 1, &submitInfo, inFlightFence);
            
            var presentInfo = new PresentInfoKHR()
            {
                SType = StructureType.PresentInfoKhr,
                WaitSemaphoreCount = 1,
                PWaitSemaphores = signalSemaphoresPtr,
                SwapchainCount = 1,
                PSwapchains = swapChainsPtr,
                PImageIndices = &imageIndex,
                PResults = null
            };

            VulkanRenderer.SwapchainInstance.QueuePresent(VulkanRenderer.DeviceQueues.Present, &presentInfo);
        }
    }
}