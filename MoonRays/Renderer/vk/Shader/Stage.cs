using System.Text;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk.Shader;

public static class Stage
{
    public static unsafe PipelineShaderStageCreateInfo BuildShaderStageCreateInfo(ShaderModule module, ShaderStageFlags flags)
    {
        fixed (byte* pName = Encoding.UTF8.GetBytes("main"))
        {
            var createInfo = new PipelineShaderStageCreateInfo()
            {
                SType = StructureType.PipelineShaderStageCreateInfo,
                Stage = flags,
                Module = module,
                PName = pName,
            };
            
            return createInfo;
        }
    }
}