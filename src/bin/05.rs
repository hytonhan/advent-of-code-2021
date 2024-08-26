use std::{collections::HashMap, hash::Hash};

advent_of_code::solution!(5);

#[derive(Hash, Clone, Copy)]
#[derive(Eq)]
#[derive(PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

// impl Hash for Coordinate {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.x.hash(state);
//         self.y.hash(state);
//     }
// }

struct Line {
    start: Coordinate,
    end: Coordinate,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y.eq(&self.end.y)
    }

    fn is_vertical(&self) -> bool {
        self.start.x.eq(&self.end.x)
    }

    pub fn is_straight(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    pub fn collides_with(&self, other_line: Line) -> bool {
        let x_collides = self.start.x >= other_line.start.x && self.start.x <= other_line.end.x || // fully in
            self.start.x < other_line.start.x && self.end.x >= other_line.start.x || // bigger from left
            self.end.x > other_line.end.x && self.start.x <= other_line.end.x || // bugger from right
            self.start.x < other_line.start.x && self.end.x > other_line.end.x; //bigger from both sides
        let y_collides = self.start.y >= other_line.start.y && self.start.y <= other_line.end.y || // fully in
                self.start.y < other_line.start.y && self.end.y >= other_line.start.y || // bigger from left
                self.end.y > other_line.end.y && self.start.y <= other_line.end.y || // bugger from right
                self.start.y < other_line.start.y && self.end.y > other_line.end.y; //bigger from both sides
        x_collides && y_collides        
    }

    pub fn delta_x(&self) -> i32 {
        self.end.x - self.start.x
    }

    pub fn delta_y(&self) -> i32 {
        self.end.y - self.start.y
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Line> = input
        .trim()
        .lines()
        .filter_map(|item| {
            let parts = item.split_once(" -> ").unwrap();
            let start = Coordinate {
                x: parts.0.split_once(',').unwrap().0.parse().unwrap(),
                y: parts.0.split_once(',').unwrap().1.parse().unwrap(),
            };
            let end = Coordinate {
                x: parts.1.split_once(',').unwrap().0.parse().unwrap(),
                y: parts.1.split_once(',').unwrap().1.parse().unwrap(),
            };
            let line = Line {
                start: start,
                end: end,
            };
            if line.is_straight() {
                return Some(line);
            }
            None
        })
        .collect();

    let mut visited_coordinates: HashMap<Coordinate, usize> = HashMap::new();

    lines.into_iter()
        .for_each(|item| {
            let mut delta_x = item.delta_x();
            if delta_x != 0 {
                delta_x /= delta_x.abs();
            }
            let mut delta_y = item.delta_y();
            if delta_y != 0 {
                delta_y /= delta_y.abs();
            }
            let mut coord = item.start;

            while coord != item.end {                
                let mut visits = 1;
                if visited_coordinates.contains_key(&coord) {
                    visits = visited_coordinates.get(&coord).unwrap() + 1;
                }
                visited_coordinates.insert(coord, visits);
                coord = Coordinate{x: coord.x + delta_x, y: coord.y + delta_y};
            }
            let mut visits = 1;
            if visited_coordinates.contains_key(&coord) {
                visits = visited_coordinates.get(&coord).unwrap() + 1;
            }
            visited_coordinates.insert(coord, visits);
        });

    let result = visited_coordinates
        .into_iter()
        .filter(|item| item.1 > 1)
        .count();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Line> = input
        .trim()
        .lines()
        .map(|item| {
            let parts = item.split_once(" -> ").unwrap();
            let start = Coordinate {
                x: parts.0.split_once(',').unwrap().0.parse().unwrap(),
                y: parts.0.split_once(',').unwrap().1.parse().unwrap(),
            };
            let end = Coordinate {
                x: parts.1.split_once(',').unwrap().0.parse().unwrap(),
                y: parts.1.split_once(',').unwrap().1.parse().unwrap(),
            };
            let line = Line {
                start: start,
                end: end,
            };
            line
        })
        .collect();

    let mut visited_coordinates: HashMap<Coordinate, usize> = HashMap::new();

    lines.into_iter()
        .for_each(|item| {
            let mut delta_x = item.delta_x();
            if delta_x != 0 {
                delta_x /= delta_x.abs();
            }
            let mut delta_y = item.delta_y();
            if delta_y != 0 {
                delta_y /= delta_y.abs();
            }
            let mut coord = item.start;

            while coord != item.end {                
                let mut visits = 1;
                if visited_coordinates.contains_key(&coord) {
                    visits = visited_coordinates.get(&coord).unwrap() + 1;
                }
                visited_coordinates.insert(coord, visits);
                coord = Coordinate{x: coord.x + delta_x, y: coord.y + delta_y};
            }
            let mut visits = 1;
            if visited_coordinates.contains_key(&coord) {
                visits = visited_coordinates.get(&coord).unwrap() + 1;
            }
            visited_coordinates.insert(coord, visits);
        });

    let result = visited_coordinates
        .into_iter()
        .filter(|item| item.1 > 1)
        .count();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
