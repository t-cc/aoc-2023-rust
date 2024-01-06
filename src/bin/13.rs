advent_of_code::solution!(13);

fn find_first_vertical_reflection(pattern: Vec<String>) -> u32 {
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
            return (i + 1) as u32;
        }
    }

    return 0u32;
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

fn build_patterns(input: &str) -> Vec<Vec<String>> {
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
    }
    return pattern_list;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0u32;

    let pattern_list = build_patterns(input);
    // println!("patterns: {:?}", pattern_list);

    for pattern in pattern_list {
        let vertical = find_first_vertical_reflection(pattern.clone());
        // println!("{:?} -> vertical {:?}", pattern, vertical);

        sum += 100 * vertical;

        let transposed = transpose_patter(pattern.clone());
        let horizontal = find_first_vertical_reflection(transposed.clone());
        // println!("{:?} -> horizontal {:?}", transposed, horizontal);
        sum += horizontal;
    }

    return Some(sum);
}

fn find_different_reflection(pattern: Vec<String>, original: u32) -> u32 {
    let first: String = pattern.first().unwrap().clone();

    let y_max = pattern.len();
    let x_max = first.len();
    for y in 0..y_max {
        for x in 0..x_max {
            let mut new_pattern = pattern.clone();

            let mut row = pattern.iter().nth(y).unwrap().clone();
            // if y == 0 {
            //     println!("ORIGNAL ROW {:?}", row);
            // }
            let char = row.chars().nth(x).unwrap();
            let mut next_chart = ".";
            if char == '.' {
                next_chart = "#";
            }
            row.replace_range(x..x + 1, next_chart);
            // if y == 0 {
            //    println!("NEW ROW     {:?}", row);
            // }
            new_pattern[y] = row;
            let new_reflection = find_first_vertical_reflection(new_pattern.clone());
            // println!(
            //    "CLEANED: {:?} -> {:?} ?= {:?}",
            //    new_pattern, new_reflection, original
            // );
            if new_reflection > 0 && new_reflection != original {
                return new_reflection;
            }
        }
    }

    return 0u32;
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0u32;
    let pattern_list = build_patterns(input);
    // println!("patterns: {:?}", pattern_list);

    for pattern in pattern_list {
        let vertical = find_first_vertical_reflection(pattern.clone());
        // println!("{:?} -> vertical {:?}", pattern, vertical);
        let new_vertical = find_different_reflection(pattern.clone(), vertical);
        sum += 100 * new_vertical;

        let transposed = transpose_patter(pattern.clone());
        let horizontal = find_first_vertical_reflection(transposed.clone());
        // println!("{:?} -> horizontal {:?}", transposed, horizontal);
        let new_horizontal = find_different_reflection(transposed.clone(), horizontal);
        sum += new_horizontal;
    }

    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400u32));
    }
}
