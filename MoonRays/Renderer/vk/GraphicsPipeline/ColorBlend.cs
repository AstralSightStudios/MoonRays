using Silk.NET.Core;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk.GraphicsPipeline;

public static class VkColorBlend
{
    public static PipelineColorBlendAttachmentState BuildAttachmentState()
    {
        return new PipelineColorBlendAttachmentState()
        {
            ColorWriteMask = ColorComponentFlags.RBit | ColorComponentFlags.GBit | ColorComponentFlags.BBit |
                             ColorComponentFlags.ABit,
            BlendEnable = new Bool32(false),
            SrcColorBlendFactor = BlendFactor.SrcAlpha,
            DstColorBlendFactor = BlendFactor.OneMinusSrcAlpha,
            ColorBlendOp = BlendOp.Add,
            SrcAlphaBlendFactor = BlendFactor.One,
            DstAlphaBlendFactor = BlendFactor.Zero,
            AlphaBlendOp = BlendOp.Add
        };
    }

    public static unsafe PipelineColorBlendStateCreateInfo BuildStateCreateInfo()
    {
        var attachment = BuildAttachmentState();
        return new PipelineColorBlendStateCreateInfo()
        {
            SType = StructureType.PipelineColorBlendStateCreateInfo,
            LogicOpEnable = false,
            LogicOp = LogicOp.Copy,
            AttachmentCount = 1,
            PAttachments = &attachment,
        };
    }
}