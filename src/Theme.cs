using Terminal.Gui;

namespace DIL
{
    public class Theme
    {
        public static ColorScheme GetDefaultScheme()
        {
            var transparentBackground = Color.Black; 

            return new ColorScheme() 
            {
                Normal =    new Terminal.Gui.Attribute(Color.White,         transparentBackground),
                Focus =     new Terminal.Gui.Attribute(Color.Black,         Color.BrightBlue),
                HotNormal = new Terminal.Gui.Attribute(Color.BrightMagenta, transparentBackground),
                HotFocus =  new Terminal.Gui.Attribute(Color.BrightMagenta, Color.BrightBlue),
                Disabled =  new Terminal.Gui.Attribute(Color.Gray,          transparentBackground)
            };        
        }

        public static ColorScheme GetActiveScheme()
        {
            return new ColorScheme()
            {
                Normal =    new Terminal.Gui.Attribute(Color.White,         Color.Black),
                Focus =     new Terminal.Gui.Attribute(Color.Black,         Color.BrightMagenta), 
                HotNormal = new Terminal.Gui.Attribute(Color.BrightMagenta, Color.Black), 
                HotFocus =  new Terminal.Gui.Attribute(Color.BrightMagenta, Color.Black),
            };
        }

        public static ColorScheme GetInactiveScheme()
        {
            return new ColorScheme()
            {
                Normal =    new Terminal.Gui.Attribute(Color.DarkGray,      Color.Black),
                Focus =     new Terminal.Gui.Attribute(Color.DarkGray,      Color.Black),
                HotNormal = new Terminal.Gui.Attribute(Color.DarkGray,      Color.Black),
                HotFocus =  new Terminal.Gui.Attribute(Color.DarkGray,      Color.Black),
            };
        }
    }
}
