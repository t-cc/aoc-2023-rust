advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0i32;

    for line in input.lines() {
        let line_parts = line.split(':');
        if line_parts.clone().count() == 2 {
            let mut points = 0i32;
            let data = line_parts.last().unwrap();
            let mut data_parts = data.split('|');
            if data_parts.clone().count() == 2 {
                let wining: Vec<i32> = data_parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|c| c.trim())
                    .filter(|&c| c.len() != 0)
                    .map(|c| c.parse::<i32>().unwrap())
                    .collect();

                let game: Vec<i32> = data_parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|c| c.trim())
                    .filter(|&c| c.len() != 0)
                    .map(|c| c.parse::<i32>().unwrap())
                    .collect();
                println!("{:?}", wining);
                println!("{:?}", game);
                for win in wining {
                    let found = game.iter().find(|&&number| number == win).iter().len();
                    if found > 0 {
                        if points == 0 {
                            points = 1;
                        } else {
                            points = 2 * points;
                        }
                    }
                }
                sum += points;
            }
        }
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
        assert_eq!(result, Some(13u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
