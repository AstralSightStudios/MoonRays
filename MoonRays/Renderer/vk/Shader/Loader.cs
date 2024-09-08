using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk.Shader;

public static class Loader
{
    public static unsafe ShaderModule LoadShader(string shaderName)
    {
        var shaderBinary = File.ReadAllBytes($"Shaders/{shaderName}.spirv");
        fixed (byte* shaderBinaryPtr = shaderBinary)
        {
            var createInfo = new ShaderModuleCreateInfo()
            {
                SType = StructureType.ShaderModuleCreateInfo,
                CodeSize = (uint)shaderBinary.Length,
                PCode = (uint*)shaderBinaryPtr
            };
            
            ShaderModule module = new ShaderModule();
            VulkanRenderer.VkApi().CreateShaderModule(VulkanRenderer.Device, &createInfo, null, out module);
            
            return module;
        }
    }
}