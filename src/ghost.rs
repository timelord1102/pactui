use crate::colorize;
use ratatui::{style::Color, text::Span};

#[derive(Debug)]
pub struct Ghost {
    pub x: usize,
    pub y: usize,
    pub direction: char,
    pub character: char,
    pub speed: u128,
    pub move_time: std::time::Instant,
    name: String,
    pub target: (usize, usize),
    last_symbol: char,
    start_x: usize,
    start_y: usize,
}

impl Ghost {
    pub fn new(direction: char, character: char, name: String, target: (usize, usize)) -> Self {
        Self {
            x: 0,
            y: 0,
            direction,
            character,
            speed: 100,
            move_time: std::time::Instant::now(),
            name,
            target,
            last_symbol: ' ',
            start_x: 0,
            start_y: 0,
        }
    }

    pub fn move_ghost(&mut self, buf: &mut Vec<Vec<Span<'static>>>, target: (usize, usize)) {
        if std::time::Instant::now()
            .duration_since(self.move_time)
            .as_millis()
            >= self.speed
            && self.direction != 'N'
        {
            self.target = target;
            buf[self.y][self.x] = colorize(self.last_symbol, Color::White);
            match self.direction {
                'L' | 'R' => {
                    if self.y > 0
                        && manhattan_distance(self.x, self.y - 1, self.target.0, self.target.1)
                            < manhattan_distance(self.x, self.y, self.target.0, self.target.1)
                    {
                        self.y -= 1;
                        self.direction = 'U';
                    } else if self.y < buf.len() - 1
                        && manhattan_distance(self.x, self.y + 1, self.target.0, self.target.1)
                            < manhattan_distance(self.x, self.y, self.target.0, self.target.1)
                    {
                        self.y += 1;
                        self.direction = 'D';
                    } else {
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
                            _ => {}
                        }
                    }
                }
                'U' | 'D' => {
                    if self.x > 0
                        && manhattan_distance(self.x - 2, self.y, self.target.0, self.target.1)
                            < manhattan_distance(self.x, self.y, self.target.0, self.target.1)
                    {
                        self.x -= 2;
                        self.direction = 'L';
                    } else if self.x < buf[0].len() - 2
                        && manhattan_distance(self.x + 2, self.y, self.target.0, self.target.1)
                            < manhattan_distance(self.x, self.y, self.target.0, self.target.1)
                    {
                        self.x += 2;
                        self.direction = 'R';
                    } else {
                        match self.direction {
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
                    }
                }
                _ => {}
            }
            self.move_time = std::time::Instant::now();
            if buf[self.y][self.x].to_string() != "C" && buf[self.y][self.x].to_string() != "O" {
                self.last_symbol = buf[self.y][self.x].to_string().chars().next().unwrap();
            }
        }

        buf[self.y][self.x] = colorize_help(self.character, self.name.clone());
    }

    pub fn start_move(&mut self) {
        self.direction = 'U';
    }

    pub fn set_start(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
        self.start_x = x;
        self.start_y = y;
    }

    pub fn reset(&mut self, buf: &mut Vec<Vec<Span<'static>>>) {
        buf[self.y][self.x] = colorize(self.last_symbol, Color::White); // Clear current position
        self.x = self.start_x;
        self.y = self.start_y;
        self.direction = 'N'; // Reset direction to 'N' (not moving)
        self.move_time = std::time::Instant::now(); // Reset move time
        self.last_symbol = ' '; // Clear last symbol
    }
}

pub fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    (x1 as isize - x2 as isize).abs() as usize + (y1 as isize - y2 as isize).abs() as usize
}

fn colorize_help(character: char, name: String) -> Span<'static> {
    if name == "pinky" {
        colorize(character, Color::LightRed)
    } else if name == "inky" {
        colorize(character, Color::LightBlue)
    } else if name == "blinky" {
        colorize(character, Color::LightYellow)
    } else if name == "clyde" {
        colorize(character, Color::LightGreen)
    } else {
        colorize(character, Color::Blue)
    }
}
