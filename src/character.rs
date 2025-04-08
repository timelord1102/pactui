use crate::colorize;
use crate::ghost;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{style::Color, text::Span};

#[derive(Debug)]
pub struct Character {
    pub x: usize,
    pub y: usize,
    pub character: char,
    direction: char,
    move_time: std::time::Instant,
    pub speed: u128,
    pub speed_boost: bool,
    pub boost_timer: i32,
    pub fruits: String,
    pub lives: i32, // Added lives attribute
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
            lives: 3, // Initialize lives to 3
        }
    }

    pub fn handle_input(
        &mut self,
        key_event: KeyEvent,
        cheat: &mut String,
        ghost: &mut Vec<ghost::Ghost>,
    ) {
        for ghost in ghost.iter_mut() {
            if ghost.direction == 'N' {
                ghost.start_move();
            }
        }
        match key_event.code {
            KeyCode::Left => self.direction = 'L',
            KeyCode::Right => self.direction = 'R',
            KeyCode::Up => self.direction = 'U',
            KeyCode::Down => self.direction = 'D',
            _ => cheat.push(key_event.code.to_string().chars().next().unwrap()),
        }
    }

    pub fn render(&mut self, buf: &mut Vec<Vec<Span<'static>>>, score: &mut i32) {
        if std::time::Instant::now()
            .duration_since(self.move_time)
            .as_millis()
            >= self.speed
        {
            self.move_time = std::time::Instant::now();
            buf[self.y][self.x] = colorize(' ', Color::White);
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

        buf[self.y][self.x] = colorize(self.character, Color::Yellow);
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.character = 'C'; // Reset character to original state
        self.direction = ' ';
        self.move_time = std::time::Instant::now();
        self.speed = 100;
        self.speed_boost = false;
        self.boost_timer = 0;
    }
}
