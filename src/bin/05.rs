use std::ptr::null;
advent_of_code::solution!(5);

#[derive(Debug)]
struct SeedMap {
    min: i64,
    max: i64,
    sum: i64,
}
pub fn part_one(input: &str) -> Option<u32> {
    let seed_list: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|a| a.parse::<i64>().unwrap())
        .collect();
    //println!("{:?}", seed_list);

    let mut start_map = false;
    // let mut current_map: HashMap<u32, u32> = HashMap::new();
    let mut current_map: Vec<SeedMap> = Vec::new();
    let mut map_list = Vec::new();
    for line in input.lines() {
        if line.ends_with("map:") {
            start_map = true;
            if !current_map.is_empty() {
                map_list.push(current_map);
                current_map = Vec::new();
            }
            // println!("Header {:?}", line)
        } else if start_map && line != "" {
            println!("Process -> {:?}", line);
            let numbers: Vec<i64> = line
                .trim()
                .split_whitespace()
                .map(|a| a.parse::<i64>().unwrap())
                .collect();
            if numbers.len() == 3 {
                let start_in = numbers[1];
                let start_out = numbers[0];
                let length = numbers[2];
                current_map.push(SeedMap {
                    min: start_in,
                    max: start_in + length,
                    sum: start_out - start_in,
                });
            }
            // println!("{:?}", current_map.len())
        }
    }
    if !current_map.is_empty() {
        map_list.push(current_map);
    }
    // println!("MAP LIST{:?}", map_list);

    let mut out_list = Vec::new();
    for seed in seed_list {
        let mut start = seed;
        // println!("SEED -> {:?}", start);
        for map_group in map_list.iter() {
            // iter().rev().find() parece que no funca
            let map_wrp = map_group
                .iter()
                .rev()
                .find(|map| map.min <= start && map.max >= start);

            if map_wrp.is_some() {
                let map = map_wrp.unwrap();
                // println!(
                //     "TRANSFORMACIÓN ({:?}) --> + {:?} ---> {:?}",
                //     start,
                //     map.sum,
                //     start + map.sum
                // );
                start = start + map.sum;
            }
            // println!("MAP -> {:?}", start);
        }
        out_list.push(start)
    }
    let out = out_list.iter().min().unwrap().clone();
    println!("{:?} -> {:?}", out_list, out);
    return Some(out as u32);
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
        assert_eq!(result, Some(35u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
