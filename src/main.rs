/*use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Borders},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    let mut terminal= ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit{
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());

        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Length(5), Constraint::Min(0)],
        )
        .split(Rect::new(0, 0, 5, 3));
        //frame.render_widget(Paragraph::new("foo"), layout[0]);
        //frame.render_widget(Paragraph::new("bar"), layout[1]);

        frame.render_widget(
            Paragraph::new("Top")
                .block(Block::new().borders(Borders::ALL)),
            layout[0]);
        frame.render_widget(
            Paragraph::new("Bottom")
                .block(Block::new().borders(Borders::ALL)),
            layout[1]);
    }
    
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        if self.counter ==255{
            self.counter+=0;
        }
        else{
            self.counter += 1;
        }
    }

    fn decrement_counter(&mut self) {
        if self.counter ==0{
            self.counter-=0;
        }
        else{
        self.counter -= 1;
        }
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

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);


        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
*/   
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

pub struct Wall {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
}


#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    character: Character,
    board: Vec<Vec<char>>,
    score: i32,
    //wall: Rect,
}

/*impl Wall{
    pub fn new(x: usize, y: usize, height: usize, width: usize) -> Self{
        Self{x ,y ,width ,height}
    }
    pub fn render(&self, buf: Vec<Vec<char>>){
        for i in 0..self.height{
            for j in 0..self.width{
                if self.y + i < buf.len() && self.x + j < buf[0].len(){
                    buf[self.y + i][self.x + j] = '#';
                }
            }
        }
    }
}*/

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
        Self { exit, character, board: vec![], score: 0 ,
        //wall: Rect::new(5,5,2,2)
    }}
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.generate_board(&terminal.get_frame().area());
        while !self.exit {
            self.update_board(self.character.x, self.character.y);
            self.character.render(&mut self.board, &mut self.score);
            //self.wall.render(&mut self.board);
            terminal.draw(|f: &mut Frame<'_>| {
                self.draw(f);
            })?;
            /*terminal.draw(| z: &mut Frame<'_>| {
                self.draw_wall(z);
            })*/
            
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
        //let rect = Rect::new(0,0, 2,2);
        //let rect = Rect::new(self.character., self.character.y,2,2);
        if self.character.x <= u16::MAX as usize && self.character.y <= u16::MAX as usize {
            let convert: u16 = self.character.x as u16;
            let converty: u16 = self.character.y as u16;
            let rect = Rect::new(convert, converty,2,2);
            f.render_widget(self, area);
            f.render_widget(self,rect);
        }
        else{
        f.render_widget(self, area);}
        //f.render_widget(self,rect);
    }

    pub fn generate_board(&mut self, area: &Rect) {
        self.board = vec![vec!['.'; area.width as usize]; area.height as usize];
    }
    /*pub fn draw_wall (&self, z: &mut Frame){
        let rect = Rect::new(0,0, 2,2);
        z.render_widget(self, rect);
    } */

//'.'; area.width as usize
    pub fn update_board(&mut self, x: usize, y: usize) {
        self.board[y][x] = ' ';
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

        let text = self.board.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n");

        Paragraph::new(Text::from(text))
            .block(block)
            .render(area, buf);
    }
    
}

