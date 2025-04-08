use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
mod character;
mod ghost;
use rand::Rng;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new(false, character::Character::new(0, 0, 'C')).run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    character: character::Character,
    ghosts: Vec<ghost::Ghost>,
    board: Vec<Vec<Span<'static>>>,
    score: i32,
    cheat: String,
}

impl App {
    pub fn new(exit: bool, character: character::Character) -> Self {
        Self {
            exit,
            character,
            ghosts: vec![],
            board: vec![],
            score: 0,
            cheat: String::new(),
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.generate_board(&terminal.get_frame().area());
        self.ghosts
            .push(ghost::Ghost::new('N', '3', "Pinky".to_string(), (0, 0)));

        for i in 0..self.ghosts.len() {
            let mut x = self.board[0].len() / 2;
            if x % 2 != 0 {
                x -= 1; // Ensure x is even for proper placement
            }
            self.ghosts[i].set_start(x, self.board.len() / 2 + i);
        }
        let res = self.main_loop(terminal);
        if let Err(e) = res {
            eprintln!("Error during game loop: {:?}", e);
            self.exit = true; // Ensure we exit on error
        }
        Ok(())
    }

    fn main_loop(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            self.character.render(&mut self.board, &mut self.score);
            let mut lost_life = false;
            for ghost in self.ghosts.iter() {
                if ghost.x == self.character.x && ghost.y == self.character.y {
                    self.character.lives -= 1;
                    if self.character.lives <= 0 {
                        self.exit = true;
                        println!("Game Over! Your final score was: {}", self.score);
                    }
                    lost_life = true;
                    break;
                }
            }

            if lost_life {
                break;
            }
            for ghost in self.ghosts.iter_mut() {
                ghost.move_ghost(&mut self.board, (self.character.x, self.character.y));
            }
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
                let mut x = self.board[0].len() / 2;
                let y = self.board.len() / 2;
                if x % 2 != 0 {
                    x -= 1;
                }
                self.board[y][x] = colorize('∞', Color::Red);
            }
        }
        if self.character.lives <= 0 {
            println!("Game Over! Your final score was: {}", self.score);
        } else {
            if self.exit {
                println!("Exiting game...");
            } else {
                self.reset_game();
                self.main_loop(terminal)?;
            }
        }
        Ok(())
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('x') => self.run_cheat(),
            _ => self
                .character
                .handle_input(key_event, &mut self.cheat, &mut self.ghosts),
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        let area = f.area();
        f.render_widget(self, area);
    }

    pub fn generate_board(&mut self, area: &Rect) {
        self.board = vec![
            vec![colorize(' ', Color::White); (area.width - 1) as usize];
            (area.height - 2) as usize
        ];
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                if x % 2 == 0 {
                    self.board[y][x] = colorize('·', Color::White);
                }
            }
        }

        for _ in 0..3 {
            let x = rand::rng().random_range(0..self.board[0].len() / 2) * 2;
            let y = rand::rng().random_range(0..self.board.len());
            self.board[y][x] = colorize('>', Color::LightCyan);
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

    fn reset_game(&mut self) {
        self.character.reset();
        for ghost in self.ghosts.iter_mut() {
            ghost.reset(&mut self.board);
        }
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

        let mut lives_string = String::new();

        for i in 0..3 {
            if i < self.character.lives {
                lives_string.push_str("C ");
            } else {
                lives_string.push_str("  "); // Empty hearts for remaining lives
            }
        }

        let lives = Line::from(vec![
            " Lives: ".into(),
            lives_string.trim_end().yellow().bold().into(), // Trim to avoid trailing spaces
        ]);

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
                .map(|ch| colorize(ch, Color::Yellow))
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

pub fn colorize(ch: char, color: Color) -> Span<'static> {
    return Span::styled(ch.to_string(), Style::default().fg(color));
}
