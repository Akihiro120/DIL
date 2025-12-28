use pancurses::endwin;

pub struct TerminalGuard {}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        endwin();
        println!("ending session...");
    }
}
