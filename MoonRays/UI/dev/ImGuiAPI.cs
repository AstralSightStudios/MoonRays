using Serilog;

namespace MoonRays.UI.dev;

public static class ImGuiAPI
{
    public static void SayHello()
    {
        Log.Information("Hello From ImGui!");
    }
}