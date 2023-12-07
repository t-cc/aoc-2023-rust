advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|a| a.parse::<u32>().unwrap())
        .collect();

    let distances: Vec<u32> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|a| a.parse::<u32>().unwrap())
        .collect();

    println!("{:?}", times);
    println!("{:?}", distances);

    let mut sums: Vec<u32> = Vec::new();
    for i in 0..times.len() {
        let mut best_counts = 0u32;
        let time = times[i];
        let record = distances[i];
        for speed in 0..time {
            let time_left = time - speed;
            if time_left * speed >= record {
                best_counts += 1;
            }
        }
        sums.push(best_counts);
    }
    println!("SUM -> {:?}", sums);
    return sums.iter().cloned().reduce(|a, b| a * b);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut best_counts = 0u64;

    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    println!("{:?}", time);

    let record: u64 = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    println!("{:?}", record);

    for speed in 0..time {
        let time_left = time - speed;
        if time_left * speed >= record {
            best_counts += 1;
        }
    }

    println!("SUM -> {:?}", best_counts);
    return best_counts.into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503u32));
    }
}
