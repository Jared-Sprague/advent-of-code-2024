//! A solution to day 6 year 2024.
//! https://adventofcode.com/2024/day/6

type Model = Game;
type Answer = usize;

trait Actor {
    fn draw(&self);
    fn update(&mut self);
}

pub struct Game {
    tick_ms: Option<u32>,
    grid: GameGrid,
    guard: Guard,
}

impl Actor for Game {
    fn draw(&self) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }
}

impl From<String> for Game {
    fn from(input: String) -> Self {
        let input = input.trim().to_string();
        let mut guard = Guard::new();

        let lines: Vec<&str> = input.split("\n").collect();
        let width = lines[0].len();
        let height = lines.len();

        let mut grid = GameGrid::new(width, height);

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '.' => grid.set_space(x, y, GridSpace::Open),
                    '#' => grid.set_space(x, y, GridSpace::Obstructed),
                    '^' => guard.set_position(x as i32, y as i32),
                    _ => panic!("invalid input"),
                }
            }
        }

        Game {
            tick_ms: None,
            grid,
            guard,
        }
    }
}

impl Game {}

struct GameGrid {
    grid: Vec<Vec<GridSpace>>,
}

impl Actor for GameGrid {
    fn draw(&self) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }
}

impl GameGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![GridSpace::Open; width]; height];
        GameGrid { grid }
    }

    pub fn set_space(&mut self, x: usize, y: usize, space: GridSpace) {
        self.grid[y][x] = space;
    }
}

struct Guard {
    position: Position,
    direction: Direction,
    traveled_path: Vec<Position>,
}

impl Actor for Guard {
    fn draw(&self) {
        todo!()
    }

    fn update(&mut self) {
        // check the next space in current direction
        // if obstructed, change direction
        // else move_direction()

        todo!()
    }
}

impl Guard {
    pub fn new() -> Self {
        Guard {
            position: Position { x: 0, y: 0 },
            direction: Direction::Up,
            traveled_path: vec![],
        }
    }

    pub fn is_on_grid(&self) -> bool {
        todo!()
    }

    pub fn move_direction(&mut self) {}

    fn set_position(&mut self, x: i32, y: i32) {
        self.position.x = x;
        self.position.y = y;
    }
}

struct Position {
    x: i32,
    y: i32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug, Clone)]
enum GridSpace {
    Obstructed,
    Open,
}

pub fn parse(input: String) -> Model {
    Game::from(input)
}

pub fn part1(model: Model) -> Answer {
    0
}

pub fn part2(model: Model) -> Answer {
    0
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
                vec![GridSpace::Open, GridSpace::Open, GridSpace::Obstructed],
                vec![GridSpace::Open, GridSpace::Open, GridSpace::Open],
                vec![
                    GridSpace::Obstructed,
                    GridSpace::Obstructed,
                    GridSpace::Open
                ],
            ]
        );

        // Verify guard's initial position
        assert_eq!(game.guard.position.x, 1); // Guard is at column 1
        assert_eq!(game.guard.position.y, 1); // Guard is at row 1
    }
}
