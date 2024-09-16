using System.Text;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk;

public static class AppInfo
{
    public static unsafe ApplicationInfo GetAppInfo()
    {
        fixed(byte* gameNamePtr = Encoding.UTF8.GetBytes(Config.Engine.Config.GameName), engineNamePtr = Encoding.UTF8.GetBytes("MoonRays Engine")){
            return new ApplicationInfo()
            {
                SType = StructureType.ApplicationInfo,
                PApplicationName = gameNamePtr,
                PEngineName = engineNamePtr,
                ApiVersion = new Silk.NET.Core.Version32(1, 2, 0)
            };
        }
    }
}