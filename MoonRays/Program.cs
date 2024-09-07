using MoonRays.Renderer;

namespace MoonRays
{
    public static class MoonRaysEntry
    {
        private static void Main(string[] args)
        {
            Logger.Configuration.Init();
            Config.Engine.LoadConfig();
            Window.Main.Create();
            VulkanRenderer.Init();
            Window.Loop.RunLoop();
        }
    }
}