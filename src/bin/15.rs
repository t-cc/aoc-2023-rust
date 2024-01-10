advent_of_code::solution!(15);

fn get_value(chars: &str) -> u32 {
    let mut value = 0u32;
    for char in chars.chars() {
        let ascii = char as u32;
        // println!("{:?}", char);
        value += ascii;
        value = value * 17;
        value = value % 256;
    }
    return value;
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0u32;

    let line: Vec<&str> = input.lines().next().unwrap().split(',').collect();
    // println!("{:?}", line);
    for part in &line {
        sum += get_value(part);
    }
    return Some(sum);
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal: u32,
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes : Vec<Vec<Lens>> = vec![vec![]; 256];

    let line: Vec<&str> = input.lines().next().unwrap().split(',').collect();
    for part in line {
        let has_equal = part.chars().find(|&c| c == '=').is_some();
        let mut info = Vec::new();
        let mut focal_length = 0u32;
        if has_equal {
            info = part.split('=').collect();
            focal_length = info.last()?.parse::<u32>().unwrap();
        } else {
            info = part.split('-').collect();
        }
        let label_str = info.first().unwrap();
        let box_number = get_value(label_str);
        let label = label_str.to_string();
        // println!("Label {:?} -> value {:?}, focus {:?}", label, box_number, focal_length);


        let existing = boxes[box_number as usize].to_vec();
        let lens_index =  existing.iter().position(|l| l.label == label);
        if has_equal {
            if lens_index.is_some() {
                // replace focal
                boxes[box_number as usize][lens_index.unwrap()].focal = focal_length;

            } else {
                // append
                boxes[box_number as usize].push(Lens {
                    label: label.to_string(),
                    focal: focal_length
                })
            }
        } else if lens_index.is_some() {
            // remove the relevant lens in the current box
            boxes[box_number as usize] = existing.iter().filter(|l| l.label != label ).cloned().collect();
        }
        // println!("BOXES: {:?}", boxes);
    }


    let mut sum = 0u32;

    for i in 0..boxes.len() {
        let box_count = (i + 1) as u32;
        for j in 0..boxes[i].len() {
            let slot_count = (j +1) as u32;
            sum += box_count * slot_count * boxes[i][j].focal;
        }

    }
    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145u32));
    }
}
