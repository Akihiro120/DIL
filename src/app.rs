use ratatui::{
    Frame,
    crossterm::event::KeyCode,
    layout::{Constraint, Direction, Layout},
};

use crate::{
    completed_panel::CompletedPanel,
    description_panel::DescriptionPanel,
    panel_state::PanelState,
    task_panel::{TaskPanel, TaskPanelState},
    task_storage::TaskStorage,
};

pub struct App {
    task_storage: TaskStorage,
    task_panel_state: TaskPanelState,
    comp_panel_state: PanelState,
    desc_panel_state: PanelState,
}

impl App {
    pub fn new() -> std::io::Result<Self> {
        let mut app = Self {
            task_storage: TaskStorage::new(),
            task_panel_state: TaskPanelState::new(true),
            comp_panel_state: PanelState::new(false),
            desc_panel_state: PanelState::new(true),
        };

        app.task_storage.load()?;

        Ok(app)
    }

    pub fn on_key_press(&mut self, key: char) {
        // pending tasks panel focused
        if self.task_panel_state.focused {
            if key == 'k' {
                self.task_panel_state.decrement_selection();
            }
            if key == 'j' {
                self.task_panel_state.increment_selection();
            }
        }

        // completed tasks panel focused
        if self.comp_panel_state.focused {}

        // toggle between
        if key.is_digit(10) {
            if let Some(digit) = key.to_digit(10) {
                if digit == 0 {
                    self.focus_task_panel();
                }
                if digit == 1 {
                    self.focus_comp_panel();
                }
            }
        }
    }

    fn focus_task_panel(&mut self) {
        self.task_panel_state.focused = true;
        self.comp_panel_state.focused = false;
    }

    fn focus_comp_panel(&mut self) {
        self.task_panel_state.focused = false;
        self.comp_panel_state.focused = true;
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
