using System.Diagnostics;
using Serilog;
using Serilog.Core;
using Serilog.Events;
using Serilog.Sinks.SystemConsole.Themes;

namespace MoonRays.Logger;

public static class Configuration
{
    public static void Init()
    {
        var customTheme = new AnsiConsoleTheme(new Dictionary<ConsoleThemeStyle, string>
        {
            [ConsoleThemeStyle.Text] = "\x1b[37m",
            [ConsoleThemeStyle.LevelInformation] = "\x1b[32m",
            [ConsoleThemeStyle.LevelWarning] = "\x1b[33m",
            [ConsoleThemeStyle.LevelError] = "\x1b[31m",
        });
        
        Log.Logger = new LoggerConfiguration()
            .MinimumLevel.Debug()
            .WriteTo.Console(
                outputTemplate: "[{Timestamp:yyyy-MM-dd HH:mm:ss} {Level}] {Message}{NewLine}{Exception}",
                theme: customTheme)
            .WriteTo.File("logs/log.txt", rollingInterval: RollingInterval.Day,
                outputTemplate: "[{Timestamp:yyyy-MM-dd HH:mm:ss.fff zzz} {Level}] [{SourceContext}] {Message}{NewLine}{Exception}")
            .CreateLogger();
    }
}