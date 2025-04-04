use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

// Define the grid size
const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cell(usize, usize); // (x, y)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


impl Direction {
    fn delta(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// Maze is a 2D grid with walls between cells.
// We'll store for each cell which directions have passages (i.e., no wall).
#[derive(Debug)]
struct Maze {
    cells: Vec<Vec<HashSet<Direction>>>, // Each cell has a set of open directions
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![HashSet::new(); height]; width];
        Self { cells }
    }
    //function used to determine whether the next direction is in bounds
    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize
    }
    //function used to create a passage from the current cell to the next cell
    fn add_passage(&mut self, from: Cell, to: Cell, dir: Direction) {
        self.cells[from.0][from.1].insert(dir);
        self.cells[to.0][to.1].insert(dir.opposite());
    }
    //function begins the generation of the board and initializes the backtracker function with the starting location
    fn generate(&mut self) {
        let mut visited = HashSet::new();
        let start = Cell(0, 0);
        self.backtracker(start, &mut visited);
    }
    //back tracker function. this recursively calls itself (dfs) in order to create a maze
    fn backtracker(&mut self, current: Cell, visited: &mut HashSet<Cell>) {
        visited.insert(current);

        let mut rng = thread_rng();
        let mut directions = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        directions.shuffle(&mut rng);

        for dir in directions {
            let (dx, dy) = dir.delta();
            let new_x = current.0 as isize + dx;
            let new_y = current.1 as isize + dy;

            if self.in_bounds(new_x, new_y) {
                let neighbor = Cell(new_x as usize, new_y as usize);
                if !visited.contains(&neighbor) {
                    self.add_passage(current, neighbor, dir);
                    self.backtracker(neighbor, visited);
                }
            }
        }
    }

    // Optional: print the maze to the terminal
}

fn main() {
    let mut maze = Maze::new(WIDTH, HEIGHT);
    maze.generate();
}
