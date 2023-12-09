use std::cmp::Ordering;
advent_of_code::solution!(7);

#[derive(Debug)]
struct CardFrequency {
    card: char,
    frequency: u32,
}

#[derive(Debug)]
struct Hand {
    value: u32,
    cards: String,
    cards_frequency: Vec<CardFrequency>,
}

const CARD_SCORES: &str = "AKQJT98765432";

const CARD_SCORES_JOKER: &str = "AKQT98765432J";

fn sort_cards(
    freq_a: &Vec<CardFrequency>,
    cards_a: &String,
    freq_b: &Vec<CardFrequency>,
    cards_b: &String,
    joker: bool,
) -> Ordering {
    if freq_a[0].frequency > freq_b[0].frequency {
        return Ordering::Greater;
    } else if freq_a[0].frequency < freq_b[0].frequency {
        return Ordering::Less;
    } else {
        if freq_a.len() > 1 && freq_b.len() > 1 {
            if freq_a[1].frequency > freq_b[1].frequency {
                return Ordering::Greater;
            } else if freq_a[1].frequency < freq_b[1].frequency {
                return Ordering::Less;
            }
        }
    }

    for (i, char_a) in cards_a.chars().enumerate() {
        let char_b = cards_b.chars().nth(i).unwrap();
        if char_a != char_b {
            if joker {
                let index_a = CARD_SCORES_JOKER.chars().position(|c| c == char_a).unwrap();
                let index_b = CARD_SCORES_JOKER.chars().position(|c| c == char_b).unwrap();
                return if index_a < index_b {
                    Ordering::Greater
                } else {
                    Ordering::Less
                };
            } else {
                let index_a = CARD_SCORES.chars().position(|c| c == char_a).unwrap();
                let index_b = CARD_SCORES.chars().position(|c| c == char_b).unwrap();
                return if index_a < index_b {
                    Ordering::Greater
                } else {
                    Ordering::Less
                };
            }
        }
    }

    return Ordering::Equal;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut car_list: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let mut values = line.split_whitespace();
        let cards: String = values.next().unwrap().parse().unwrap();
        let value: u32 = values.next().unwrap().parse::<u32>().unwrap();
        let mut cards_frequency: Vec<CardFrequency> = Vec::new();
        for card in cards.chars() {
            let index = cards_frequency.iter().position(|f| f.card == card);
            if index.is_some() {
                cards_frequency[index.unwrap()].frequency += 1;
            } else {
                cards_frequency.push(CardFrequency { card, frequency: 1 })
            }
        }
        cards_frequency.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        println!("Cad freq (sorted): {:?}", cards_frequency);
        println!("------");
        car_list.push(Hand {
            cards,
            value,
            cards_frequency,
        })
    }

    // println!("Raw card list {:?}", car_list);
    // println!("------");
    car_list.sort_by(|a, b| {
        sort_cards(
            &a.cards_frequency,
            &a.cards,
            &b.cards_frequency,
            &b.cards,
            false,
        )
    });
    // println!("sorted card list {:?}", car_list);

    let mut sum = 0u32;
    let card_list_len = car_list.len();
    for (i, hand) in car_list.iter().enumerate() {
        sum += (i as u32 + 1) * hand.value;
        println!(
            "HAND {:?} -> vale ({:?}) -> rank ({:?})",
            hand.cards,
            hand.value,
            i + 1
        );
    }

    return sum.into();
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0u32;

    let mut car_list: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let mut values = line.split_whitespace();
        let cards: String = values.next().unwrap().parse().unwrap();
        let value: u32 = values.next().unwrap().parse::<u32>().unwrap();
        let mut cards_frequency: Vec<CardFrequency> = Vec::new();
        let mut jokers = 0u32;
        if cards == "JJJJJ" {
            cards_frequency.push(CardFrequency {
                card: 'J',
                frequency: 5,
            })
        } else {
            for card in cards.chars() {
                if card == 'J' {
                    jokers += 1;
                } else {
                    let index = cards_frequency.iter().position(|f| f.card == card);
                    if index.is_some() {
                        cards_frequency[index.unwrap()].frequency += 1;
                    } else {
                        cards_frequency.push(CardFrequency { card, frequency: 1 })
                    }
                }
            }
            cards_frequency.sort_by(|a, b| b.frequency.cmp(&a.frequency));
            if jokers > 0 {
                cards_frequency[0].frequency += jokers;
                println!("JOKERS: ")
                // cards_for_sort = cards_for_sort.replace('J', &cards_frequency[0].card.to_string())
            }
        }
        println!("Cad freq (sorted): {:?}", cards_frequency);
        println!("------");
        car_list.push(Hand {
            cards,
            value,
            cards_frequency,
        })
    }

    // println!("Raw card list {:?}", car_list);
    // println!("------");
    car_list.sort_by(|a, b| {
        sort_cards(
            &a.cards_frequency,
            &a.cards,
            &b.cards_frequency,
            &b.cards,
            true,
        )
    });
    // println!("sorted card list {:?}", car_list);

    for (i, hand) in car_list.iter().enumerate() {
        sum += (i as u32 + 1) * hand.value;
        /*
        println!(
            "HAND {:?} -> vale ({:?}) -> rank ({:?})",
            hand.cards,
            hand.value,
            i + 1
        );

         */
    }

    return sum.into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905u32));
    }
}
