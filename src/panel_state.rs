use ratatui::style::Style;

pub struct PanelState {
    pub focused: bool,
}

impl PanelState {
    pub fn new(focused: bool) -> PanelState {
        PanelState { focused }
    }

    pub fn get_panel_style(&self) -> Style {
        let style = if self.focused {
            Style::default().white()
        } else {
            Style::default().gray().dim()
        };

        style
    }
}
