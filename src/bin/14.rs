advent_of_code::solution!(14);

fn transpose_patter(pattern: Vec<String>) -> Vec<String> {
    let first: String = pattern.first().unwrap().clone();
    let mut transposed = Vec::new();
    for i in 0..first.len() {
        let mut string = String::new();
        for row in pattern.iter().cloned() {
            string.push(row.chars().nth(i).unwrap());
        }
        transposed.push(string);
    }

    return transposed;
}

fn build_plane(input: &str) -> Vec<String> {
    let mut vector = Vec::new();
    for line in input.lines() {
        vector.push(line.to_string());
    }

    return transpose_patter(vector);
}

fn move_rocks_to_left(line: String) -> String {
    let mut new_line: Vec<String> = Vec::new();
    for part in line.split('#') {
        let mut rocks = String::new();
        let mut empty = String::new();
        for char in part.chars() {
            if char == 'O' {
                rocks.push('O');
            } else {
                empty.push('.')
            }
        }
        rocks.push_str(&empty);
        new_line.push(rocks);
    }
    return new_line.join("#");
}

fn get_rocks_weight_for_line(line: String) -> u32 {
    let mut sum = 0usize;
    let max = line.len();
    for i in 0..max {
        if line.chars().nth(i).unwrap() == 'O' {
            sum += max - i;
        }
    }
    return sum as u32;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0u32;
    let plane = build_plane(input);
    // println!("PLANE: {:?}", plane);
    for line in plane {
        // println!("OLD {:?}", line);
        let new_line = move_rocks_to_left(line);
        // println!("NEW {:?}", new_line);
        sum += get_rocks_weight_for_line(new_line);
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    return Some(0u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0u32));
    }
}
