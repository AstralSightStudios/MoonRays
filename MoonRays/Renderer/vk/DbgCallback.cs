using MoonRays.Tools;
using Serilog;
using Silk.NET.Vulkan;

namespace MoonRays.Renderer.vk;

public static class DbgCallback
{
    public static unsafe uint DebugCallback(
        DebugUtilsMessageSeverityFlagsEXT messageSeverity,
        DebugUtilsMessageTypeFlagsEXT messageType,
        DebugUtilsMessengerCallbackDataEXT* pCallbackData,
        void* pUserData)
    {
        string message = NativeType.BytePtrToString(pCallbackData->PMessage);
        string logMessage = $"[Vulkan Validation Layer] Severity: {messageSeverity}, Type: {messageType}, Message: {message}";
        
        switch (messageSeverity)
        {
            case DebugUtilsMessageSeverityFlagsEXT.InfoBitExt:
                Log.Information(logMessage);
                break;
            case DebugUtilsMessageSeverityFlagsEXT.WarningBitExt:
                Log.Warning(logMessage);
                break;
            case DebugUtilsMessageSeverityFlagsEXT.ErrorBitExt:
                Log.Error(logMessage);
                break;
            case DebugUtilsMessageSeverityFlagsEXT.VerboseBitExt:
                Log.Verbose(logMessage);
                break;
        }

        return 1;
    }

    public static unsafe DebugUtilsMessengerCreateInfoEXT CreateInfoExt()
    {
        var debugCallbackDelegate = new DebugUtilsMessengerCallbackFunctionEXT(DebugCallback);
        var debugCallback = new PfnDebugUtilsMessengerCallbackEXT(debugCallbackDelegate);
        return new DebugUtilsMessengerCreateInfoEXT()
        {
            SType = StructureType.DebugUtilsMessengerCreateInfoExt,
            MessageSeverity = DebugUtilsMessageSeverityFlagsEXT.VerboseBitExt | DebugUtilsMessageSeverityFlagsEXT.ErrorBitExt | DebugUtilsMessageSeverityFlagsEXT.WarningBitExt | DebugUtilsMessageSeverityFlagsEXT.InfoBitExt,
            MessageType = DebugUtilsMessageTypeFlagsEXT.GeneralBitExt | DebugUtilsMessageTypeFlagsEXT.PerformanceBitExt | DebugUtilsMessageTypeFlagsEXT.PerformanceBitExt | DebugUtilsMessageTypeFlagsEXT.ValidationBitExt | DebugUtilsMessageTypeFlagsEXT.DeviceAddressBindingBitExt,
            PfnUserCallback = debugCallback
        };
    }
}