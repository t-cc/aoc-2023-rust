advent_of_code::solution!(11);

fn get_galaxy_positions(input: &str) -> Vec<[i32; 2]> {
    let mut galaxies: Vec<[i32; 2]> = Vec::new();
    let mut y = 0i32;
    for line in input.lines() {
        let mut x = 0i32;
        for char in line.chars() {
            if char != '.' {
                galaxies.push([x, y]);
            }
            x += 1;
        }
        y += 1;
    }

    return galaxies;
}

fn expand_universe(universe: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    let mut expanded: Vec<[i32; 2]> = universe.clone();

    let with = universe.iter().map(|a| a[0]).max().unwrap();
    let height = universe.iter().map(|a| a[1]).max().unwrap();

    let mut delta_x = 0i32;
    for x in 0..with {
        if !universe.iter().find(|position| position[0] == x).is_some() {
            delta_x += 1;
            expanded
                .iter_mut()
                .filter(|galaxy| galaxy[0] >= x + delta_x)
                .for_each(|galaxy| *galaxy = [galaxy[0] + 1, galaxy[1]]);
        }
    }
    let mut delta_y = 0i32;
    for y in 0..height {
        if !universe.iter().find(|position| position[1] == y).is_some() {
            delta_y += 1;
            expanded
                .iter_mut()
                .filter(|galaxy| galaxy[1] >= y + delta_y)
                .for_each(|galaxy| *galaxy = [galaxy[0], galaxy[1] + 1]);
        }
    }
    return expanded;
}

fn get_galaxy_pairs(galaxies: Vec<[i32; 2]>) -> Vec<[[i32; 2]; 2]> {
    let mut pairs: Vec<[[i32; 2]; 2]> = Vec::new();

    let length = galaxies.len();
    for (i, galaxy_a) in galaxies.iter().enumerate() {
        if i < length {
            for galaxy_b in &galaxies[i + 1..length] {
                pairs.push([*galaxy_a, *galaxy_b]);
            }
        }
    }

    return pairs;
}

fn get_shortest_paths(galaxy_pairs: Vec<[[i32; 2]; 2]>) -> Vec<u32> {
    let mut paths: Vec<u32> = Vec::new();
    for pair in galaxy_pairs {
        let a = pair[0];
        let b = pair[1];
        paths.push(((a[0] - b[0]).abs() + (a[1] - b[1]).abs()) as u32)
    }

    return paths;
}

pub fn part_one(input: &str) -> Option<u32> {
    let galaxies = get_galaxy_positions(input);
    // println!("GALAXIES -> {:?}", galaxies);
    let expanded = expand_universe(galaxies);
    // println!("EXPANDED -> {:?}", expanded);
    let pairs = get_galaxy_pairs(expanded);
    // println!("GALAXY PAIRS -> {:?}", pairs);
    let mut distances = get_shortest_paths(pairs);
    println!("GALAXY DISTANCES -> {:?}", distances);

    return distances.iter().cloned().reduce(|a, b| a + b);
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
        assert_eq!(result, Some(374u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
