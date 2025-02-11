use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Widget, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    let mut terminal= ratatui::init();
    let app_result = App::new(false, Character::new(0, 0, 'C')).run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct Character {
    x: usize,
    y: usize,
    character: char,
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    character: Character,
}

impl Character {
    pub fn new(x: usize, y: usize, character: char) -> Self {
        Self { x, y, character }
    }

    fn handle_input(&mut self, key_event: KeyEvent, area: Rect) {
        let m = area.width as usize;
        let n = area.height as usize;
        match key_event.code {
            KeyCode::Left => if self.x > 0 {self.x -= 1} else {},
            KeyCode::Right => if self.x < m - 1 {self.x += 1} else {},
            KeyCode::Up => if self.y > 0 {self.y -= 1} else {},
            KeyCode::Down => if self.y < n - 1 {self.y += 1} else {},
            _ => {}
        }
    }

    fn render(&self, buf: &mut Vec<Vec<char>>) {
        if self.y < buf.len() && self.x < buf[0].len() {
            buf[self.y][self.x] = self.character;
        }
    }
}

impl App {
    pub fn new(exit: bool, character: Character) -> Self {
        Self { exit, character }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            let mut frame = terminal.get_frame();
            self.draw(&mut frame);
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_input(key_event, &frame)
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn handle_input(&mut self, key_event: KeyEvent, frame: &Frame) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            _ => self.character.handle_input(key_event, frame.area()),
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        let area = f.area();
        f.render_widget(self, area);
    }
            
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        let mut buffer = vec![vec!['.'; area.width as usize]; area.height as usize];
        self.character.render(&mut buffer);
        let text = buffer.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n");

        Paragraph::new(Text::from(text))
            .block(block)
            .render(area, buf);
    }
}
