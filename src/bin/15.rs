advent_of_code::solution!(15);

fn get_value(chars: &str) -> u32 {
    let mut value = 0u32;
    for char in chars.chars() {
        let ascii = char as u32;
        // println!("{:?}", char);
        value += ascii;
        value = value * 17;
        value = value % 256;
    }
    return value;
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0u32;

    let line: Vec<&str> = input.lines().next().unwrap().split(',').collect();
    // println!("{:?}", line);
    for part in &line {
        sum += get_value(part);
    }
    return Some(sum);
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
        assert_eq!(result, Some(1320u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
