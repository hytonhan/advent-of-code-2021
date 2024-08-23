advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let line_length = input.lines().next().unwrap().len();
    let mut majority_numbers: Vec<i32> = vec![0; line_length];

    input.lines().for_each(|line| {
        for (i, char) in line.chars().enumerate() {
            match char {
                '0' => majority_numbers[i] -= 1,
                '1' => majority_numbers[i] += 1,
                _ => panic!("Should not happen."),
            }
        }
    });

    let mut gamma_rate_string = String::new();
    let mut epsilon_rate_string = String::new();
    for x in majority_numbers {
        if x > 0 {
            gamma_rate_string.push('1');
            epsilon_rate_string.push('0');
            continue;
        } else {
            gamma_rate_string.push('0');
            epsilon_rate_string.push('1');
        }
    }
    let gamma_rate = u32::from_str_radix(&gamma_rate_string, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate_string, 2).unwrap();
    Some(gamma_rate * epsilon_rate)
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_length = input.lines().next().unwrap().len();

    let mut oxygen_rate_string: &str = "";
    let mut co2_rate_string: &str = "";

    let a: Vec<_> = input.lines().collect();
    let mut oxygen: Vec<usize> = Vec::new();
    let mut co2: Vec<usize> = Vec::new();
    for x in 0..a.len() {
        oxygen.push(x);
        co2.push(x);
    }

    for i in 0..line_length {
        let mut sum = 0;
        let mut sum2 = 0;
        for j in oxygen.clone() {
            if input.lines().nth(j).unwrap().chars().nth(i).unwrap() == '1' {
                sum += 1;
            } else {
                sum -= 1;
            }
        }
        for j2 in co2.clone() {
            if input.lines().nth(j2).unwrap().chars().nth(i).unwrap() == '1' {
                sum2 += 1;
            } else {
                sum2 -= 1;
            }
        }
        if sum >= 0 {
            if oxygen.len() > 1 {
                oxygen = oxygen
                    .into_iter()
                    .filter(|item| input.lines().nth(*item).unwrap().chars().nth(i).unwrap() == '1')
                    .collect();
            }
        }
        if sum2 >= 0 {
            if co2.len() > 1 {
                co2 = co2
                    .into_iter()
                    .filter(|item| input.lines().nth(*item).unwrap().chars().nth(i).unwrap() == '0')
                    .collect();
            }
        }
        if sum < 0 {
            if oxygen.len() > 1 {
                oxygen = oxygen
                    .into_iter()
                    .filter(|item| input.lines().nth(*item).unwrap().chars().nth(i).unwrap() == '0')
                    .collect();
            }
        }
        if sum2 < 0 {
            if co2.len() > 1 {
                co2 = co2
                    .into_iter()
                    .filter(|item| input.lines().nth(*item).unwrap().chars().nth(i).unwrap() == '1')
                    .collect();
            }
        }
        if oxygen.len() == 1 {
            oxygen_rate_string = input.lines().nth(*oxygen.first().unwrap()).unwrap();
        }
        if co2.len() == 1 {
            co2_rate_string = input.lines().nth(*co2.first().unwrap()).unwrap();
        }
        if oxygen.len() == 1 && co2.len() == 1 {
            break;
        }
    }

    let oxygen_rate = u32::from_str_radix(&oxygen_rate_string, 2).unwrap();
    let co2_rate = u32::from_str_radix(&co2_rate_string, 2).unwrap();
    Some(oxygen_rate * co2_rate)
    // 4030301 too low

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(198));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(230));
    }
}
