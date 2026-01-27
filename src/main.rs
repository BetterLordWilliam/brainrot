use std::io;
use ratatui::{
    DefaultTerminal, Frame
};

fn app_loop(terminal: &mut DefaultTerminal) -> io::Result<()> {
    loop {
        // App code for the main loop begins in this place
    };

    Ok(())
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let res = app_loop(&mut terminal);
    ratatui::restore();

    res
}
