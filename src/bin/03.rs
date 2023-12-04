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

struct NumberPosition {
    value: i32,
    start: usize,
    end: usize,
}

fn get_numbers_from_line(rows: &str, numbers: &mut Vec<NumberPosition>) {
    let mut found_a_number_start = false;
    let mut where_number_starts = 0usize;
    let mut number = String::new();
    for (i, char) in rows.chars().enumerate() {
        if char.is_ascii_digit() {
            if !found_a_number_start {
                found_a_number_start = true;
                where_number_starts = i;
            }
            number.push(char);
        } else {
            found_a_number_start = false;
            if number.len() > 0 {
                let mut number_data = NumberPosition {
                    value: number.parse::<i32>().unwrap(),
                    start: where_number_starts,
                    end: where_number_starts + number.len(),
                };
                if number_data.start > 0 {
                    number_data.start = number_data.start - 1;
                }
                numbers.push(number_data);
                println!("{:?}", number)
            }
            number = String::new();
        }
    }
    if number.len() > 0 {
        let mut number_data = NumberPosition {
            value: number.parse::<i32>().unwrap(),
            start: where_number_starts,
            end: where_number_starts + number.len(),
        };
        if number_data.start > 0 {
            number_data.start = number_data.start - 1;
        }
        numbers.push(number_data);
        println!("{:?}", number)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0i32;

    let mut index = 0usize;
    let rows: Vec<&str> = input.lines().collect::<Vec<_>>();

    while index < rows.len() {
        let mut numbers: Vec<NumberPosition> = Vec::new();
        // numbers in the current row
        get_numbers_from_line(rows[index], &mut numbers);
        // from the previous line...
        if index > 0 {
            get_numbers_from_line(rows[index - 1], &mut numbers);
        }
        // and the next line
        if index + 1 < rows.len() {
            get_numbers_from_line(rows[index + 1], &mut numbers);
        }

        // find * in current line
        for (i, char) in rows[index].chars().enumerate() {
            if char == '*' {
                let mut num_list: Vec<i32> = Vec::new();
                for number in &numbers {
                    if number.start <= i && number.end >= i {
                        num_list.push(number.value)
                    }
                }
                if num_list.len() == 2 {
                    let ratio = num_list.iter().next().unwrap() * num_list.iter().last().unwrap();
                    sum += ratio;
                    println!("{:?} -> {:?}", num_list, ratio);
                }
            }
        }

        index += 1;
    }

    return (sum as u32).into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835u32));
    }
}
