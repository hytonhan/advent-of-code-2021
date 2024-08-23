advent_of_code::solution!(2);

struct Submarine {
    x: i32,
    y: i32,
    aim: i32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut submarine_position = Submarine{x: 0, y: 0, aim: 0};
    input.lines()
        .for_each(|line| {
            let parts = line.split_once(' ').unwrap();
            match parts.0 {
                "forward" => submarine_position.x += parts.1.parse::<i32>().unwrap(),
                "down" => submarine_position.y -= parts.1.parse::<i32>().unwrap(),
                "up" => submarine_position.y += parts.1.parse::<i32>().unwrap(),
                _ => panic!("This should be impossible.")
            }
        });
    let depth = submarine_position.x * (-submarine_position.y);
    Some(depth as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut submarine_position = Submarine{x: 0, y: 0, aim: 0};
    input.lines()
        .for_each(|line| {
            let parts = line.split_once(' ').unwrap();
            match parts.0 {
                "forward" => { 
                    submarine_position.x += parts.1.parse::<i32>().unwrap();
                    submarine_position.y -= parts.1.parse::<i32>().unwrap() * submarine_position.aim
                },
                "down" => submarine_position.aim += parts.1.parse::<i32>().unwrap(),
                "up" => submarine_position.aim -= parts.1.parse::<i32>().unwrap(),
                _ => panic!("This should be impossible.")
            }
        });
    let depth = submarine_position.x * (-submarine_position.y);
    Some(depth as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(150));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(900));
    }
}
