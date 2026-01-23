use crate::{panel_state::PanelState, task::Task};
use ratatui::{
    style::{Color, Style, Stylize},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

pub struct TaskPanel<'a> {
    tasks: &'a Vec<Task>,
}

impl<'a> TaskPanel<'a> {
    pub fn new(tasks: &'a Vec<Task>) -> Self {
        Self { tasks }
    }
}

impl<'a> StatefulWidget for TaskPanel<'a> {
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
        content = content.block(Block::bordered().title_top("[0] Pending"));

        Widget::render(content, area, buf);
    }
}
