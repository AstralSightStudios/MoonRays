using MoonRays.Config;
using MoonRays.Renderer.vk;
using Serilog;
using Silk.NET.SDL;
using Version = Silk.NET.SDL.Version;

namespace MoonRays.Window;

public static unsafe class Main
{
    public static Sdl sdl;
    public static Silk.NET.SDL.Window* window;

    public static void Create()
    {
        sdl = Sdl.GetApi();

        sdl.Init(Sdl.InitVideo);

        Version sdlVersion;
        sdl.GetVersion(&sdlVersion);
        Log.Information($"SDL initialized. Version: {sdlVersion.Major}.{sdlVersion.Minor}.{sdlVersion.Patch}");
        
        if (Engine.Config == null)
        {
            Log.Error("[CreateWindow] Engine.Config is null");
            throw new NullReferenceException();
        }

        byte* placeholder = null;
        if (sdl.VulkanLoadLibrary(placeholder) != 0)
        {
            throw new DllNotFoundException($"[CreateWindow] SDL Load Vulkan library failed, msg: {Tools.NativeType.BytePtrToString(sdl.GetError())}");
        }
        
        window = sdl.CreateWindow(Engine.Config.GameName, Sdl.WindowposCentered, Sdl.WindowposCentered, Engine.Config.WindowSettings.Width, Engine.Config.WindowSettings.Height ,
            (uint)WindowFlags.Vulkan | (uint)WindowFlags.Shown);

        SdlBool resizable = SdlBool.False;
        if (Config.Engine.Config.WindowSettings.Resizable)
        {
            resizable = SdlBool.True;
        }
        sdl.SetWindowResizable(window, resizable);
        Log.Information("[CreateWindow] Created Window");
    }
}

public static unsafe class Loop
{
    public static void RunLoop()
    {
        Log.Information("[RunLoop] Starting Window Loop");

        var shouldClose = false;
        while (!shouldClose)
        {
            Event sdlEvent;
            while (Main.sdl.PollEvent(&sdlEvent) == 1)
            {
                if (sdlEvent.Type == (uint)EventType.Quit)
                {
                    shouldClose = true;
                }
            }
            
            Drawer.DrawFrame();
            MoonRays.Renderer.VulkanRenderer.VkApi().DeviceWaitIdle(MoonRays.Renderer.VulkanRenderer.Device);
        }
        
        Log.Information("[RunLoop] Stopping Window Loop");
    }
}