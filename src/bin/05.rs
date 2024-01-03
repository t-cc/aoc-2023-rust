advent_of_code::solution!(5);

#[derive(Debug)]
struct SeedMap {
    min: i64,
    max: i64,
    sum: i64,
}

fn get_seed_list(line: Option<&str>) -> Vec<i64> {
    return line
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|a| a.parse::<i64>().unwrap())
        .collect();
}

fn build_mapper(mut lines: std::str::Lines<'_>) -> Vec<Vec<SeedMap>> {
    let mut mapper = Vec::new();

    let mut start_map = false;
    let mut current_map: Vec<SeedMap> = Vec::new();
    while let Some(line) = lines.next() {
        if line.ends_with("map:") {
            start_map = true;
            if !current_map.is_empty() {
                mapper.push(current_map);
                current_map = Vec::new();
            }
            // println!("Header {:?}", line)
        } else if start_map && line != "" {
            // println!("Process -> {:?}", line);
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
        mapper.push(current_map);
    }
    return mapper;
}

fn get_location_for_sed(mapper: &Vec<Vec<SeedMap>>, seed: i64) -> i64 {
    let mut start = seed;
    // println!("SEED -> {:?}", start);
    for map_group in mapper.iter() {
        let map_wrp = map_group
            .iter()
            .rfind(|map| map.min <= start && map.max >= start);

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
    }
    return start;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let seed_list: Vec<i64> = get_seed_list(lines.next());
    // println!("{:?}", seed_list);

    let mapper = build_mapper(lines);
    let mut out_list = Vec::new();
    for seed in seed_list {
        out_list.push(get_location_for_sed(&mapper, seed))
    }
    return Some(out_list.iter().min().unwrap().clone() as u32);
}

fn get_min_location_for_range(mapper: &Vec<Vec<SeedMap>>, seed_min: i64, seed_max: i64) -> i64 {
    let mut start = seed_min;
    // println!("SEED -> {:?}", start);
    for map_group in mapper.iter() {
        let map_wrp = map_group
            .iter()
            .rfind(|map| map.min <= start && map.max >= start);

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
    }
    return start;
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let seed_list: Vec<i64> = get_seed_list(lines.next());
    println!("{:?}", seed_list);

    let mapper = build_mapper(lines);
    let mut out_list = Vec::new();
    for i in 0..seed_list.len() / 2 {
        let start = seed_list[i * 2];
        let end = start + seed_list[(i * 2) + 1];
        println!("SEED {:?}+{:?}", start, seed_list[(i * 2) + 1]);
        for seed in start..end {
            out_list.push(get_location_for_sed(&mapper, seed))
        }
    }
    return Some(out_list.iter().min().unwrap().clone() as u32);
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
        assert_eq!(result, Some(46u32));
    }
}
