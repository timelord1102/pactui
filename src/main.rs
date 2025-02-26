use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new(false, Character::new(0, 0, 'C')).run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct Character {
    x: usize,
    y: usize,
    character: char,
    direction: char,
    move_time: std::time::Instant,
    speed: u128,
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    character: Character,
    board: Vec<Vec<Span<'static>>>,
    score: i32,
}

impl Character {
    pub fn new(x: usize, y: usize, character: char) -> Self {
        Self {
            x,
            y,
            character,
            direction: ' ',
            move_time: std::time::Instant::now(),
            speed: 100,
        }
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

    fn render(&mut self, buf: &mut Vec<Vec<Span<'static>>>, score: &mut i32) {
        if std::time::Instant::now()
            .duration_since(self.move_time)
            .as_millis()
            >= self.speed
        {
            self.move_time = std::time::Instant::now();
            match self.direction {
                'L' => {
                    self.x = if self.x > 0 {
                        self.x - 2
                    } else {
                        buf[0].len() - 2
                    }
                }
                'R' => {
                    self.x = if self.x < buf[0].len() - 2 {
                        self.x + 2
                    } else {
                        0
                    }
                }
                'U' => {
                    self.y = if self.y > 0 {
                        self.y - 1
                    } else {
                        buf.len() - 1
                    }
                }
                'D' => {
                    self.y = if self.y < buf.len() - 1 {
                        self.y + 1
                    } else {
                        0
                    }
                }
                _ => {}
            }
            if self.character == 'C' && self.direction != ' ' {
                self.character = 'O';
            } else if self.character == 'O' && self.direction != ' ' {
                self.character = 'C';
            }
        }
        if buf[self.y][self.x].to_string() == "•" {
            *score += 1;
        }
        buf[self.y][self.x] = colorize(self.character);
    }
}

impl App {
    pub fn new(exit: bool, character: Character) -> Self {
        Self {
            exit,
            character,
            board: vec![],
            score: 0,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.generate_board(&terminal.get_frame().area());
        while !self.exit {
            self.update_board(self.character.x, self.character.y);
            self.character.render(&mut self.board, &mut self.score);
            terminal.draw(|f| {
                self.draw(f);
            })?;
            if event::poll(std::time::Duration::from_millis(0))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.kind == KeyEventKind::Press {
                        self.handle_input(key_event);
                        while event::poll(std::time::Duration::from_millis(0))? {
                            event::read()?;
                        }
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
        self.board =
            vec![vec![colorize(' '); (area.width - 1) as usize]; (area.height - 2) as usize];
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                if x % 2 == 0 {
                    self.board[y][x] = colorize('•');
                }
            }
        }
    }

    pub fn update_board(&mut self, x: usize, y: usize) {
        self.board[y][x] = colorize(' ');
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from((" Score: ".to_owned() + &self.score.to_string() + " ").bold());
        let instructions = Line::from(vec![
            " Left ".into(),
            "<Left>".blue().bold(),
            " Right ".into(),
            "<Right>".blue().bold(),
            " Up ".into(),
            "<Up>".blue().bold(),
            " Down ".into(),
            "<Down>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let text: Vec<Line> = self
            .board
            .iter()
            .map(|row| {
                Line::from(
                    row.iter()
                        .map(|ch| ch.clone())
                        .collect::<Vec<Span<'static>>>(),
                )
            })
            .collect();

        Paragraph::new(Text::from(text))
            .block(block)
            .render(area, buf);
    }
}

pub fn colorize(ch: char) -> Span<'static> {
    match ch {
        'C' | 'O' => ch.to_string().yellow().bold(),
        _ => ch.to_string().white(),
    }
}
