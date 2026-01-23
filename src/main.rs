mod app;
mod completed_panel;
mod description_panel;
mod panel_state;
mod task;
mod task_panel;
mod task_storage;

use ratatui::{
    DefaultTerminal,
    crossterm::{
        self,
        event::{Event, KeyCode},
    },
};

use crate::app::App;

fn main() -> std::io::Result<()> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new()?;

    loop {
        terminal.draw(|frame| {
            app.render(frame);
        })?;

        match crossterm::event::read() {
            Ok(event) => match event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Char('q') => break Ok(()),
                    _ => {}
                },
                _ => {}
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
