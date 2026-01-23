use crate::panel_state::PanelState;
use ratatui::{
    style::{Color, Style, Stylize},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

pub struct DescriptionPanel {}

impl DescriptionPanel {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for DescriptionPanel {
    type State = PanelState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let mut content = Paragraph::new("Task list content");

        let style = state.get_panel_style();
        content = content.style(style);
        content = content.block(Block::bordered().title_top("Task Description"));

        Widget::render(content, area, buf);
    }
}
