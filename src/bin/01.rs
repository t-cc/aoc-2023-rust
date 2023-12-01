advent_of_code::solution!(1);

const NUMBER_NAMES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut numbers = Vec::new();
        for c in line.chars() {
            if c.is_ascii_digit() {
                numbers.push(c.to_digit(10).unwrap());
            }
        }
        if numbers.len() > 0 {
            let first = numbers.first().copied().unwrap();
            let last = numbers.last().copied().unwrap();
            sum += first * 10 + last;
        }
    }
    return sum.into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut numbers = Vec::new();
        let mut index = 0;
        let line_size= line.len();
        while index < line_size {
            let slice = &line[index..line_size ];
            let char = slice.to_string().chars().next().unwrap();
            if char.is_ascii_digit() {
                numbers.push(char.to_digit(10).unwrap());
            } else {
                for (name_index, name) in NUMBER_NAMES.iter().enumerate() {
                    if slice.starts_with(name) {
                        numbers.push(name_index as u32 + 1);
                        break
                    }
                }
            }
            index += 1
        }
        let first = numbers.first().copied().unwrap();
        let last = numbers.last().copied().unwrap();
        sum += first * 10 + last;
    }
    return sum.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(282u32));
    }
}
