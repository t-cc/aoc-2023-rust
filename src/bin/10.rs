advent_of_code::solution!(10);

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Vec::new();
    let mut s_point = Point { x: 0i32, y: 0i32 };

    for (y_, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x_, char) in line.chars().enumerate() {
            let x = x_ as i32;
            let y = y_ as i32;
            // println!("{:?},{:?} => {:?}", x, y, char);
            if char == 'S' {
                s_point = Point { x, y };
                row.push(vec![]);
            } else if char == '-' {
                row.push(vec![Point { x: x - 1, y }, Point { x: x + 1, y }]);
            } else if char == '|' {
                row.push(vec![Point { x, y: y - 1 }, Point { x, y: y + 1 }])
            } else if char == 'L' {
                row.push(vec![Point { x, y: y - 1 }, Point { x: x + 1, y }])
            } else if char == 'J' {
                row.push(vec![Point { x, y: y - 1 }, Point { x: x - 1, y }])
            } else if char == '7' {
                row.push(vec![Point { x, y: y + 1 }, Point { x: x - 1, y }])
            } else if char == 'F' {
                row.push(vec![Point { x, y: y + 1 }, Point { x: x + 1, y }])
            } else {
                row.push(vec![]);
            }
        }
        map.push(row);
    }

    println!("MAP:     {:?}", map);
    println!("S POINT: {:?}", s_point);
    println!("------");
    let valid_start_movements = vec![
        Point {
            x: s_point.x + 1,
            y: s_point.y,
        },
        Point {
            x: s_point.x,
            y: s_point.y - 1,
        },
        Point {
            x: s_point.x,
            y: s_point.y + 1,
        },
        Point {
            x: s_point.x - 1,
            y: s_point.y,
        },
    ];

    let y_size = map.iter().count();
    let x_size = map.iter().nth(0).unwrap().len();
    let mut distances = vec![vec![0; x_size]; y_size];

    for start_movement in valid_start_movements {
        let mut count = 0u32;
        let mut previous_point = s_point;
        let mut current_point = start_movement;
        let mut has_movements = true;
        while has_movements {
            println!("C POINT {:?}", current_point);
            if current_point.y < map.len() as i32 && current_point.y >= 0i32 {
                let row = map.iter().nth(current_point.y as usize).unwrap();
                // println!("row: {:?}", row);
                if current_point.x < row.len() as i32 && current_point.x >= 0i32 {
                    let possible_movements: Vec<Point> =
                        row.iter().nth(current_point.x as usize).unwrap().clone();
                    println!("movements: {:?}", possible_movements);
                    // println!("POINT: {:?}", previous_point);
                    if possible_movements.len() > 0 {
                        for movement in possible_movements {
                            if movement.x != previous_point.x || movement.y != previous_point.y {
                                let saved =
                                    distances[previous_point.y as usize][previous_point.x as usize];
                                if saved == 0 || saved > count {
                                    distances[previous_point.y as usize]
                                        [previous_point.x as usize] = count;
                                }

                                count += 1;
                                previous_point = current_point;
                                current_point = movement;
                                println!("NEXT {:?}", movement);
                                break;
                            }
                        }
                    } else {
                        has_movements = false;
                    }
                } else {
                    has_movements = false;
                }
            } else {
                has_movements = false;
            }
        }
    }

    println!(
        "DISTANCES {:?}",
        (distances.iter().flatten().filter(|&x| x > &0).count() + 1) / 2
    );
    println!("------");
    // return distances.iter().flatten().max().copied();
    return Some(((distances.iter().flatten().filter(|&x| x > &0).count() + 1) / 2) as u32);
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
        assert_eq!(result, Some(14u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
