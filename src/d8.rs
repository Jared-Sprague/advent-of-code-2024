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
                let line = AntennaLine::new(freq, p.0, p.1);
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

#[derive(Debug)]
pub enum Slope {
    Up,
    Down,
    Vertical,
    Horizontal,
}

#[derive(Debug)]
pub struct AntennaLine {
    #[allow(unused)]
    freq: Frequency,
    #[allow(unused)]
    slope: Slope,
    #[allow(unused)]
    step: Point<u16>,
    #[allow(unused)]
    coord1: Point<i16>,
    #[allow(unused)]
    coord2: Point<i16>,
    anti1: Point<i16>,
    anti2: Point<i16>,
}

impl AntennaLine {
    pub fn new(freq: Frequency, p1: Point<i16>, p2: Point<i16>) -> Self {
        // calculate the sloop
        let slope_num = get_slope(p1, p2);
        dbg!("slope_num: {}", slope_num);

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

        // calculate the anti-points
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

        // TOOD: implement
        AntennaLine {
            freq,
            slope,
            step,
            coord1: p1,
            coord2: p2,
            anti1,
            anti2,
        }
    }
}

fn get_slope(p1: Point<i16>, p2: Point<i16>) -> f64 {
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
        dbg!(&line);
        if model.is_point_in_bounds(line.anti1) {
            println!("in bounds: ({},{})", line.anti1.x(), line.anti1.y());
            unique_anti_coords.insert(&line.anti1);
        } else {
            // println!("out of bounds: ({},{})", line.anti1.x(), line.anti1.y());
        }

        if model.is_point_in_bounds(line.anti2) {
            println!("in bounds: ({},{})", line.anti2.x(), line.anti2.y());
            unique_anti_coords.insert(&line.anti2);
        } else {
            // println!("out of bounds: ({},{})", line.anti2.x(), line.anti2.y());
        }
    }

    unique_anti_coords.len()
}

pub fn part2(model: Model) -> Answer {
    0
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
        let slope = get_slope(Point::new(3, 1), Point::new(4, 5));

        assert!(slope > 0.0);

        let slope = get_slope(Point::new(4, 2), Point::new(1, 10));

        assert!(slope < 0.0);
    }
}
