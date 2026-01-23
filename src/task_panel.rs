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
        TaskPanelState {
            focused,
            list_state: ListState::default(),
        }
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

        let mut content = List::new(items);

        let style = state.get_panel_style();
        content = content.style(style);
        content = content.block(Block::bordered().title_top("[0] Pending"));
        content = content.highlight_style(Style::default().white().add_modifier(Modifier::BOLD));

        StatefulWidget::render(content, area, buf, &mut state.list_state);
    }
}
