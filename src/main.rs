use std::io;
use ratatui::{
    DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEvent}, layout::{ Alignment, Rect }, style::Stylize, symbols::border, text::Line, widgets::{ Block, Borders, Paragraph }
};


#[derive(Default)]
struct App {
    quit: bool
}


impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.quit {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    self.handle_key_press(key);
                }
            }

            terminal.draw(|frame| {
                let b = Block::bordered()
                    .title(Line::raw("brainrot").bold())
                    .title_bottom(Line::default().spans(vec![
                        "quit ".blue(),
                        "<q>".blue().bold()
                    ]))
                    .title_alignment(Alignment::Center);
                let p = Paragraph::new("welcome to brainrot")
                    .block(b);

                frame.render_widget(p, frame.area());
            })?;

        };
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {

    }

    fn handle_key_press(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.quit = true;
            },
            _ => {}
        };
    }
}


fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let res = app.run(&mut terminal);

    ratatui::restore();

    res
}
