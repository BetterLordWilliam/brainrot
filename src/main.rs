use std::io;
use ratatui::{
    DefaultTerminal, Frame
};

fn app_loop(terminal: &mut DefaultTerminal) -> io::Result<()> {
    Ok(())
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let res = app_loop(&mut terminal);
    ratatui::restore();

    res
}
