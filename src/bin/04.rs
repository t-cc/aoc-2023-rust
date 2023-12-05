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

fn get_wining_cards(mut total_cards: usize, cards_with_points: &Vec<usize>, card_index: usize) {
    if card_index.clone() >= cards_with_points.len() {
        return;
    }
    let points = cards_with_points[card_index].clone();
    if points > 0 {
        for i in card_index..(card_index + points) {
            get_wining_cards(total_cards, cards_with_points, i);
        }
        total_cards = total_cards + 1usize;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards_with_points: Vec<usize> = Vec::new();
    let cards_num = input.lines().count();

    for line in input.lines() {
        let mut points = 0usize;
        let line_parts = line.split(':');
        if line_parts.clone().count() == 2 {
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
                // println!("wining: {:?}", wining);
                // println!("game:   {:?}", game);
                for win in wining {
                    points += game.iter().find(|&&number| number == win).iter().len()
                }
                // println!("points  {:?}", points);
            }
        }
        cards_with_points.push(points)
    }

    /*
    01 --> Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 (+5)
    02 --> Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19 (+2)
    04 --> Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1 (+2)
    08 --> Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83 (+1)
    14 --> Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    01 --> Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    */
    // println!("START {:?}", cards_with_points);
    let mut cards_total_points: Vec<usize> = vec![1usize; cards_num];
    for index in 0..cards_num {
        let card_points = cards_with_points[index];
        for i in index + 1..index + 1 + card_points {
            if i < cards_num {
                cards_total_points[i] += cards_total_points[index];
            }
        }
        // println!("PROCESS {:?}", cards_total_points);
    }
    // println!("END {:?}", cards_total_points);

    let total_count = cards_total_points
        .iter()
        .cloned()
        .reduce(|a, b| a + b)
        .unwrap();
    return (total_count as u32).into();
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
        assert_eq!(result, Some(30u32));
    }
}
