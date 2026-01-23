use std::default;

use crate::{panel_state::PanelState, task::Task};
use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

pub struct TaskPanelState {
    pub focused: bool,
    pub list_state: ListState,
}

impl TaskPanelState {
    pub fn new(focused: bool) -> TaskPanelState {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        TaskPanelState {
            focused,
            list_state,
        }
    }

    pub fn get_panel_style(&self) -> Style {
        let style = if self.focused {
            Style::default().white().not_dim()
        } else {
            Style::default().gray().dim()
        };

        style
    }
}

pub struct TaskPanel<'a> {
    tasks: &'a [Task],
}

impl<'a> TaskPanel<'a> {
    pub fn new(tasks: &'a [Task]) -> Self {
        Self { tasks }
    }
}

impl<'a> StatefulWidget for TaskPanel<'a> {
    type State = TaskPanelState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let items: Vec<ListItem> = self
            .tasks
            .iter()
            .map(|task| ListItem::new(task.name.as_str()))
            .collect();

        let style = state.get_panel_style();
        let content = List::new(items)
            .style(Style::default().gray().dim())
            .highlight_style(
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)
                    .not_dim()
                    .add_modifier(Modifier::BOLD)
                    .remove_modifier(Modifier::DIM),
            )
            .block(
                Block::bordered()
                    .title_top("[0] Pending")
                    .border_style(style),
            );

        StatefulWidget::render(content, area, buf, &mut state.list_state);
    }
}
