use std::io;
use ratatui::{
    DefaultTerminal, buffer::Buffer, layout::Rect
};

#[derive(Default)]
struct App {
    quit: bool
}

// This is a so-called implementation
// This has methods of a struct, among other things
impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.quit {
            terminal.draw(|frame| {
                let block = Block::new();
                frame.render_widget(widget, area);
            });
        };
        Ok(())
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {

    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let res = app.run(&mut terminal);

    ratatui::restore();

    res
}
