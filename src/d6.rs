//! A solution to day 6 year 2024.
//! https://adventofcode.com/2024/day/6

use std::{collections::HashSet, thread, time::Duration};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::stdout;

use indexmap::IndexSet;

type Model = Game;
type Answer = usize;

trait Actor {
    fn draw(&mut self);
    fn update(&mut self);
}

// static TICK: Option<u64> = Some(20);
// static VISUALIZE: bool = true;

static TICK: Option<u64> = None;
static VISUALIZE: bool = false;

pub struct Game {
    tick_ms: Option<u64>,
    grid: GameGrid,
    guard: Guard,
}

impl Actor for Game {
    fn draw(&mut self) {
        if VISUALIZE {
            self.grid.draw();
            self.guard.draw();
        }
    }

    fn update(&mut self) {
        // check the next grid space in guards direction for obstuction
        let (guard_x, guard_y) = self.guard.get_position_tup();

        // dbg!((guard_x, guard_y));

        let next_space = match self.guard.direction {
            Direction::Up => self.grid.get_space(guard_x, guard_y - 1),
            Direction::Down => self.grid.get_space(guard_x, guard_y + 1),
            Direction::Left => self.grid.get_space(guard_x - 1, guard_y),
            Direction::Right => self.grid.get_space(guard_x + 1, guard_y),
        };

        // dbg!(&next_space);

        match next_space {
            GridSpace::Obstructed(_) => self.guard.turn(),
            GridSpace::Open => self.guard.move_direction(),
            GridSpace::OutOfBounds => {
                self.guard.move_direction();
                self.guard.in_bounds = false
            }
        }

        self.guard.update();
    }
}

impl From<String> for Game {
    fn from(input: String) -> Self {
        let input = input.trim().to_string();
        let mut guard_start_position = Position { x: 0, y: 0 };

        let lines: Vec<&str> = input.split("\n").collect();
        let width = lines[0].len();
        let height = lines.len();

        let mut grid = GameGrid::new(width, height);

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '.' => grid.set_space(x, y, GridSpace::Open),
                    '#' => grid.set_space(x, y, GridSpace::Obstructed(ObsticleType::Wall)),
                    '^' => {
                        guard_start_position = Position {
                            x: x as i32,
                            y: y as i32,
                        }
                    }
                    _ => panic!("invalid input"),
                }
            }
        }

        let guard = Guard::new(guard_start_position.x, guard_start_position.y);

        Game {
            tick_ms: TICK,
            grid,
            guard,
        }
    }
}

impl Game {
    pub fn draw_loop_msg() {
        if VISUALIZE {
            // println!("inloop");
            let mut stdout = stdout();

            // Set the cursor position
            execute!(stdout, MoveTo(4, 5)).unwrap();

            // Set the foreground color to Blue and print text
            execute!(stdout, SetForegroundColor(Color::Magenta), Print("LOOP!"),).unwrap();

            thread::sleep(Duration::from_millis(1500));
        }
    }
}

#[derive(Debug)]
struct GameGrid {
    width: usize,
    height: usize,
    drawn: bool,
    grid: Vec<Vec<GridSpace>>,
}

impl Actor for GameGrid {
    fn draw(&mut self) {
        if !self.drawn {
            // draw grid using crossterm
            let border_color: Color = Color::White;
            let obsticle_color: Color = Color::Yellow;
            let open_color: Color = Color::DarkGrey;
            let mut stdout = stdout();
            let grid_size = 10;

            // Define border dimensions
            let border_width = grid_size + 2; // +2 for the left and right borders
            let border_height = grid_size + 2; // +2 for the top and bottom borders

            // Clear the screen and hide the cursor
            execute!(stdout, Clear(ClearType::All), Hide).unwrap();

            // Draw the border
            for y in 0..border_height {
                for x in 0..border_width {
                    let (char_to_draw, color) = if y == 0 {
                        // Top border
                        if x == 0 {
                            ("┌", border_color) // Top-left corner
                        } else if x == border_width - 1 {
                            ("┐", border_color) // Top-right corner
                        } else {
                            ("─", border_color) // Horizontal line
                        }
                    } else if y == border_height - 1 {
                        // Bottom border
                        if x == 0 {
                            ("└", border_color) // Bottom-left corner
                        } else if x == border_width - 1 {
                            ("┘", border_color) // Bottom-right corner
                        } else {
                            ("─", border_color) // Horizontal line
                        }
                    } else {
                        // Middle rows (sides only)
                        if x == 0 || x == border_width - 1 {
                            ("│", border_color) // Vertical line
                        } else {
                            // Inside the border
                            match self.grid[y - 1][x - 1] {
                                GridSpace::Obstructed(ObsticleType::Wall) => ("#", obsticle_color),
                                GridSpace::Obstructed(ObsticleType::Crate) => ("0", Color::White),
                                GridSpace::Open => (".", open_color),
                                GridSpace::OutOfBounds => panic!("should not be in grid"),
                            }
                        }
                    };

                    // Set the cursor position
                    execute!(stdout, MoveTo(x as u16, y as u16)).unwrap();

                    // Set the foreground color to Blue and print text
                    execute!(stdout, SetForegroundColor(color), Print(char_to_draw),).unwrap();
                }
            }

            self.drawn = true;
        }
    }

    fn update(&mut self) {
        // currently static
    }
}

impl GameGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![GridSpace::Open; width]; height];
        GameGrid {
            drawn: false,
            grid,
            width,
            height,
        }
    }

    pub fn get_space(&self, x: i32, y: i32) -> GridSpace {
        // check for out of bounds
        if x < 0 || y < 0 || (x as usize >= self.width || y as usize >= self.height) {
            return GridSpace::OutOfBounds;
        }

        self.grid[y as usize][x as usize]
    }

    pub fn set_space(&mut self, x: usize, y: usize, space: GridSpace) {
        self.grid[y][x] = space;
    }
}

#[derive(Debug)]
struct Guard {
    start_position: Position,
    position: Position,
    direction: Direction,
    traveled_path: IndexSet<(i32, i32)>,
    in_bounds: bool,
    in_loop: bool,
    hit_obsticles: HashSet<(Position, Direction)>,
    track_path: bool,
}

impl Actor for Guard {
    fn draw(&mut self) {
        let mut stdout = stdout();
        let x = self.position.x as u16;
        let y = self.position.y as u16;
        let direction_char = match self.direction {
            Direction::Up => "^",
            Direction::Down => "V",
            Direction::Left => "<",
            Direction::Right => ">",
        };

        // Set the cursor position
        execute!(stdout, MoveTo(x + 1, y + 1)).unwrap();

        // Set the foreground color to Blue and print text
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(direction_char),
        )
        .unwrap();
    }

    fn update(&mut self) {
        // update guard
    }
}

impl Guard {
    pub fn new(x: i32, y: i32) -> Self {
        let position = Position { x, y };
        let traveled_path = IndexSet::new();

        Guard {
            start_position: position.clone(),
            position: position.clone(),
            direction: Direction::Up,
            traveled_path,
            in_bounds: true,
            in_loop: false,
            hit_obsticles: HashSet::new(),
            track_path: true,
        }
    }

    pub fn move_direction(&mut self) {
        match self.direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }

        if self.track_path {
            self.traveled_path
                .insert((self.position.x, self.position.y));
        }
    }

    fn get_position_tup(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }

    fn reset(&mut self) {
        self.position = self.start_position.clone();
        self.direction = Direction::Up;
        self.traveled_path = IndexSet::new();
        self.in_bounds = true;
        self.in_loop = false;
        self.hit_obsticles = HashSet::new();
    }

    fn turn(&mut self) {
        // println!(
        //     "turn(): current pos: {:?}, current dir: {:?}",
        //     self.position, self.direction
        // );

        // before turning record our position and direction we were going that made us turn for loop detection
        if self
            .hit_obsticles
            .contains(&(self.position.clone(), self.direction.clone()))
        {
            // we've turned at this spot going the same direction once before
            self.in_loop = true;
        } else {
            self.hit_obsticles
                .insert((self.position.clone(), self.direction.clone()));
        }

        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn is_start_position(&self, x: i32, y: i32) -> bool {
        self.start_position.x == x && self.start_position.y == y
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum GridSpace {
    Obstructed(ObsticleType),
    Open,
    OutOfBounds,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum ObsticleType {
    Wall,
    Crate,
}

// make sure cursor is always shown at the end
struct CursorGuard;

impl Drop for CursorGuard {
    fn drop(&mut self) {
        if VISUALIZE {
            let mut stdout = stdout();

            // Ensure the cursor is shown when the program exits
            execute!(stdout, Show).unwrap();

            // move cursor to bottom
            execute!(stdout, MoveTo(0, 12)).unwrap();
        }
    }
}

pub fn parse(input: String) -> Model {
    Game::from(input)
}

pub fn part1(model: Model) -> Answer {
    let mut model = model;

    // Create a guard to ensure the cursor is shown when exiting
    let _guard = CursorGuard;

    // game loop
    while model.guard.in_bounds {
        model.draw();
        model.update();

        if let Some(tick_ms) = model.tick_ms {
            thread::sleep(Duration::from_millis(tick_ms));
        }
    }

    model.guard.traveled_path.len() - 1
}

pub fn part2(model: Model) -> Answer {
    let mut model = model;
    let mut num_positions = 0;

    // Create a guard to ensure the cursor is shown when exiting
    let _guard = CursorGuard;

    // calculate the first path travelled, only new obsticles should go in this path
    while model.guard.in_bounds {
        model.draw();
        model.update();

        if let Some(tick_ms) = model.tick_ms {
            thread::sleep(Duration::from_millis(tick_ms));
        }
    }

    // starting position and remove the last out of bounds position in the path
    let mut traveled_path = model.guard.traveled_path.clone();
    let _ = traveled_path.shift_remove_index(0);
    let _ = traveled_path.pop();

    // reset the guard back to the starting position
    model.guard.reset();
    model.guard.track_path = false; // tracking path takes a lot of resources to do the IndexSet inserts, it's not needed for finding the loops

    for (x, y) in traveled_path {
        // println!("set O: ({}, {})", x, y);

        if model.guard.is_start_position(x, y) {
            // skip the starting position
            continue;
        }

        // place a crate as a new obsticle in the path
        model.grid.set_space(
            x as usize,
            y as usize,
            GridSpace::Obstructed(ObsticleType::Crate),
        );

        while model.guard.in_bounds && !model.guard.in_loop {
            // println!("update");
            model.draw();
            model.update();

            // println!("({}, {})", model.guard.position.x, model.guard.position.y);

            if let Some(tick_ms) = model.tick_ms {
                thread::sleep(Duration::from_millis(tick_ms));
            }
        }

        if model.guard.in_loop {
            Game::draw_loop_msg();
            num_positions += 1;
        }

        model.guard.reset();

        // remove the temporary obsticle
        model
            .grid
            .set_space(x as usize, y as usize, GridSpace::Open);

        model.grid.drawn = false;
    }

    num_positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_from_input() {
        let input = "\
..#
.^.
##.";

        // Convert input into a Game instance
        let game: Game = Game::from(input.to_string());

        // Verify grid initialization
        assert_eq!(
            game.grid.grid,
            vec![
                vec![
                    GridSpace::Open,
                    GridSpace::Open,
                    GridSpace::Obstructed(ObsticleType::Wall)
                ],
                vec![GridSpace::Open, GridSpace::Open, GridSpace::Open],
                vec![
                    GridSpace::Obstructed(ObsticleType::Wall),
                    GridSpace::Obstructed(ObsticleType::Wall),
                    GridSpace::Open
                ],
            ]
        );

        // Verify guard's initial position
        assert_eq!(game.guard.position.x, 1); // Guard is at column 1
        assert_eq!(game.guard.position.y, 1); // Guard is at row 1
    }
}
