use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::{
    completed_panel::CompletedPanel, description_panel::DescriptionPanel, panel_state::PanelState,
    task_panel::TaskPanel, task_storage::TaskStorage,
};

pub struct App {
    task_storage: TaskStorage,
    task_panel_state: PanelState,
    comp_panel_state: PanelState,
    desc_panel_state: PanelState,
}

impl App {
    pub fn new() -> std::io::Result<Self> {
        let mut app = Self {
            task_storage: TaskStorage::new(),
            task_panel_state: PanelState::new(true),
            comp_panel_state: PanelState::new(false),
            desc_panel_state: PanelState::new(true),
        };

        app.task_storage.load()?;

        Ok(app)
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(frame.area());

        let task_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[0]);

        frame.render_stateful_widget(
            TaskPanel::new(&self.task_storage.tasks),
            task_layout[0],
            &mut self.task_panel_state,
        );
        frame.render_stateful_widget(
            CompletedPanel::new(),
            task_layout[1],
            &mut self.comp_panel_state,
        );

        let desc_layout = main_layout[1];

        frame.render_stateful_widget(
            DescriptionPanel::new(),
            desc_layout,
            &mut self.desc_panel_state,
        );
    }
}
