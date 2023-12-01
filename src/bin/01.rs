advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut numbers = Vec::new();
        for c in line.chars() {
            if c.is_ascii_digit() {
                numbers.push(c.to_digit(10).unwrap());
            }
        }
        let first = numbers.first().copied().unwrap();
        let last = numbers.last().copied().unwrap();
        sum += first * 10 + last;
    }
    return sum.into()
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
