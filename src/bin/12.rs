advent_of_code::solution!(12);

fn parse_numbers(row: &str) -> Vec<usize> {
    return row.split(',').map(|value| value.parse().unwrap()).collect();
}

fn check_combinations(record: String, numbers: Vec<usize>) -> usize {
    if record.is_empty() {
        if numbers.len() == 0 {
            return 1;
        }
        return 0;
    }

    if numbers.len() == 0 {
        if record.chars().find(|&c| c == '#').is_some() {
            return 0;
        }
        return 1;
    }

    let mut sum = 0usize;

    let first_char: char = record.chars().nth(0).unwrap();
    if first_char == '.' || first_char == '?' {
        let next_chars = (&record[1..]).to_string();
        sum += check_combinations(next_chars, numbers.clone());
    }
    if first_char == '#' || first_char == '?' {
        let first_number = numbers.first().cloned().unwrap();
        let has_point: bool = record[..first_number].chars().find(|&c| c == '.').is_some();
        if first_number <= record.len()
            && !has_point
            && (first_number == record.len() || record.chars().nth(first_number).unwrap() != '#')
        {
            if record.len() < first_number + 1 {
                sum += check_combinations(
                    record[first_number + 1..].to_string(),
                    numbers[1..].to_vec(),
                )
            }
        }
    }

    return sum;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0usize;
    for line in input.lines() {
        let mut row = line.split_whitespace();
        let record = row.next().unwrap().to_string();
        let numbers = parse_numbers(row.last().unwrap());
        sum += check_combinations(record, numbers);
    }

    return Some(sum as u32);
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
        assert_eq!(result, Some(21u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
