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
    direction: char,
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    character: Character,
    board: Vec<Vec<char>>,
    score: i32,
}

impl Character {
    pub fn new(x: usize, y: usize, character: char) -> Self {
        Self { x, y, character, direction: 'R' }
    }

    fn handle_input(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => self.direction = 'L',
            KeyCode::Right => self.direction = 'R',
            KeyCode::Up => self.direction = 'U',
            KeyCode::Down => self.direction = 'D',
            _ => {}
        }
    }

    fn render(&mut self, buf: &mut Vec<Vec<char>>, score: &mut i32) {
        match self.direction {
            'L' => self.x = if self.x > 0 { self.x - 1 } else { buf[0].len() - 1 },
            'R' => self.x = if self.x < buf[0].len() - 1 { self.x + 1 } else { 0 },
            'U' => self.y = if self.y > 0 { self.y - 1 } else { buf.len() - 1 },
            'D' => self.y = if self.y < buf.len() - 1 { self.y + 1 } else { 0 },
            _ => {}
        }
        if self.character == 'C' {
            self.character = 'O';
        } else {
            self.character = 'C';
        }
        if buf[self.y][self.x] == '.' {
            *score += 1;
        }
        buf[self.y][self.x] = self.character;
    }
}

impl App {
    pub fn new(exit: bool, character: Character) -> Self {
        Self { exit, character, board: vec![], score: 0 }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.generate_board(&terminal.get_frame().area());
        while !self.exit {
            self.update_board(self.character.x, self.character.y);
            self.character.render(&mut self.board, &mut self.score);
            terminal.draw(|f| {
                self.draw(f);
            })?;
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.kind == KeyEventKind::Press {
                        self.handle_input(key_event);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            _ => self.character.handle_input(key_event),
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        let area = f.area();
        f.render_widget(self, area);
    }

    pub fn generate_board(&mut self, area: &Rect) {
        self.board = vec![vec!['.'; area.width as usize]; area.height as usize];
    }

    pub fn update_board(&mut self, x: usize, y: usize) {
        self.board[y][x] = ' ';
    }
            
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" PacMan ".bold());
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

        let text = self.board.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n");

        Paragraph::new(Text::from(text))
            .block(block)
            .render(area, buf);
    }
}


