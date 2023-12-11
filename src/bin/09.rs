advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0i32;
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|c| c.trim())
            .filter(|&c| c.len() != 0)
            .map(|c| c.parse::<i32>().unwrap())
            .collect();
        let mut diferences: Vec<i32> = numbers.clone();
        let mut last_numbers: Vec<i32> = Vec::new();
        last_numbers.push(numbers.last().unwrap().clone());
        // sum += numbers.last().unwrap().clone();
        while !diferences
            .iter()
            .map(|&a| a == 0i32)
            .reduce(|a, b| a && b)
            .unwrap()
        {
            let mut tmp: Vec<i32> = Vec::new();
            for i in 0..diferences.len() - 1 {
                tmp.push(diferences[i + 1] - diferences[i])
            }
            last_numbers.push(tmp.last().unwrap().clone());
            diferences = tmp.clone();
            // sum += diferences.last().unwrap().clone();
        }
        sum += last_numbers.iter().cloned().reduce(|a, b| a + b).unwrap()
    }

    return (sum as u32).into();
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
        assert_eq!(result, Some(114u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
