namespace WinShortcuts.Services;

public static class ServiceProvider
{
    public static TService GetService<TService>()
        => Current.GetService<TService>();

    public static IServiceProvider Current
        => MauiWinUIApplication.Current.Services;

}