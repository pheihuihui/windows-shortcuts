using Microsoft.Extensions.Logging;
using Microsoft.Maui.LifecycleEvents;
using WinShortcuts.Services;
using WinShortcuts.WinUI;

namespace WinShortcuts;

public static class MauiProgram
{
    public static MauiApp CreateMauiApp()
    {
        var builder = MauiApp.CreateBuilder();
        builder
            .UseMauiApp<App>()
            .ConfigureFonts(fonts =>
            {
                fonts.AddFont("OpenSans-Regular.ttf", "OpenSansRegular");
                fonts.AddFont("OpenSans-Semibold.ttf", "OpenSansSemibold");
            });

        builder.ConfigureLifecycleEvents(life =>
        {
            life.AddWindows(windows =>
            {
                windows.OnWindowCreated(del =>
                {
                    del.ExtendsContentIntoTitleBar = true;
                });
            });
        });

        builder.Logging.AddDebug();

        var services = builder.Services;
        services.AddSingleton<ITrayService, TrayService>();

        return builder.Build();
    }
}
