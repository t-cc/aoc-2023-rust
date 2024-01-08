use std::collections::HashMap;
advent_of_code::solution!(14);

#[derive(PartialEq)]
enum Direction {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

fn transpose_reverse_clockwise(pattern: Vec<String>) -> Vec<String> {
    let first: String = pattern.first().unwrap().clone();
    let mut transposed = Vec::new();
    for i in 0..first.len() {
        let mut string = String::new();
        for row in pattern.iter().cloned() {
            string.push(row.chars().nth(i).unwrap());
        }
        transposed.push(string);
    }
    transposed.reverse();

    return transposed;
}

fn transpose_clockwise(pattern: Vec<String>) -> Vec<String> {
    let first: String = pattern.first().unwrap().clone();
    let i_max = first.len();
    let mut transposed = Vec::new();
    for i_ in 0..i_max {
        let i = i_max - i_ - 1;
        let mut string = String::new();
        for row in pattern.iter().cloned() {
            string.push(row.chars().nth(i).unwrap());
        }
        transposed.push(string.chars().rev().collect::<String>());
    }
    transposed.reverse();

    return transposed;
}

fn build_plane(input: &str) -> Vec<String> {
    let mut vector = Vec::new();
    for line in input.lines() {
        vector.push(line.to_string());
    }
    return vector;
}

fn move_rocks_line_to(line: String, direction: Direction) -> String {
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
        if direction == Direction::LEFT {
            rocks.push_str(&empty);
            new_line.push(rocks);
        } else if direction == Direction::RIGHT {
            empty.push_str(&rocks);
            new_line.push(empty);
        }
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

fn move_rocks(plane: Vec<String>, direction: Direction) -> Vec<String> {
    let mut moved = Vec::new();
    if direction == Direction::TOP {
        for line in transpose_reverse_clockwise(plane) {
            moved.push(move_rocks_line_to(line, Direction::LEFT));
        }
        return transpose_clockwise(moved.clone());
    } else if direction == Direction::LEFT {
        // West
        for line in plane {
            moved.push(move_rocks_line_to(line, Direction::LEFT));
        }
    } else if direction == Direction::BOTTOM {
        for line in transpose_reverse_clockwise(plane) {
            moved.push(move_rocks_line_to(line, Direction::RIGHT));
        }
        return transpose_clockwise(moved.clone());
    } else if direction == Direction::RIGHT {
        for line in plane {
            moved.push(move_rocks_line_to(line, Direction::RIGHT));
        }
    }

    return moved;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0u32;
    // let mut plane = build_plane(input);
    // println!("{:?}", plane);
    // plane = transpose_reverse_clockwise(plane);
    // println!("{:?}", plane);
    // plane = transpose_clockwise(plane);
    // println!("{:?}", plane);

    let plane = move_rocks(build_plane(input).clone(), Direction::TOP);
    // println!("PLANE: {:?}", plane);
    for line in transpose_reverse_clockwise(plane) {
        // println!("OLD {:?}", line);
        // let new_line = move_rocks_line_to(line, Direction::LEFT);
        // println!("NEW {:?}", new_line);
        sum += get_rocks_weight_for_line(line);
    }

    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut plane = build_plane(input);
    // println!("{:?}", plane);
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut i = 0usize;
    while i < 1_000_000_000 - 1 {
        plane = move_rocks(plane, Direction::TOP);
        plane = move_rocks(plane, Direction::LEFT);
        plane = move_rocks(plane, Direction::BOTTOM);
        plane = move_rocks(plane, Direction::RIGHT);

        let key = plane.join("");
        let found = map.get(&key);
        if found.is_some() {
            let size = i - found.unwrap();
            println!(
                "CYCLE {:?} found -> {:?} skip size -> {:?}",
                i,
                found.unwrap(),
                size
            );
            // repeat size to the end...
            let pending = 1_000_000_000 - i;
            println!(
                "TIMES {:?} => skip total {:?}",
                pending / size,
                (pending / size) * size
            );
            i += (pending / size) * size;
            map.clear();
        } else {
            // println!("{:?}", i);
            map.insert(key, i);
            i += 1;
        }
    }

    let mut sum = 0u32;
    for line in transpose_reverse_clockwise(plane) {
        sum += get_rocks_weight_for_line(line);
    }

    return Some(sum);
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
        assert_eq!(result, Some(64u32));
    }
}
