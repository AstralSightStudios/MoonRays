using Silk.NET.GLFW;
using MoonRays.Config;
using Serilog;

namespace MoonRays.Window;

public static unsafe class Main
{
    public static Glfw glfw;
    public static WindowHandle* window;

    public static void Create()
    {
        glfw = Glfw.GetApi();

        glfw.Init();
        glfw.WindowHint(WindowHintClientApi.ClientApi, ClientApi.NoApi);
        glfw.WindowHint(WindowHintBool.Resizable, false);

        if (Engine.Config == null)
        {
            Log.Error("[CreateWindow] Engine.Config is null");
            throw new NullReferenceException();
        }
        window = glfw.CreateWindow(Engine.Config.WindowSettings.Width, Engine.Config.WindowSettings.Height,
            Engine.Config.GameName, null, null);
        Log.Information("[CreateWindow] Created Window");
    }
}

public static unsafe class Loop
{
    public static void RunLoop()
    {
        Log.Information("[RunLoop] Starting Window Loop");
        while (!Main.glfw.WindowShouldClose(Main.window))
        {
            Main.glfw.PollEvents();
        }
        Log.Information("[RunLoop] Stopping Window Loop");
    }
}