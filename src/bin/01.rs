
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {

    let lines = input.lines();
    let mut previous_depth: u32 = 0;
    let mut depth_increased_count: u32 = 0;
    for line in lines {
        if previous_depth == 0 {
            previous_depth = line.parse::<u32>().unwrap();
            continue;
        }
        let current_depth = line.parse::<u32>().unwrap();

        if current_depth > previous_depth {
            depth_increased_count += 1;
        }
        previous_depth = current_depth;
    }
    Some(depth_increased_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers: Vec<usize>;
    numbers.windows(size)
    let lines = input.lines();
    let mut depth_increased_count: u32 = 0;
    for i in 0..lines.count() - 3 {
        let first = input.lines().nth(i).unwrap().parse::<u32>().unwrap();
        let second = input.lines().nth(i+3).unwrap().parse::<u32>().unwrap();

        if second > first {
            depth_increased_count +=1;
        }
    }
    Some(depth_increased_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
