using Terminal.Gui;

namespace DIL
{
    public class Program
    {
        public static void Main(String[] args)
        {
            Application.Init();

            var top = Application.Top;
            var scheme = Theme.GetDefaultScheme();
            var activeScheme = Theme.GetActiveScheme();
            var inactiveScheme = Theme.GetInactiveScheme();
            Colors.ColorSchemes["Base"] = scheme;
            Colors.ColorSchemes["TopLevel"] = scheme;
            Colors.ColorSchemes["Dialog"] = scheme;
            Colors.ColorSchemes["Menu"] = scheme;
            
            var window = new Window("DIL")
            {
                X = 0,
                Y = 0,
                Width = Dim.Fill(),
                Height = Dim.Fill(),
            };
            top.Add(window);

            var taskView = new View("Tasks")
            {
                X = 0,
                Y = 0,
                Width = Dim.Percent(30),
                Height = Dim.Fill(),
            };

            taskView.KeyPress += (e) =>
            {
                if (e.KeyEvent.Key == Key.D1)
                {
                    e.Handled = true;
                }
                if (e.KeyEvent.Key == Key.D2)
                {
                    e.Handled = true;
                }
            };

            var pendingView = new FrameView("[1] Pending")
            {
                X = 0,
                Y = 0,
                Width = Dim.Fill(),
                Height = Dim.Percent(50),
                ColorScheme = activeScheme
            };

            var completedView = new FrameView("[2] Completed")
            {
                X = 0,
                Y = Pos.Bottom(pendingView),
                Width = Dim.Fill(),
                Height = Dim.Fill(),
                ColorScheme = inactiveScheme
            };
            taskView.Add(pendingView, completedView);

            var descriptionView = new FrameView("Description")
            {
                X = Pos.Right(taskView),
                Y = 0,
                Width = Dim.Percent(70),
                Height = Dim.Fill(),
            };
            window.Add(taskView, descriptionView);

            var status = new StatusBar(new []
            {
                new StatusItem(Key.q, "Quit: q", () => Application.RequestStop()),
                new StatusItem(Key.k, "Up: k", () => {}),
                new StatusItem(Key.j, "Down: j", () => {}),
            });
            top.Add(status);

            Application.Run();
            Application.Shutdown();
        }
    }
}
