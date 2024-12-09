//! A solution to day 8 year 2024.
//! https://adventofcode.com/2024/day/8

use std::collections::{HashMap, HashSet};

use geo::Point;

type Model = CityMap;
type Answer = usize;
type Frequency = char;

#[derive(Debug)]
pub struct CityMap {
    width: u16,
    height: u16,
    antennas: HashMap<Frequency, Vec<Point<i16>>>,
}

impl From<String> for CityMap {
    fn from(input: String) -> Self {
        let input = input.trim();
        let mut lines: Vec<&str> = input.split("\n").collect();
        let mut antennas: HashMap<Frequency, Vec<Point<i16>>> = HashMap::new();
        let width = lines[0].len() as u16;
        let height = lines.len() as u16;

        lines.reverse();

        for (y, line) in lines.iter().enumerate() {
            for (x, freq) in line.chars().enumerate() {
                if freq != '.' {
                    let point = Point::new(x as i16, y as i16);

                    // add this antenna location for it's frequency
                    if let Some(frequency) = antennas.get_mut(&freq) {
                        frequency.push(point);
                    } else {
                        // add this frequency and it's first point
                        antennas.insert(freq, vec![point]);
                    }
                }
            }
        }

        CityMap {
            width,
            height,
            antennas,
        }
    }
}

impl CityMap {
    /// is a given point on the city map bounds
    pub fn is_point_in_bounds(&self, point: Point<i16>) -> bool {
        point.x() >= 0
            && point.y() >= 0
            && point.x() < self.width as i16
            && point.y() < self.height as i16
    }

    /// finds all the unique coordinate pairs of antennas on the map
    pub fn find_unique_antenna_lines(&self) -> HashMap<Frequency, Vec<AntennaLine>> {
        let mut antenna_lines: HashMap<Frequency, Vec<AntennaLine>> = HashMap::new();

        // for each antenna generate it's lines
        self.antennas.iter().for_each(|a| {
            let unique_pairs = find_unique_point_pairs(a.1);
            let freq = *a.0;

            // generate the lines based on the unique pairs for this antenna freq
            let mut lines: Vec<AntennaLine> = vec![];
            unique_pairs.iter().for_each(|p| {
                let line = AntennaLine::new(self.width, self.height, freq, p.0, p.1);
                lines.push(line);
            });

            antenna_lines.insert(freq, lines);
        });

        antenna_lines
    }
}

/// given a vec of points return all the possible uniue pairs
fn find_unique_point_pairs(points: &[Point<i16>]) -> HashSet<(Point<i16>, Point<i16>)> {
    let mut pairs: HashSet<(Point<i16>, Point<i16>)> = HashSet::new();

    // find all the unique pair tuples, note (p1, p2) is the same pair as (p2, p1)
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            if pairs.contains(&(p1, p2)) || pairs.contains(&(p2, p1)) {
                continue;
            } else {
                pairs.insert((p1, p2));
            }
        }
    }

    pairs
}

#[derive(Debug, PartialEq)]
pub enum Slope {
    Up,
    Down,
    Vertical,
    Horizontal,
}

#[derive(Debug)]
pub struct AntennaLine {
    width: u16,
    height: u16,

    #[allow(unused)]
    freq: Frequency,
    slope: Slope,
    /// absolute value of x and y steps to take in either direction or any slope to get the next point on a line
    step: Point<u16>,
    p1: Point<i16>,
    p2: Point<i16>,

    // TODO: refactor this into a Vec of anti points because their could be many on either end
    anti1: Point<i16>,
    anti2: Point<i16>,
}

impl AntennaLine {
    pub fn new(width: u16, height: u16, freq: Frequency, p1: Point<i16>, p2: Point<i16>) -> Self {
        // calculate the sloop
        let slope_num = calc_slope(p1, p2);

        let slope = if p1.y() == p2.y() {
            Slope::Horizontal
        } else if p1.x() == p2.x() {
            Slope::Vertical
        } else if slope_num > 0.0 {
            Slope::Up
        } else {
            Slope::Down
        };

        // calculate the step
        let step_x = p1.x().abs_diff(p2.x());
        let step_y = p1.y().abs_diff(p2.y());
        let step = Point::new(step_x, step_y);

        // calculate the anti-points, for Part 1
        let (anti1, anti2) = match slope {
            Slope::Up => {
                let (left_point, right_point) = if p1.x() < p2.x() { (p1, p2) } else { (p2, p1) };

                let a1_x = left_point.x() - step.x() as i16;
                let a1_y = left_point.y() - step.y() as i16;
                let a2_x = right_point.x() + step.x() as i16;
                let a2_y = right_point.y() + step.y() as i16;
                (Point::new(a1_x, a1_y), Point::new(a2_x, a2_y))
            }
            Slope::Down => {
                let (left_point, right_point) = if p1.x() < p2.x() { (p1, p2) } else { (p2, p1) };
                let a1_x = left_point.x() - step.x() as i16;
                let a1_y = left_point.y() + step.y() as i16;
                let a2_x = right_point.x() + step.x() as i16;
                let a2_y = right_point.y() - step.y() as i16;
                (Point::new(a1_x, a1_y), Point::new(a2_x, a2_y))
            }
            Slope::Vertical => {
                let (top_point, bottom_point) = if p1.y() > p2.y() { (p1, p2) } else { (p2, p1) };
                let a1_x = top_point.x(); // p1 and p2 are the same x
                let a1_y = top_point.y() + step.y() as i16;
                let a2_y = bottom_point.y() - step.y() as i16;
                (Point::new(a1_x, a1_y), Point::new(a1_x, a2_y))
            }
            Slope::Horizontal => {
                let (left_point, right_point) = if p1.x() < p2.x() { (p1, p2) } else { (p2, p1) };
                let a1_y = left_point.y(); // p1 and p2 are the same y
                let a1_x = left_point.x() - step.x() as i16;
                let a2_x = right_point.x() + step.x() as i16;
                (Point::new(a1_x, a1_y), Point::new(a2_x, a1_y))
            }
        };

        AntennaLine {
            width,
            height,
            freq,
            slope,
            step,
            p1,
            p2,
            anti1,
            anti2,
        }
    }

    /// return the left hand point of a line that is not vertical
    pub fn get_left_point(&self) -> Option<Point<i16>> {
        if self.slope == Slope::Vertical {
            return None;
        }

        Some(if self.p1.x() < self.p2.x() {
            self.p1
        } else {
            self.p2
        })
    }

    /// return the right hand point of a line that is not vertical
    pub fn get_right_point(&self) -> Option<Point<i16>> {
        if self.slope == Slope::Vertical {
            return None;
        }

        Some(if self.p1.x() > self.p2.x() {
            self.p1
        } else {
            self.p2
        })
    }

    /// return the top point of a line that is not horizantal
    pub fn get_top_point(&self) -> Option<Point<i16>> {
        if self.slope == Slope::Horizontal {
            return None;
        }

        Some(if self.p1.y() > self.p2.y() {
            self.p1
        } else {
            self.p2
        })
    }

    /// return the bottom point of a line that is not horizantal
    pub fn get_bottom_point(&self) -> Option<Point<i16>> {
        if self.slope == Slope::Horizontal {
            return None;
        }

        Some(if self.p1.y() < self.p2.y() {
            self.p1
        } else {
            self.p2
        })
    }

    /// find the next point in the left direction from a given point if it's in bounds
    pub fn step_left_in_bounds(&self, point: Point<i16>) -> Option<Point<i16>> {
        let next_left_point = match self.slope {
            Slope::Up => {
                let next_left_point_x = point.x() - self.step.x() as i16;
                let next_left_point_y = point.y() - self.step.y() as i16;
                Some(Point::new(next_left_point_x, next_left_point_y))
            }
            Slope::Down => {
                let next_left_point_x = point.x() - self.step.x() as i16;
                let next_left_point_y = point.y() + self.step.y() as i16;
                Some(Point::new(next_left_point_x, next_left_point_y))
            }
            Slope::Vertical => None,
            Slope::Horizontal => {
                let next_left_point_x = point.x() - self.step.x() as i16;
                Some(Point::new(next_left_point_x, point.y()))
            }
        };

        if next_left_point.is_some() && self.is_point_in_bounds(next_left_point.unwrap()) {
            next_left_point
        } else {
            None
        }
    }

    /// find the next point in the right direction from a given point if it's in bounds
    pub fn step_right_in_bounds(&self, point: Point<i16>) -> Option<Point<i16>> {
        let next_right_point = match self.slope {
            Slope::Up => {
                let next_right_point_x = point.x() + self.step.x() as i16;
                let next_right_point_y = point.y() + self.step.y() as i16;
                Some(Point::new(next_right_point_x, next_right_point_y))
            }
            Slope::Down => {
                let next_right_point_x = point.x() + self.step.x() as i16;
                let next_right_point_y = point.y() - self.step.y() as i16;
                Some(Point::new(next_right_point_x, next_right_point_y))
            }
            Slope::Vertical => None,
            Slope::Horizontal => {
                let next_right_point_x = point.x() + self.step.x() as i16;
                Some(Point::new(next_right_point_x, point.y()))
            }
        };

        if next_right_point.is_some() && self.is_point_in_bounds(next_right_point.unwrap()) {
            next_right_point
        } else {
            None
        }
    }

    /// find the next point in the up direction from a given point if it's in bounds and the line is vertical
    pub fn step_up_in_bounds(&self, point: Point<i16>) -> Option<Point<i16>> {
        let next_up_point = match self.slope {
            Slope::Up => None,
            Slope::Down => None,
            Slope::Vertical => {
                let next_up_point_y = point.y() + self.step.y() as i16;
                Some(Point::new(point.x(), next_up_point_y))
            }
            Slope::Horizontal => None,
        };

        if next_up_point.is_some() && self.is_point_in_bounds(next_up_point.unwrap()) {
            next_up_point
        } else {
            None
        }
    }

    /// find the next point in the down direction from a given point if it's in bounds and the line is vertical
    pub fn step_down_in_bounds(&self, point: Point<i16>) -> Option<Point<i16>> {
        let next_down_point = match self.slope {
            Slope::Up => None,
            Slope::Down => None,
            Slope::Vertical => {
                let next_down_point_y = point.y() - self.step.y() as i16;
                Some(Point::new(point.x(), next_down_point_y))
            }
            Slope::Horizontal => None,
        };

        if next_down_point.is_some() && self.is_point_in_bounds(next_down_point.unwrap()) {
            next_down_point
        } else {
            None
        }
    }

    /// is a given point on the city map bounds
    pub fn is_point_in_bounds(&self, point: Point<i16>) -> bool {
        point.x() >= 0
            && point.y() >= 0
            && point.x() < self.width as i16
            && point.y() < self.height as i16
    }

    pub fn find_all_antinodes_in_bounds(&self) -> Vec<Point<i16>> {
        let mut antinodes: Vec<Point<i16>> = vec![];

        match self.slope {
            Slope::Up | Slope::Down | Slope::Horizontal => {
                let mut next_antinode = self.get_left_point();

                loop {
                    if next_antinode.is_some() {
                        next_antinode = self.step_left_in_bounds(next_antinode.unwrap());
                        if let Some(node) = next_antinode {
                            // println!("next left step: {:?}", node);
                            antinodes.push(node);
                        } else {
                            break;
                        }
                    }
                }

                next_antinode = self.get_right_point();

                loop {
                    if next_antinode.is_some() {
                        next_antinode = self.step_right_in_bounds(next_antinode.unwrap());
                        if let Some(node) = next_antinode {
                            // println!("next right step: {:?}", node);
                            antinodes.push(node);
                        } else {
                            break;
                        }
                    }
                }
            }
            Slope::Vertical => {
                let mut next_antinode = self.get_top_point();

                loop {
                    if next_antinode.is_some() {
                        next_antinode = self.step_up_in_bounds(next_antinode.unwrap());
                        if let Some(node) = next_antinode {
                            // println!("next up step: {:?}", node);
                            antinodes.push(node);
                        } else {
                            break;
                        }
                    }
                }

                next_antinode = self.get_bottom_point();

                loop {
                    if next_antinode.is_some() {
                        next_antinode = self.step_down_in_bounds(next_antinode.unwrap());
                        if let Some(node) = next_antinode {
                            // println!("next down step: {:?}", node);
                            antinodes.push(node);
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        antinodes
    }
}

fn calc_slope(p1: Point<i16>, p2: Point<i16>) -> f64 {
    // slope formula
    (p2.y() as f64 - p1.y() as f64) / (p2.x() as f64 - p1.x() as f64)
}

pub fn parse(input: String) -> CityMap {
    CityMap::from(input)
}

pub fn part1(model: Model) -> Answer {
    // dbg!(model);
    let lines = model.find_unique_antenna_lines();
    let mut unique_anti_coords: HashSet<&Point<i16>> = HashSet::new();

    for line in lines.values().flatten() {
        // dbg!(&line);
        //TODO: refactor this to get first element of antinodes vec
        if model.is_point_in_bounds(line.anti1) {
            // println!("in bounds: ({},{})", line.anti1.x(), line.anti1.y());
            unique_anti_coords.insert(&line.anti1);
        } else {
            // println!("out of bounds: ({},{})", line.anti1.x(), line.anti1.y());
        }

        //TODO: refactor this to get second element of antinodes vec
        if model.is_point_in_bounds(line.anti2) {
            // println!("in bounds: ({},{})", line.anti2.x(), line.anti2.y());
            unique_anti_coords.insert(&line.anti2);
        } else {
            // println!("out of bounds: ({},{})", line.anti2.x(), line.anti2.y());
        }
    }

    unique_anti_coords.len()
}

pub fn part2(model: Model) -> Answer {
    // dbg!(model);
    let lines = model.find_unique_antenna_lines();
    let mut unique_anti_coords: HashSet<Point<i16>> = HashSet::new();

    // add the anti cords to the hashset including the antena coords themselves
    for line in lines.values().flatten() {
        // antenna coords
        unique_anti_coords.insert(line.p1);
        unique_anti_coords.insert(line.p2);

        // antinode coords
        let antinodes = line.find_all_antinodes_in_bounds();
        for node in antinodes {
            unique_anti_coords.insert(node);
        }
    }

    unique_anti_coords.len()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    // const INPUT: &str = include_str!("../input/d8");
    const EXAMPLE: &str = include_str!("../examples/d8");

    #[test]
    fn unique_pairs_test() {
        let p1 = Point::new(1, 3);
        let p2 = Point::new(4, 7);

        let mut unique_pairs: HashSet<(Point<i16>, Point<i16>)> = HashSet::new();

        unique_pairs.insert((p1, p2));

        assert!(unique_pairs.contains(&(p1, p2)));
        assert!(!unique_pairs.contains(&(p2, p1)));

        unique_pairs.insert((p1, p2));

        assert_eq!(unique_pairs.len(), 1);
    }

    #[test]
    fn test_is_point_in_bounds() {
        let city_map = CityMap::from(EXAMPLE.to_string());

        // Test points within bounds
        assert!(city_map.is_point_in_bounds(Point::new(0, 0))); // Bottom-left corner
        assert!(city_map.is_point_in_bounds(Point::new(5, 10))); // Top-right corner
        assert!(city_map.is_point_in_bounds(Point::new(10, 7))); // Center point
        assert!(city_map.is_point_in_bounds(Point::new(5, 5))); // Center point

        // Test points out of bounds
        assert!(!city_map.is_point_in_bounds(Point::new(-1, 5))); // Negative x
        assert!(!city_map.is_point_in_bounds(Point::new(5, -1))); // Negative y
        assert!(!city_map.is_point_in_bounds(Point::new(12, 5))); // x exactly at width
        assert!(!city_map.is_point_in_bounds(Point::new(5, 12))); // y exactly at height
        assert!(!city_map.is_point_in_bounds(Point::new(200, 1))); // Far x outside bounds
        assert!(!city_map.is_point_in_bounds(Point::new(1, 200))); // Far y outside bounds
    }

    #[test]
    fn test_find_unique_point_pairs_static() {
        let points = vec![Point::new(1, 1), Point::new(2, 2), Point::new(3, 3)];

        let expected_pairs: HashSet<(Point<i16>, Point<i16>)> = [
            (Point::new(1, 1), Point::new(2, 2)),
            (Point::new(1, 1), Point::new(3, 3)),
            (Point::new(2, 2), Point::new(3, 3)),
        ]
        .iter()
        .cloned()
        .collect();

        let result_pairs = find_unique_point_pairs(&points);

        // Check if the result matches the expected pairs
        assert_eq!(result_pairs, expected_pairs);
    }

    #[test]
    fn test_find_unique_point_pairs_example() {
        let city_map = CityMap::from(EXAMPLE.to_string());

        let result_pairs = find_unique_point_pairs(city_map.antennas.get(&'0').unwrap());

        // Expected unique pairs
        let expected_pairs: HashSet<(Point<i16>, Point<i16>)> = [
            (Point::new(7, 8), Point::new(8, 10)),
            (Point::new(4, 7), Point::new(8, 10)),
            (Point::new(7, 8), Point::new(5, 9)),
            (Point::new(4, 7), Point::new(7, 8)),
            (Point::new(4, 7), Point::new(5, 9)),
            (Point::new(5, 9), Point::new(8, 10)),
        ]
        .iter()
        .cloned()
        .collect();

        // Check if the result matches the expected pairs
        assert_eq!(result_pairs, expected_pairs);
    }

    #[test]
    fn test_get_slope() {
        let slope = calc_slope(Point::new(3, 1), Point::new(4, 5));

        assert!(slope > 0.0);

        let slope = calc_slope(Point::new(4, 2), Point::new(1, 10));

        assert!(slope < 0.0);
    }

    #[test]
    fn test_step_left() {
        // slope UP
        let a_line = AntennaLine::new(10, 10, '0', Point::new(4, 4), Point::new(6, 8));
        assert_eq!(a_line.slope, Slope::Up);
        let left_point = a_line.get_left_point().unwrap();
        assert_eq!(Point::new(4, 4), left_point);
        let next_step_left = a_line.step_left_in_bounds(left_point).unwrap();
        let expected_point: Point<i16> = Point::new(2, 0);
        assert_eq!(expected_point, next_step_left);
        let next_next_step_left = a_line.step_left_in_bounds(next_step_left);
        assert!(next_next_step_left.is_none());

        // slope DOWN
        let a_line = AntennaLine::new(10, 10, '0', Point::new(4, 3), Point::new(8, 2));
        assert_eq!(a_line.slope, Slope::Down);
        let left_point = a_line.get_left_point().unwrap();
        assert_eq!(Point::new(4, 3), left_point);
        let next_step_left = a_line.step_left_in_bounds(left_point).unwrap();
        let expected_point: Point<i16> = Point::new(0, 4);
        assert_eq!(expected_point, next_step_left);
        let next_next_step_left = a_line.step_left_in_bounds(next_step_left);
        assert!(next_next_step_left.is_none());

        // slope Horizantal
        let a_line = AntennaLine::new(10, 10, '0', Point::new(6, 0), Point::new(3, 0));
        assert_eq!(a_line.slope, Slope::Horizontal);
        let left_point = a_line.get_left_point().unwrap();
        assert_eq!(Point::new(3, 0), left_point);
        let next_step_left = a_line.step_left_in_bounds(left_point).unwrap();
        let expected_point: Point<i16> = Point::new(0, 0);
        assert_eq!(expected_point, next_step_left);
        let next_next_step_left = a_line.step_left_in_bounds(next_step_left);
        assert!(next_next_step_left.is_none());

        // slope Vertical
        let a_line = AntennaLine::new(10, 10, '0', Point::new(4, 9), Point::new(4, 1));
        assert_eq!(a_line.slope, Slope::Vertical);
        assert_eq!(a_line.get_left_point(), None);
        assert_eq!(a_line.step_left_in_bounds(a_line.p1), None);
    }

    #[test]
    fn test_step_right() {
        // slope UP
        let a_line = AntennaLine::new(10, 10, '0', Point::new(2, 1), Point::new(3, 5));
        assert_eq!(a_line.slope, Slope::Up);
        let right_point = a_line.get_right_point().unwrap();
        assert_eq!(Point::new(3, 5), right_point);
        let next_step_left = a_line.step_right_in_bounds(right_point).unwrap();
        let expected_point: Point<i16> = Point::new(4, 9);
        assert_eq!(expected_point, next_step_left);
        let next_next_step_left = a_line.step_right_in_bounds(next_step_left);
        assert_eq!(next_next_step_left, None);

        // slope DOWN
        let a_line = AntennaLine::new(10, 10, '0', Point::new(1, 8), Point::new(3, 4));
        assert_eq!(a_line.slope, Slope::Down);
        let right_point = a_line.get_right_point().unwrap();
        assert_eq!(Point::new(3, 4), right_point);
        let next_step_left = a_line.step_right_in_bounds(right_point).unwrap();
        let expected_point: Point<i16> = Point::new(5, 0);
        assert_eq!(expected_point, next_step_left);
        let next_next_step_left = a_line.step_right_in_bounds(next_step_left);
        assert_eq!(next_next_step_left, None);

        // slope Vertical
        let a_line = AntennaLine::new(10, 10, '0', Point::new(4, 9), Point::new(4, 1));
        assert_eq!(a_line.slope, Slope::Vertical);
        assert_eq!(a_line.get_right_point(), None);
        assert_eq!(a_line.step_right_in_bounds(a_line.p1), None);

        // slope Horizantal
        let a_line = AntennaLine::new(10, 10, '0', Point::new(1, 8), Point::new(5, 8));
        assert_eq!(a_line.slope, Slope::Horizontal);
        let right_point = a_line.get_right_point().unwrap();
        assert_eq!(Point::new(5, 8), right_point);
        let next_step_left = a_line.step_right_in_bounds(right_point).unwrap();
        let expected_point: Point<i16> = Point::new(9, 8);
        assert_eq!(expected_point, next_step_left);
        let next_next_step_left = a_line.step_right_in_bounds(next_step_left);
        assert_eq!(next_next_step_left, None);
    }

    #[test]
    fn test_step_up() {
        // slope Vertical
        let a_line = AntennaLine::new(10, 10, '0', Point::new(4, 7), Point::new(4, 5));
        let top_point = a_line.get_top_point().unwrap();
        assert_eq!(Point::new(4, 7), top_point);
        assert_eq!(a_line.slope, Slope::Vertical);
        let next_up_point = a_line.step_up_in_bounds(top_point).unwrap();
        assert_eq!(Point::new(4, 9), next_up_point);
        assert_eq!(a_line.step_up_in_bounds(next_up_point), None);
    }

    #[test]
    fn test_step_down() {
        // slope Vertical
        let a_line = AntennaLine::new(10, 10, '0', Point::new(4, 8), Point::new(4, 5));
        let bottom_point = a_line.get_bottom_point().unwrap();
        assert_eq!(Point::new(4, 5), bottom_point);
        assert_eq!(a_line.slope, Slope::Vertical);
        let next_down_point = a_line.step_down_in_bounds(bottom_point).unwrap();
        assert_eq!(Point::new(4, 2), next_down_point);
        assert_eq!(a_line.step_down_in_bounds(next_down_point), None);
    }
}
