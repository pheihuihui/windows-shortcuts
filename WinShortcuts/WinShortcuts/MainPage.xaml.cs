using AdvancedSharpAdbClient;
using Microsoft.Maui.Controls;
using Windows.Media.Protection.PlayReady;

namespace WinShortcuts;

public partial class MainPage : ContentPage
{
    int count = 0;

    static AdbClient client;

    static DeviceData device;

    public MainPage()
    {
        InitializeComponent();
    }

    private void OnCounterClicked(object sender, EventArgs e)
    {
        //count++;

        //if (count == 1)
        //	CounterBtn.Text = $"Clicked {count} time";
        //else
        //	CounterBtn.Text = $"Clicked {count} times";

        //SemanticScreenReader.Announce(CounterBtn.Text);

    }

    private async void OnAdbClicked(object sender, EventArgs e)
    {
        if (!AdbServer.Instance.GetStatus().IsRunning)
        {
            AdbServer server = new AdbServer();
            StartServerResult result = server.StartServer(@"adb.exe", false);
            if (result == StartServerResult.Started)
            {
                await DisplayAlert("Alert", "success", "ok");
            }
            else
            {
                await DisplayAlert("Alert", "failed", "ok");
            }
        }
    }

    private async void OnAdbConnectClicked(object sender, EventArgs e)
    {
        client = new AdbClient();
        client.Connect("192.168.100.167");
        device = client.GetDevices().FirstOrDefault();
        await DisplayAlert("device name", device.Name, "ok");
    }

    private void OnWakeupTvClicked(object sender, EventArgs e)
    {
        client.SendKeyEvent(device, "KEYCODE_WAKEUP");
    }

    private void OnSleepTvClicked(object sender, EventArgs e)
    {
        client.SendKeyEvent(device, "KEYCODE_SLEEP");
    }

}

