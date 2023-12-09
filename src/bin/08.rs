use std::collections::HashMap;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut steps = 0u32;

    let mut lines = input.lines();
    let directions: Vec<char> = lines.next().unwrap().chars().collect();

    let mut network: HashMap<String, [String; 2]> = HashMap::new();
    let mut start_point: String = "AAA".to_string();
    for line in lines.into_iter() {
        if !line.is_empty() {
            println!("LINE {:?}", line);
            let mut parts = line.split('=');
            let key = parts.next().unwrap().trim().to_string();
            // if start_point.is_empty() {
            //     start_point = key.clone();
            // }
            let values_str = parts.next().unwrap().replace(['(', ')'], "");
            let mut values_list = values_str.split(',').map(|v| v.trim().to_string());
            let values_to_insert = [values_list.next().unwrap(), values_list.next().unwrap()];
            // println!("VALUES {:?}", values_list);
            network.insert(key, values_to_insert);
        }
    }
    println!("MAP -> {:?}", network);

    if !network.get(&*start_point).is_some() {
        // Avoid problems on part 2 testing
        return steps.into();
    }

    let mut index = 0usize;
    let limit = directions.len();
    while start_point != "ZZZ" {
        let value = network.get(&*start_point);
        let direction = directions[index];
        println!("DIR {:?}: {:?} -> {:?}", direction, start_point, value);
        if value.is_some() {
            let mut next_directions = value.unwrap().iter();
            if direction == 'L' {
                start_point = next_directions.next().unwrap().clone();
            } else {
                start_point = next_directions.last().unwrap().clone();
            }
        }
        println!("NEXT -> {:?}", start_point);
        steps += 1;
        index += 1;
        if index == limit {
            index = 0usize
        }
    }

    return steps.into();
}

fn max_comun_divisor(a: u64, b: u64) -> u64 {
    let mut temp;
    let mut x = a.clone();
    let mut y = b.clone();

    while (y != 0) {
        temp = y;
        y = x % y;
        x = temp;
    }
    return x;
}
fn min_comun_multiplo(a: u64, b: u64) -> u64 {
    return a * b / max_comun_divisor(a, b);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let directions: Vec<char> = lines.next().unwrap().chars().collect();

    let mut network: HashMap<String, [String; 2]> = HashMap::new();
    let mut start_point_list: Vec<String> = Vec::new();
    for line in lines.into_iter() {
        if !line.is_empty() {
            println!("LINE {:?}", line);
            let mut parts = line.split('=');
            let key = parts.next().unwrap().trim().to_string();
            // if start_point.is_empty() {
            //     start_point = key.clone();
            // }
            let values_str = parts.next().unwrap().replace(['(', ')'], "");
            let mut values_list = values_str.split(',').map(|v| v.trim().to_string());
            let values_to_insert = [values_list.next().unwrap(), values_list.next().unwrap()];
            // println!("VALUES {:?}", values_list);
            network.insert(key.clone(), values_to_insert);
            if key.ends_with('A') {
                start_point_list.push(key);
            }
        }
    }
    println!("MAP -> {:?}", network);
    println!("STARTS -> {:?}", start_point_list);

    let mut step_list: Vec<u64> = Vec::new();

    for mut start_point in start_point_list {
        let mut steps = 0u32;
        let mut index = 0usize;
        let limit = directions.len();

        while !start_point.ends_with('Z') {
            let value = network.get(&*start_point);
            let direction = directions[index];
            // println!("DIR {:?}: {:?} -> {:?}", direction, start_point, value);
            if value.is_some() {
                let mut next_directions = value.unwrap().iter();
                if direction == 'L' {
                    start_point = next_directions.next().unwrap().clone();
                } else {
                    start_point = next_directions.last().unwrap().clone();
                }
            }
            // println!("NEXT -> {:?}", start_point);
            steps += 1;
            index += 1;
            if index == limit {
                index = 0usize
            }
        }
        println!("STEP {:?} - {:?}", start_point, steps);
        step_list.push(steps as u64);
    }

    let total: Option<u64> = step_list
        .into_iter()
        .reduce(|a, b| min_comun_multiplo(a, b));

    return total.into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6u32));
    }
}
