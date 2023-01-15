using WinShortcuts.NativeWindowing;
using WinShortcuts.Services;

namespace WinShortcuts.WinUI;

public class TrayService : ITrayService
{
    WindowsTrayIcon tray;

    public Action ClickHandler { get; set; }

    public void Initialize()
    {
        tray = new WindowsTrayIcon("Platforms/Windows/windows.ico");
        tray.LeftClick = () =>
        {
            WindowExtensions.BringToFront();
            ClickHandler?.Invoke();
        };
    }
}
