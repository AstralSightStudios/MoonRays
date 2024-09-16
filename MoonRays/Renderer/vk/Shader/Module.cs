using System.Runtime.InteropServices;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk.Shader;

[StructLayout(LayoutKind.Sequential)]
public struct Vertex
{
    public GlmSharp.vec2 pos;
    public GlmSharp.vec3 color;
}

public static unsafe class VertexTools
{
    public static VertexInputBindingDescription GetBindingDescription()
    {
        return new VertexInputBindingDescription()
        {
            Binding = 0,
            Stride = (uint)sizeof(Vertex),
            InputRate = VertexInputRate.Vertex
        };
    }

    public static List<VertexInputAttributeDescription> GetAttributeDescriptions()
    {
        var descriptions = new List<VertexInputAttributeDescription>();
        descriptions.Add(new VertexInputAttributeDescription()
        {
            Binding = 0,
            Location = 0,
            Format = Format.R32G32B32Sfloat,
            Offset = (uint)Marshal.OffsetOf(typeof(Vertex), "pos"),
        });
        descriptions.Add(new VertexInputAttributeDescription()
        {
            Binding = 0,
            Location = 1,
            Format = Format.R32G32B32Sfloat,
            Offset = (uint)Marshal.OffsetOf(typeof(Vertex), "color"),
        });
        
        return descriptions;
    }
}