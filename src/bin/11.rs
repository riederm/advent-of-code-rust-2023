use std::{usize, collections::HashMap};

advent_of_code::solution!(11);

type id = usize;

struct Galaxy {
    x : i32,
    y : i32,
}

fn parse_galaxies(input: &str) -> HashMap<id, Galaxy> {
    let mut id = 1usize;
    let mut galaxies = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.insert(id, Galaxy { x: x as i32, y: y as i32 });
                id += 1;
            }
        }
    }
    galaxies
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut galaxies = parse_galaxies(input);
    let (max_x, max_y) = galaxies.values().fold((0, 0), |(max_x, max_y), galaxy| {
        (max_x.max(galaxy.x), max_y.max(galaxy.y))
    });

    let cols = (0..=max_x)
        .map(|tx| galaxies.iter().filter(|(_,galaxy)| galaxy.x == tx)
        .map(|(id, _)| *id)
        .collect::<Vec<_>>()).collect::<Vec<_>>();
    let rows = (0..=max_y)
        .map(|ty| galaxies.iter().filter(|(_, galaxy)| galaxy.y == ty)
        .map(|(id, _)| *id)
        .collect::<Vec<_>>()).collect::<Vec<_>>();

    let empty_cols = cols.iter().enumerate().filter(|(_, col)| col.is_empty()).map(|(tx, _)| tx).collect::<Vec<_>>();
    let empty_rows = rows.iter().enumerate().filter(|(_, row)| row.is_empty()).map(|(ty, _)| ty).collect::<Vec<_>>();

    for (transitive, tx) in empty_cols.iter().enumerate() {
        let galaxies_in_col = cols.get(*tx).unwrap();
        for id in  galaxies_in_col.iter() {
            galaxies.get_mut(id).expect("galaxy not found").x += 1 + transitive as i32;
        }
    }

    for (transitive, ty) in empty_rows.iter().enumerate() {
        let galaxies_in_row = rows.get(*ty).unwrap();
        for id in  galaxies_in_row.iter() {
            galaxies.get_mut(id).expect("galaxy not found").y += 1 + transitive as i32;
        }
    }

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
