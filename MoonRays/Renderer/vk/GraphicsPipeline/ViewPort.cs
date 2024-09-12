using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk.GraphicsPipeline;

public static class VkViewPort
{
    public static Viewport Viewport;
    public static Rect2D Scissor;
    public static void Create()
    {
        Viewport = new Viewport()
        {
            X = 0.0f,
            Y = 0.0f,
            Width = VkSwapChain.extent.Width,
            Height = VkSwapChain.extent.Height,
            MinDepth = 0.0f,
            MaxDepth = 1.0f,
        };
        Scissor = new Rect2D()
        {
            Offset = new Offset2D(){ X = 0, Y = 0 },
            Extent = VkSwapChain.extent
        };
    }

    public static unsafe PipelineViewportStateCreateInfo BuildViewportStateCreateInfo()
    {
        var viewPorts = new []{ Viewport };
        var scissors = new[]{ Scissor };
        
        fixed (Viewport* pViewPorts = viewPorts)
        fixed (Rect2D* pScissors = scissors)
        {
            return new PipelineViewportStateCreateInfo()
            {
                SType = StructureType.PipelineViewportStateCreateInfo,
                PViewports = pViewPorts,
                PScissors = pScissors,
                ViewportCount = 1,
                ScissorCount = 1
            };
        }
    }
}