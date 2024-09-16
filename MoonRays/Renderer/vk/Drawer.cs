using Silk.NET.Vulkan;
using Semaphore = Silk.NET.Vulkan.Semaphore;

namespace MoonRays.Renderer.vk;

public static class Drawer
{
    public static int CurrentFrame = 0;
    public static unsafe void DrawFrame()
    {
        var inFlightFence = VkSyncObjects.InFlightFences[CurrentFrame];
        VulkanRenderer.VkApi().WaitForFences(VulkanRenderer.Device, 1, &inFlightFence, true, UInt64.MaxValue);

        uint imageIndex;
        var result = VulkanRenderer.SwapchainInstance.AcquireNextImage(VulkanRenderer.Device, VulkanRenderer.SwapchainKHR, UInt64.MaxValue, VkSyncObjects.ImageAvailableSemaphores[CurrentFrame], new Fence(), &imageIndex);
        if (result == Result.ErrorOutOfDateKhr || (result == Result.SuboptimalKhr &&
                                                   Config.Engine.Config.RendererSettings.ReCreateSwapChainWhenSuboptimal))
        {
            VkSwapChain.ReCreate();
            return;
        }
        if (result != Result.Success)
        {
            throw new Exception("Failed to acquire swap chain image!");
        }
        
        VulkanRenderer.VkApi().ResetFences(VulkanRenderer.Device, 1, &inFlightFence);

        VulkanRenderer.VkApi().ResetCommandBuffer(VulkanRenderer.CommandBuffers[CurrentFrame], 0);
        VkCommandBuffer.Record(VulkanRenderer.CommandBuffers[CurrentFrame], (int)imageIndex);

        var waitSemaphores = (new List<Semaphore>() { VkSyncObjects.ImageAvailableSemaphores[CurrentFrame] }).ToArray();
        var waitStages = (new List<PipelineStageFlags> { PipelineStageFlags.ColorAttachmentOutputBit }).ToArray();
        var commandBuffers = (new List<CommandBuffer>() { VulkanRenderer.CommandBuffers[CurrentFrame] }).ToArray();
        var signalSemaphores = (new List<Semaphore>() { VkSyncObjects.RenderFinishedSemaphores[CurrentFrame] }).ToArray();
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

        CurrentFrame = (CurrentFrame + 1) % Config.Engine.Config.GraphicsSettings.MaxFramesInFlight;
    }
}