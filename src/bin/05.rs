use core::fmt;

advent_of_code::solution!(5);

struct Coordinate {
    x: usize,
    y: usize
}

struct Line {
    start: Coordinate,
    end: Coordinate
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
}

// impl fmt::Debug for Line {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_struct("Line")
//             .field("start", String::from("(" + &self.start.x + ", " + &self.start.y + ")"))
//             .field("end", String::from("(" + &self.end.x + ", " + &self.end.y + ")"))
//             .finish()
//     }
// }

pub fn part_one(input: &str) -> Option<u32> {

    let lines: Vec<Line> = input
        .lines()
        .filter_map(|item| {
            let parts = item.split_once(" -> ").unwrap();
            let start = Coordinate{x: parts.0.chars().nth(0).unwrap().to_digit(10).unwrap() as usize, y: parts.0.chars().nth(2).unwrap().to_digit(10).unwrap() as usize};
            let end = Coordinate{x: parts.1.chars().nth(0).unwrap().to_digit(10).unwrap() as usize, y: parts.1.chars().nth(2).unwrap().to_digit(10).unwrap() as usize};
            let line = Line{start: start, end: end};
            if (line.is_straight()) {
                return Some(line);
            }
            None
        })
        .collect();

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
