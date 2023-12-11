use std::{collections::HashMap, usize};

advent_of_code::solution!(11);

type ID = usize;

#[derive(Debug)]
struct Galaxy {
    x: i32,
    y: i32,
}

type MultiVec<T> = Vec<Vec<T>>;
struct ParseGalaxiesResult(HashMap<ID, Galaxy>, MultiVec<usize>, MultiVec<usize>);

fn parse_galaxies(input: &str) -> ParseGalaxiesResult {
    let mut id = 1usize;
    let mut galaxies = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                galaxies.insert(
                    id,
                    Galaxy {
                        x: x as i32,
                        y: y as i32,
                    },
                );
                id += 1;
            }
        }
    }
    let (max_x, max_y) = galaxies.values().fold((0, 0), |(max_x, max_y), galaxy| {
        (max_x.max(galaxy.x), max_y.max(galaxy.y))
    });

    let cols = (0..=max_x)
        .map(|tx| {
            galaxies
                .iter()
                .filter(|(_, galaxy)| galaxy.x == tx)
                .map(|(id, _)| *id)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let rows = (0..=max_y)
        .map(|ty| {
            galaxies
                .iter()
                .filter(|(_, galaxy)| galaxy.y == ty)
                .map(|(id, _)| *id)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    ParseGalaxiesResult(galaxies, cols, rows)
}

fn get_path(from: ID, to: ID, galaxies: &HashMap<ID, Galaxy>) -> u32 {
    let from_galaxy = galaxies.get(&from).expect("galaxy not found");
    let to_galaxy = galaxies.get(&to).expect("galaxy not found");
    (from_galaxy.x - to_galaxy.x).unsigned_abs() + (from_galaxy.y - to_galaxy.y).unsigned_abs()
}

pub fn part_one(input: &str) -> Option<u32> {
    let ParseGalaxiesResult(mut galaxies, cols, rows) = parse_galaxies(input);

    expand_universe(cols, rows, &mut galaxies, 1);

    let mut sum = 0;
    for start in 1..=galaxies.len() {
        for end in (start + 1)..=galaxies.len() {
            let path = get_path(start, end, &galaxies);
            sum += path;
        }
    }
    Some(sum)
}

fn expand_universe(
    cols: Vec<Vec<usize>>,
    rows: Vec<Vec<usize>>,
    galaxies: &mut HashMap<usize, Galaxy>,
    factor: i32,
) {
    let empty_cols = cols
        .iter()
        .enumerate()
        .filter(|(_, col)| col.is_empty())
        .map(|(tx, _)| tx)
        .collect::<Vec<_>>();
    let empty_rows = rows
        .iter()
        .enumerate()
        .filter(|(_, row)| row.is_empty())
        .map(|(ty, _)| ty)
        .collect::<Vec<_>>();

    for (_transitive, tx) in empty_cols.iter().enumerate() {
        cols.iter()
            .skip(*tx)
            .flatten()
            .for_each(|id| galaxies.get_mut(id).expect("galaxy not found").x += factor)
    }

    for (_transitive, ty) in empty_rows.iter().enumerate() {
        rows.iter()
            .skip(*ty)
            .flatten()
            .for_each(|id| galaxies.get_mut(id).expect("galaxy not found").y += factor)
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    solve_two(input, 1_000_000 - 1) // -1 to account for the existing empty row/col
}

fn solve_two(input: &str, factor: i32) -> Option<u64> {
    let ParseGalaxiesResult(mut galaxies, cols, rows) = parse_galaxies(input);

    expand_universe(cols, rows, &mut galaxies, factor);

    let mut sum = 0u64;
    for start in 1..=galaxies.len() {
        for end in (start + 1)..=galaxies.len() {
            let path = get_path(start, end, &galaxies);
            sum += path as u64;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = solve_two(
            &advent_of_code::template::read_file("examples", DAY),
            10 - 1,
        );
        assert_eq!(result, Some(1030));

        let result = solve_two(
            &advent_of_code::template::read_file("examples", DAY),
            100 - 1,
        );
        assert_eq!(result, Some(8410));
    }
}
