use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    symbols::border::{self},
    widgets::{Block, BorderType, List, ListItem},
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Tasks {
    tasks: Vec<Task>,
}

struct State {
    running: bool,
    tasks: Tasks,
}

impl State {
    pub fn new() -> anyhow::Result<State, std::io::Error> {
        // load the tasks.json file
        let json_data = fs::read_to_string("tasks.json")?;

        Ok(State {
            running: true,
            tasks: serde_json::from_str(json_data.as_str())?,
        })
    }
}

fn render(frame: &mut Frame, state: &State) {
    let task_block = Block::bordered()
        .border_type(BorderType::Thick)
        .title("DIL: Do it Later");

    let edit_block = Block::bordered()
        .border_type(BorderType::Thick)
        .title("Editing");

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    let items: Vec<ListItem> = state
        .tasks
        .tasks
        .iter()
        .map(|task| task.name.clone().into())
        .collect();

    let list = List::new(items).block(task_block);

    frame.render_widget(list, layout[0]);
    frame.render_widget(edit_block, layout[1]);
}

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = State::new()?;

    while state.running {
        terminal.draw(|frame| render(frame, &state))?;

        // input handle
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Esc => state.running = false,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    ratatui::restore();
    Ok(())
}
