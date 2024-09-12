using Serilog;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk.GraphicsPipeline;

public static class VkPipeline
{
    public static PipelineLayout PipelineLayout;

    private static unsafe void SetupPipelineLayout()
    {
        var createInfo = new PipelineLayoutCreateInfo()
        {
            SType = StructureType.PipelineLayoutCreateInfo,
        };
        VulkanRenderer.VkApi().CreatePipelineLayout(VulkanRenderer.Device, &createInfo, null, out PipelineLayout);
    }
    
    public static unsafe void Create()
    {
        var vertShader = Shader.Loader.LoadShader("testvert");
        var fragShader = Shader.Loader.LoadShader("testfrag");
        var shaderStageCreateInfos = new List<PipelineShaderStageCreateInfo>();
        shaderStageCreateInfos.Add(Shader.Stage.BuildShaderStageCreateInfo(vertShader, ShaderStageFlags.VertexBit));
        shaderStageCreateInfos.Add(Shader.Stage.BuildShaderStageCreateInfo(fragShader, ShaderStageFlags.FragmentBit));
        
        List<DynamicState> dynamicStates = new List<DynamicState>()
        {
            DynamicState.Viewport,
            DynamicState.Scissor
        };
        var dynamicStateCreateInfo = VkDynamicState.BuildDynamicStateCreateInfo(dynamicStates);
        var vertexInputStateCreateInfo = VkVertexInput.BuildVertexInputStateCreateInfo();
        var inputAssemblyStateCreateInfo = new PipelineInputAssemblyStateCreateInfo()
        {
            SType = StructureType.PipelineInputAssemblyStateCreateInfo,
            Topology = PrimitiveTopology.TriangleList,
            PrimitiveRestartEnable = false
        };
        VkViewPort.Create();
        var viewPortState = VkViewPort.BuildViewportStateCreateInfo();
        var rasterizerInfo = VkRasterizer.BuildRasterizationStateCreateInfo();
        var multiSamplingInfo = VkMultisampling.BuildMultisampleStateCreateInfo();
        var colorBlendingStateInfo = VkColorBlend.BuildStateCreateInfo();
        
        SetupPipelineLayout();
        
        VkRenderPass.Create();

        fixed (PipelineShaderStageCreateInfo* shaderStageCreateInfosPtr = shaderStageCreateInfos.ToArray())
        {
            var createInfo = new GraphicsPipelineCreateInfo()
            {
                SType = StructureType.GraphicsPipelineCreateInfo,
                StageCount = (uint)shaderStageCreateInfos.Count,
                PStages = shaderStageCreateInfosPtr,
                PVertexInputState = &vertexInputStateCreateInfo,
                PInputAssemblyState = &inputAssemblyStateCreateInfo,
                PViewportState = &viewPortState,
                PRasterizationState = &rasterizerInfo,
                PMultisampleState = &multiSamplingInfo,
                PDepthStencilState = null,
                PColorBlendState = &colorBlendingStateInfo,
                PDynamicState = &dynamicStateCreateInfo,
                Layout = PipelineLayout,
                RenderPass = VulkanRenderer.RenderPass,
                Subpass = 0
            };
            
            VulkanRenderer.VkApi().CreateGraphicsPipelines(VulkanRenderer.Device, new PipelineCache(), 1, &createInfo, null, out VulkanRenderer.GraphicsPipeline);
            Log.Information("Created Graphics Pipeline");
        }
    }
}