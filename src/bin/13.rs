advent_of_code::solution!(13);

fn find_first_vertical_reflection(pattern: Vec<String>) -> u32 {
    let mut sum = 0u32;
    let max_y = pattern.len() - 1;
    for i in 0..max_y {
        // println!("INDEX {:?}", i);
        let limit = [i + 1, max_y - i].iter().min().unwrap().clone();
        let mut all_equal = true;
        for j in 0..limit {
            //println!("CMP {:?} ?= {:?}", i - j, i + j + 1);
            if pattern.iter().nth(i - j).unwrap() != pattern.iter().nth(i + 1 + j).unwrap() {
                all_equal = false;
            }
        }
        if all_equal {
            sum += (i + 1) as u32;
        }
    }

    return sum;
}

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

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0u32;
    let mut pattern: Vec<String> = Vec::new();
    let mut pattern_list: Vec<Vec<String>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            if !pattern.is_empty() {
                pattern_list.push(pattern);
                pattern = Vec::new();
            }
        } else {
            pattern.push(line.to_string())
        }
    }
    if !pattern.is_empty() {
        pattern_list.push(pattern);
        pattern = Vec::new();
    }
    // println!("patterns: {:?}", pattern_list);

    for pattern in pattern_list {
        let vertical = find_first_vertical_reflection(pattern.clone());
        // println!("{:?} -> vertical {:?}", pattern, vertical);
        if vertical > 0 {
            sum += 100 * vertical;
        }
        let transposed = transpose_patter(pattern.clone());
        let horizontal = find_first_vertical_reflection(transposed.clone());
        // println!("{:?} -> horizontal {:?}", transposed, horizontal);
        if horizontal > 0 {
            sum += horizontal;
        }
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
        assert_eq!(result, Some(4059u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
