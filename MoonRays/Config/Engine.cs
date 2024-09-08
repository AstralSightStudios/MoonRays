using Serilog;

namespace MoonRays.Config;

public class EngineConfig
{
    public readonly string GameName = "EMPTY";
    public readonly string GameVersion = "EMPTY";
    public readonly WindowSettings WindowSettings = new WindowSettings()
    {
        Width = 800,
        Height = 600
    };
    public readonly RendererSettings RendererSettings = new RendererSettings() {};
}

public class WindowSettings
{
    public int Width;
    public int Height;
}

public class RendererSettings
{
    public readonly List<string> VkEnabledLayers = new List<string>()
    {
        "VK_LAYER_KHRONOS_validation"
    };
    public readonly List<string> VkEnabledExtensions = new List<string>()
    {
        "VK_EXT_debug_utils"
    };
    public readonly List<string> VkDeviceEnabledExtensions = new List<string>()
    {
        "VK_KHR_swapchain"
    };
}

public static class Engine
{
    public static EngineConfig Config = new EngineConfig();

    public static void LoadConfig()
    {
        if (File.Exists("engine_config.json"))
        {
            var readResult = Newtonsoft.Json.JsonConvert.DeserializeObject<EngineConfig>(File.ReadAllText("engine_config.json"));
            if (readResult != null)
            {
                Config = readResult;
            }
            else
            {
                throw new FileNotFoundException("Engine config file not found.");
            }
        }
        else
        {
            Log.Warning("[Load Config] Engine config file is empty, so a default config is used.");
        }
        
        Log.Information("[Load Config] Loaded Engine Config");
    }
}