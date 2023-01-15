using WinShortcuts.Services;
using ServiceProvider = WinShortcuts.Services.ServiceProvider;

namespace WinShortcuts;

public partial class MainPage : ContentPage
{
	int count = 0;

	public MainPage()
	{
		InitializeComponent();

		SetupTrayIcon();
	}

	private void OnCounterClicked(object sender, EventArgs e)
	{
		count++;

		if (count == 1)
			CounterBtn.Text = $"Clicked {count} time";
		else
			CounterBtn.Text = $"Clicked {count} times";

		SemanticScreenReader.Announce(CounterBtn.Text);
	}

	private void SetupTrayIcon()
	{
		var trayService = ServiceProvider.GetService<ITrayService>();

		if (trayService != null)
		{
			trayService.Initialize();
			trayService.ClickHandler = () =>
			{
				DisplayAlert("aaa", "ccc", "sss");
			};

		}
	}
}

