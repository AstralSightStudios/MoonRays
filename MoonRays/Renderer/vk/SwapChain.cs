using Serilog;
using Silk.NET.SDL;
using Silk.NET.Vulkan;
using Silk.NET.Vulkan.Extensions.KHR;
using Event = Silk.NET.SDL.Event;

namespace MoonRays.Renderer.vk;

public class SwapChainSupportDetails
{
    public SurfaceCapabilitiesKHR SurfaceCapabilities;
    public List<SurfaceFormatKHR> SupportedSurfaceFormats;
    public List<PresentModeKHR> SupportedPresentModes;
}

public static class VkSwapChain
{
    public static SwapChainSupportDetails supportDetails = new SwapChainSupportDetails();
    public static SurfaceFormatKHR surfaceFormat;
    public static PresentModeKHR presentMode;
    public static Extent2D extent;
    
    private static unsafe List<SurfaceFormatKHR> GetSurfaceFormats()
    {
        uint count = 0;
        VulkanRenderer.SurfaceInstance.GetPhysicalDeviceSurfaceFormats(VulkanRenderer.PhysicalDevice,
            VulkanRenderer.SurfaceKHR, ref count, null);
        SurfaceFormatKHR[] formats = new SurfaceFormatKHR[count];
        VulkanRenderer.SurfaceInstance.GetPhysicalDeviceSurfaceFormats(VulkanRenderer.PhysicalDevice,
            VulkanRenderer.SurfaceKHR, ref count, out formats[0]);
        
        return new List<SurfaceFormatKHR>(formats);
    }
    
    private static unsafe List<PresentModeKHR> GetPresentFormats()
    {
        uint count = 0;
        VulkanRenderer.SurfaceInstance.GetPhysicalDeviceSurfacePresentModes(VulkanRenderer.PhysicalDevice,
            VulkanRenderer.SurfaceKHR, ref count, null);
        PresentModeKHR[] formats = new PresentModeKHR[count];
        VulkanRenderer.SurfaceInstance.GetPhysicalDeviceSurfacePresentModes(VulkanRenderer.PhysicalDevice,
            VulkanRenderer.SurfaceKHR, ref count, out formats[0]);
        
        return new List<PresentModeKHR>(formats);
    }
    
    private static void CheckSwapChainSupport()
    {
        supportDetails = new SwapChainSupportDetails();
        VulkanRenderer.SurfaceInstance.GetPhysicalDeviceSurfaceCapabilities(VulkanRenderer.PhysicalDevice, VulkanRenderer.SurfaceKHR, out supportDetails.SurfaceCapabilities);
        supportDetails.SupportedSurfaceFormats = GetSurfaceFormats();
        supportDetails.SupportedPresentModes = GetPresentFormats();
    }

    private static SurfaceFormatKHR ChooseSurfaceFormat()
    {
        foreach (var supportDetailsSupportedSurfaceFormat in supportDetails.SupportedSurfaceFormats)
        {
            if (supportDetailsSupportedSurfaceFormat.Format == Format.B8G8R8A8Unorm &&
                supportDetailsSupportedSurfaceFormat.ColorSpace == ColorSpaceKHR.SpaceSrgbNonlinearKhr)
            {
                return supportDetailsSupportedSurfaceFormat;
            }
        }
        Log.Warning("Your Graphics Card is not support best surface format, using default surface format. ");
        return supportDetails.SupportedSurfaceFormats[0];
    }

    private static PresentModeKHR ChoosePresentMode()
    {
        foreach (var supportDetailsSupportedPresentMode in supportDetails.SupportedPresentModes)
        {
            if (supportDetailsSupportedPresentMode == PresentModeKHR.MailboxKhr)
            {
                return supportDetailsSupportedPresentMode;
            }
        }

        Log.Warning("Your Graphics Card is not support best present mode, using default present mode (fifo_khr) . ");
        return PresentModeKHR.FifoKhr;
    }

    private static unsafe Extent2D ChooseSwapExtent()
    {
        if (supportDetails.SurfaceCapabilities.CurrentExtent.Width != uint.MaxValue)
        {
            return supportDetails.SurfaceCapabilities.CurrentExtent;
        }
        else
        {
            int width = 0, height = 0;
            Window.Main.sdl.VulkanGetDrawableSize(Window.Main.window, ref width, ref height);
            
            Extent2D extent = new Extent2D()
            {
                Width = (uint)width,
                Height = (uint)height
            };

            extent.Width = Math.Min(Math.Max(extent.Width, supportDetails.SurfaceCapabilities.MinImageExtent.Width),
                supportDetails.SurfaceCapabilities.MaxImageExtent.Width);
            extent.Height = Math.Min(Math.Max(extent.Height, supportDetails.SurfaceCapabilities.MinImageExtent.Height),
                supportDetails.SurfaceCapabilities.MaxImageExtent.Height);
            
            return extent;
        }
    }

    public static unsafe void Create()
    {
        CheckSwapChainSupport();
        surfaceFormat = ChooseSurfaceFormat();
        presentMode = ChoosePresentMode();
        extent = ChooseSwapExtent();
        
        uint imageCount = supportDetails.SurfaceCapabilities.MinImageCount + 1;
        if (supportDetails.SurfaceCapabilities.MaxImageCount > 0 && imageCount > supportDetails.SurfaceCapabilities.MaxImageCount) {
            imageCount = supportDetails.SurfaceCapabilities.MaxImageCount;
        }

        if (VulkanRenderer.QueueFamilyIndices.GraphicsFamily == null || VulkanRenderer.QueueFamilyIndices.PresentFamily == null)
        {
            throw new Exception("Data in VulkanRenderer.QueueFamilyIndices is null");
        }
        
        uint[] queueFamilyIndices = new uint[]
        {
            (uint)VulkanRenderer.QueueFamilyIndices.GraphicsFamily,
            (uint)VulkanRenderer.QueueFamilyIndices.PresentFamily
        };
        fixed (uint* queueFamilyIndicesPtr = queueFamilyIndices.ToArray())
        {
            var imageSharingMode = SharingMode.Exclusive;
            uint queueFamilyIndexCount = 0;
            uint* pQueueFamilyIndices = null;

            if (VulkanRenderer.QueueFamilyIndices.GraphicsFamily != VulkanRenderer.QueueFamilyIndices.PresentFamily)
            {
                imageSharingMode = SharingMode.Concurrent;
                queueFamilyIndexCount = 2;
                pQueueFamilyIndices = queueFamilyIndicesPtr;
            }

            var createInfo = new SwapchainCreateInfoKHR()
            {
                SType = StructureType.SwapchainCreateInfoKhr,
                Surface = VulkanRenderer.SurfaceKHR,
                MinImageCount = imageCount,
                ImageFormat = surfaceFormat.Format,
                ImageColorSpace = surfaceFormat.ColorSpace,
                ImageExtent = extent,
                ImageArrayLayers = 1,
                ImageUsage = ImageUsageFlags.ColorAttachmentBit,
                ImageSharingMode = imageSharingMode,
                QueueFamilyIndexCount = queueFamilyIndexCount,
                PQueueFamilyIndices = pQueueFamilyIndices,
                PreTransform = supportDetails.SurfaceCapabilities.CurrentTransform,
                CompositeAlpha = CompositeAlphaFlagsKHR.OpaqueBitKhr,
                PresentMode = presentMode,
                // OldSwapchain = new SwapchainKHR() 并没有在尝试重创建swapchain 因此不添加此传参
            };
            
            VulkanRenderer.SwapchainInstance.CreateSwapchain(VulkanRenderer.Device, &createInfo, null, out VulkanRenderer.SwapchainKHR);
            Log.Information("Created Swapchain");

            uint swapChainImageCount = 0;
            VulkanRenderer.SwapchainInstance.GetSwapchainImages(VulkanRenderer.Device, VulkanRenderer.SwapchainKHR, &swapChainImageCount, null);
            Image[] swapChainImages = new Image[swapChainImageCount];
            VulkanRenderer.SwapchainInstance.GetSwapchainImages(VulkanRenderer.Device, VulkanRenderer.SwapchainKHR, &swapChainImageCount, out swapChainImages[0]);
            
            VulkanRenderer.SwapchainImages = new List<Image>(swapChainImages);
            Log.Information("Got Swapchain Images");
        }
    }

    public static unsafe void CleanUp()
    {
        foreach (var swapChainFramebuffer in VulkanRenderer.SwapChainFramebuffers)
        {
            VulkanRenderer.VkApi().DestroyFramebuffer(VulkanRenderer.Device, swapChainFramebuffer, null);
        }
        VulkanRenderer.SwapChainFramebuffers.Clear();
        foreach (var swapchainImageView in VulkanRenderer.SwapchainImageViews)
        {
            VulkanRenderer.VkApi().DestroyImageView(VulkanRenderer.Device, swapchainImageView, null);
        }
        VulkanRenderer.SwapchainImageViews.Clear();
        VulkanRenderer.SwapchainInstance.DestroySwapchain(VulkanRenderer.Device, VulkanRenderer.SwapchainKHR, null);
    }

    public static unsafe void ReCreate()
    {
        // 特殊情况：最小化处理
        var flags = Window.Main.sdl.GetWindowFlags(Window.Main.window);
        Log.Information($"[VkSwapChain.ReCreate] Now Window flag is: {flags}");
        
        Silk.NET.SDL.Event sdlEvent = new Event();
        
        while ((flags & (uint)WindowFlags.Minimized) != 0)
        {
            flags = Window.Main.sdl.GetWindowFlags(Window.Main.window);
            Window.Main.sdl.PollEvent(ref sdlEvent);
        }
        
        VulkanRenderer.VkApi().DeviceWaitIdle(VulkanRenderer.Device);
        CleanUp();
        Create();
        VkImageViews.Create();
        VkFramebuffers.Create();
    }
}