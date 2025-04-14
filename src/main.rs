use rand::seq::SliceRandom;
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::fmt;

const WIDTH: usize = 27;
const HEIGHT: usize = 30;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Path,
    GhostHouse,
    Portal,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Tile::Wall       => '$',
            Tile::Path       => ' ',
            Tile::GhostHouse => '=',
            Tile::Portal     => 'O',
        };
        write!(f, "{}", symbol)
    }
}

// Define where the portal(s) should go in the half maze (on its left side)
const PORTAL_ROWS: [usize; 1] = [HEIGHT / 2];

fn generate_half_maze(rng: &mut impl Rng) -> Vec<Vec<Tile>> {
    let half_width = (WIDTH + 1) / 2;
    let mut maze = vec![vec![Tile::Wall; half_width]; HEIGHT];
    
    // Start maze carving from (1,1)
    let mut frontier = vec![(1isize, 1isize)];
    maze[1][1] = Tile::Path;
    
    let directions = [(0, 2), (2, 0), (0, -2), (-2, 0)];
    
    while let Some(&(x, y)) = frontier.last() {
        let mut neighbors = Vec::new();
        for &(dx, dy) in &directions {
            let nx = x + dx;
            let ny = y + dy;
            if nx > 0 && ny > 0 && (nx as usize) < half_width && (ny as usize) < HEIGHT - 1 {
                if maze[ny as usize][nx as usize] == Tile::Wall {
                    neighbors.push((nx, ny, dx, dy));
                }
            }
        }
        if neighbors.is_empty() {
            frontier.pop();
            continue;
        }
        let &(nx, ny, dx, dy) = neighbors.choose(rng).unwrap();
        let mx = x + dx / 2;
        let my = y + dy / 2;
        maze[my as usize][mx as usize] = Tile::Path;
        maze[ny as usize][nx as usize] = Tile::Path;
        frontier.push((nx, ny));
    }
    
    let gh_top = HEIGHT / 2 - 1;
    let gh_bottom = HEIGHT / 2 + 1;
    let gh_left = half_width / 4;
    let gh_right = half_width / 4 + 3;
    for y in gh_top..=gh_bottom {
        for x in gh_left..=gh_right {
            if x < half_width {
                maze[y][x] = Tile::GhostHouse;
            }
        }
    }
    
    // Add portal(s) on the left side of the half maze.
    for &row in &PORTAL_ROWS {
        maze[row][0] = Tile::Portal;
    }

    // Minimize dead ends by selectively removing walls
    let num_dead_end_removals = rng.gen_range(3..7);
    for _ in 0..num_dead_end_removals {
        let x = rng.gen_range(1..half_width - 1);
        let y = rng.gen_range(1..HEIGHT - 1);
        if maze[y][x] == Tile::Wall {
            // Check if removing this wall creates a valid path connection
            let mut path_neighbors = 0;
            for &(dx, dy) in &directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx > 0 && ny > 0 && (nx as usize) < half_width && (ny as usize) < HEIGHT {
                    if maze[ny as usize][nx as usize] == Tile::Path {
                        path_neighbors += 1;
                    }
                }
            }
            if path_neighbors == 1 { // Prevent excessive connections
                maze[y][x] = Tile::Path;
            }
        }
    }

    maze
}

// Mirror the half maze horizontally to produce a full maze.
fn mirror_maze(half: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let half_width = half[0].len();
    let mut full = vec![vec![Tile::Wall; WIDTH]; HEIGHT];
    for y in 0..HEIGHT {
        for x in 0..half_width {
            full[y][x] = half[y][x];
            // Mirror to the right side.
            full[y][WIDTH - 1 - x] = half[y][x];
        }
    }
    full
}

fn print_maze(maze: &[Vec<Tile>]) {
    for row in maze {
        for &tile in row {
            print!("{}", tile);
        }
        println!();
    }
}

fn main() {
    let mut rng = StdRng::from_entropy();
    let half = generate_half_maze(&mut rng);
    let full = mirror_maze(&half);
    print_maze(&full);
}