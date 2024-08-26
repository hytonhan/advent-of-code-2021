advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {

    let mut fish: [u32; 9] = [0; 9];
    for i in 0..9 {
        let value = input
            .trim()
            .split(',')
            .filter(|item| item.parse::<u32>().unwrap() == i)
            .count();
        fish[i as usize] = value as u32;
    }

    for _ in 0..80 {
        let mut new_fish: [u32; 9] = [0; 9];
        for j in 0..9 {
            if j == 0 {
                new_fish[6] = fish[0];
                new_fish[8] = fish[0];
                continue;
            }
            new_fish[j-1] += fish[j];
        }
        fish = new_fish;
    }

    let result: u32 = fish
        .into_iter()
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {

    let mut fish: [u64; 9] = [0; 9];
    for i in 0..9 {
        let value = input
            .trim()
            .split(',')
            .filter(|item| item.parse::<u64>().unwrap() == i)
            .count();
        fish[i as usize] = value as u64;
    }

    for _ in 0..256 {
        let mut new_fish: [u64; 9] = [0; 9];
        for j in 0..9 {
            if j == 0 {
                new_fish[6] = fish[0];
                new_fish[8] = fish[0];
                continue;
            }
            new_fish[j-1] += fish[j];
        }
        fish = new_fish;
    }

    let result: u64 = fish
        .into_iter()
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5934));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26984457539));
    }
}
