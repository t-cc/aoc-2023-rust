advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0u32;

    for line in input.lines() {
        let line_parts = line.split(':');
        if line_parts.clone().count() == 2 {
            let line_vec = line_parts.collect::<Vec<&str>>();
            let game_index = line_vec[0]
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let mut has_invalid_game = false;
            for game in line_vec[1].trim().split(';') {
                for cube in game.trim().split(',') {
                    let cube_data = cube.trim().split_whitespace().collect::<Vec<&str>>();
                    let number = cube_data.first().unwrap().parse::<i32>().unwrap();
                    let color = cube_data.last().unwrap().to_string();
                    if (color == "red" && number > 12)
                        || (color == "green" && number > 13)
                        || (color == "blue" && number > 14)
                    {
                        has_invalid_game = true;
                        break;
                    }
                }
            }
            if !has_invalid_game {
                sum += game_index as u32;
            }
        }
    }
    return sum.into();
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
        assert_eq!(result, Some(8u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
