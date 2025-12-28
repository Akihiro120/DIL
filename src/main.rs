mod guard;
mod task;

extern crate pancurses;
use crate::{guard::TerminalGuard, task::Tasks};
use pancurses::{Input, cbreak, init_pair, initscr, noecho, start_color};

fn main() -> Result<(), std::io::Error> {
    let _guard = TerminalGuard {};

    let mut tasks = Tasks::new();
    tasks.load_tasks()?;

    let window = initscr();
    cbreak();
    noecho();
    window.keypad(true);

    start_color();
    init_pair(0, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
    init_pair(1, pancurses::COLOR_YELLOW, pancurses::COLOR_BLACK);

    let mut task_selection = 0;
    loop {
        // render
        window.clear();
        for (i, task) in tasks.tasks.iter().enumerate() {
            if i == task_selection {
                window.color_set(1);
            } else {
                window.color_set(0);
            }
            window.mvprintw(i as i32, 0, &task.name);
        }
        window.refresh();

        // input
        match window.getch() {
            Some(Input::Character('q')) => break,
            Some(Input::Character('k')) => {
                if task_selection > 0 {
                    task_selection -= 1;
                }
            }
            Some(Input::Character('j')) => {
                task_selection =
                    (task_selection + 1).min(tasks.tasks.len() - 1);
            }
            None => {}
            _ => {}
        }
    }

    Ok(())
}
