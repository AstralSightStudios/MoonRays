using Serilog;
using Silk.NET.Vulkan;

namespace MoonRays.Config;

public class EngineConfig
{
    public readonly string GameName = "MoonRays Engine Development Environment";
    public readonly string GameVersion = "EMPTY";
    public readonly GraphicsSettings GraphicsSettings = new GraphicsSettings();
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

public class GraphicsSettings
{
    public SampleCountFlags MultisampleRasterizationSamples = SampleCountFlags.Count1Bit;
    public SampleCountFlags RenderPassColorSamples = SampleCountFlags.Count1Bit;
}

public static class Engine
{
    private static readonly bool DebugSettingsAlwaysUseDefaultConfig = true;
    public static EngineConfig Config = new EngineConfig();

    public static void LoadConfig()
    {
        if (File.Exists("engine_config.json") && !DebugSettingsAlwaysUseDefaultConfig)
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
            File.WriteAllText("engine_config.json", Newtonsoft.Json.JsonConvert.SerializeObject(Config, Newtonsoft.Json.Formatting.Indented));
            Log.Warning("[Load Config] Engine config file is empty, so a default config is used.");
        }
        
        Log.Information("[Load Config] Loaded Engine Config");
    }
}