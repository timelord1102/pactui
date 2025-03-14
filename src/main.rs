use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::Rng;
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
    speed_boost: bool,
    boost_timer: i32,
    fruits: String,
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    character: Character,
    board: Vec<Vec<Span<'static>>>,
    score: i32,
    cheat: String,
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
            speed_boost: false,
            boost_timer: 0,
            fruits: String::new(),
        }
    }

    fn handle_input(&mut self, key_event: KeyEvent, cheat: &mut String) {
        match key_event.code {
            KeyCode::Left => self.direction = 'L',
            KeyCode::Right => self.direction = 'R',
            KeyCode::Up => self.direction = 'U',
            KeyCode::Down => self.direction = 'D',
            _ => cheat.push(key_event.code.to_string().chars().next().unwrap()),
        }
    }

    fn render(&mut self, buf: &mut Vec<Vec<Span<'static>>>, score: &mut i32) {
        if std::time::Instant::now()
            .duration_since(self.move_time)
            .as_millis()
            >= self.speed
        {
            self.move_time = std::time::Instant::now();
            buf[self.y][self.x] = colorize(' ');
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

            if self.speed_boost {
                self.boost_timer -= 1;
                if self.boost_timer == 0 {
                    self.speed = 100;
                    self.speed_boost = false;
                }
            }
        }
        if buf[self.y][self.x].to_string() == "·" {
            *score += 1;
        }
        if buf[self.y][self.x].to_string() == "∞" {
            *score += 100;
            if self.fruits.len() == 0 {
                self.fruits.push(' ');
            }
            self.fruits.push('∞');
            self.fruits.push(' ');
        }

        if buf[self.y][self.x].to_string() == ">" {
            self.speed = 50;
            self.speed_boost = true;
            self.boost_timer += 100;
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
            cheat: String::new(),
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.generate_board(&terminal.get_frame().area());
        while !self.exit {
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

            if self.score == 70 {
                let x = self.board[0].len() / 2;
                let y = self.board.len() / 2;
                self.board[y][x] = colorize('∞');
            }
        }
        Ok(())
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('x') => self.run_cheat(),
            _ => self.character.handle_input(key_event, &mut self.cheat),
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
                    self.board[y][x] = colorize('·');
                }
            }
        }

        for _ in 0..3 {
            let x = rand::rng().random_range(0..self.board[0].len() / 2) * 2;
            let y = rand::rng().random_range(0..self.board.len());
            self.board[y][x] = colorize('>');
        }
    }

    pub fn run_cheat(&mut self) {
        match self.cheat.as_str() {
            "speed" => {
                self.character.speed = 5;
                self.character.speed_boost = true;
                self.character.boost_timer += 1000;
            }
            _ => {}
        }
        self.cheat.clear();
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(vec![
            " Score: ".into(),
            self.score.to_string().into(),
            " ".into(),
        ]);
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

        let lives = Line::from(vec![" C C C ".yellow().bold()]);

        let powerups = Line::from(vec![if self.character.speed_boost
            && (self.character.boost_timer > 10
                || (self.character.boost_timer < 10 && self.character.boost_timer % 2 == 0))
        {
            " > ".light_cyan().bold()
        } else if self.character.speed_boost {
            "   ".into()
        } else {
            "".into()
        }]);

        let fruits = Line::from(
            self.character
                .fruits
                .chars()
                .map(|ch| colorize(ch))
                .collect::<Vec<Span<'static>>>(),
        );

        let block = Block::bordered()
            .title(title.centered())
            .title(lives.left_aligned())
            .title(fruits.right_aligned())
            .title(powerups.left_aligned())
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
        '>' => ch.to_string().light_cyan().bold(),
        '∞' => ch.to_string().light_red().bold(),
        _ => ch.to_string().not_dim(),
    }
}
