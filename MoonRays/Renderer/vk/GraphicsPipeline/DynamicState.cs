using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk.GraphicsPipeline;

public static class VkDynamicState
{
    public static unsafe PipelineDynamicStateCreateInfo BuildDynamicStateCreateInfo(List<DynamicState> dynamicStates)
    {
        fixed (DynamicState* dynamicStatePtr = dynamicStates.ToArray())
        {
            var createInfo = new PipelineDynamicStateCreateInfo()
            {
                SType = StructureType.PipelineDynamicStateCreateInfo,
                DynamicStateCount = (uint)dynamicStates.Count,
                PDynamicStates = dynamicStatePtr
            };

            return createInfo;
        }
    }
}