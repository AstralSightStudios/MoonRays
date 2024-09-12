using MoonRays.Config;
using Serilog;
using Silk.NET.SDL;

namespace MoonRays.Window;

public static unsafe class Main
{
    public static Sdl sdl;
    public static Silk.NET.SDL.Window* window;

    public static void Create()
    {
        sdl = Sdl.GetApi();

        sdl.Init(Sdl.InitVideo);
        
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
        
        sdl.SetWindowResizable(window, SdlBool.False);
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
        }
        
        Log.Information("[RunLoop] Stopping Window Loop");
    }
}