advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0i32;

    let mut index = 0usize;
    let rows: Vec<&str> = input.lines().collect::<Vec<_>>();

    while index < rows.len() {
        // Positions of symbols in this row and adjacent
        let mut symbol_positions: Vec<usize> = Vec::new();

        // symbols in current row
        for (i, char) in rows[index].chars().enumerate() {
            if char != '.' && !char.is_ascii_digit() {
                symbol_positions.push(i)
            }
        }
        // symbols in previous row
        if index > 0 {
            for (i, char) in rows[index - 1].chars().enumerate() {
                if char != '.' && !char.is_ascii_digit() {
                    symbol_positions.push(i)
                }
            }
        }
        // symbols in next row
        if index + 1 < rows.len() {
            for (i, char) in rows[index + 1].chars().enumerate() {
                if char != '.' && !char.is_ascii_digit() {
                    symbol_positions.push(i)
                }
            }
        }
        println!("LINE---- {:?}", symbol_positions);
        println!("LINE---- {:?}", rows[index]);

        let mut found_a_number_start = false;
        let mut where_number_starts = 0usize;
        let mut number = String::new();
        for (i, char) in rows[index].chars().enumerate() {
            if char.is_ascii_digit() {
                if !found_a_number_start {
                    found_a_number_start = true;
                    where_number_starts = i.clone();
                }
                number.push(char);
            } else {
                found_a_number_start = false;
                if number.len() > 0 {
                    let mut adjacent_star = where_number_starts;
                    let adjacent_end = where_number_starts + number.len();
                    if adjacent_star != 0 {
                        adjacent_star -= 1;
                    }
                    let found = symbol_positions
                        .iter()
                        .find(|&&size| size >= adjacent_star && size <= adjacent_end);
                    // println!("{:?} {}", number, found.iter().len() > 0);
                    if found.iter().len() > 0 {
                        sum += number.parse::<i32>().unwrap();
                    }
                }
                number = String::new();
            }
        }
        if number.len() > 0 {
            let mut adjacent_star = where_number_starts;
            let adjacent_end = where_number_starts + number.len();
            if adjacent_star != 0 {
                adjacent_star -= 1;
            }
            let found = symbol_positions
                .iter()
                .find(|&&size| size >= adjacent_star && size <= adjacent_end);
            // println!("{:?} {}", number, found.iter().len() > 0);
            if found.iter().len() > 0 {
                sum += number.parse::<i32>().unwrap();
            }
        }
        // println!("{:?}", lines[index].split('.').collect::<Vec<&str>>());
        index += 1usize;
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
        assert_eq!(result, Some(4360u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
