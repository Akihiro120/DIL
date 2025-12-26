mod task;

extern crate pancurses;
use crate::task::Tasks;
use pancurses::{Input, endwin, initscr, noecho};

fn main() -> Result<(), std::io::Error> {
    let mut tasks = Tasks::new();
    tasks.load_tasks("tasks.json".to_string())?;

    let window = initscr();
    window.keypad(true);
    noecho();

    loop {
        // render
        for (i, task) in tasks.tasks.iter().enumerate() {
            window.mvprintw(i as i32, 0, task.name.clone());
        }
        window.refresh();

        // input
        match window.getch() {
            Some(Input::Character(char)) if char == 'q' => break,
            None => {}
            _ => {}
        }
    }

    endwin();

    Ok(())
}
