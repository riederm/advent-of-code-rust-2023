advent_of_code::solution!(10);

use std::collections::HashMap;
use std::collections::HashSet;

use lazy_static::lazy_static;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vec2D {
    x: i32,
    y: i32,
}

impl Vec2D {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

lazy_static! {
    static ref N: Vec2D = Vec2D::new(0, -1);
    static ref S: Vec2D = Vec2D::new(0, 1);
    static ref W: Vec2D = Vec2D::new(-1, 0);
    static ref E: Vec2D = Vec2D::new(1, 0);
    static ref EMPTY: Vec<Vec2D> = vec![];
    static ref TILES: HashMap<char, Vec<Vec2D>> = HashMap::from([
        ('|', vec![*N, *S]),
        ('-', vec![*E, *W]),
        ('L', vec![*N, *E]),
        ('J', vec![*N, *W]),
        ('7', vec![*S, *W]),
        ('F', vec![*S, *E])
    ]);
}

struct Map<'i> {
    lines: Vec<&'i str>,
}

impl<'i> Map<'i> {
    fn get_connectors(&self, position: &Vec2D) -> &Vec<Vec2D> {
        if position.y < 0 || position.y >= self.lines.len() as i32 {
            return &EMPTY;
        }
        let row = self.lines.get(position.y as usize).unwrap();

        if position.x < 0 || position.x >= row.len() as i32 {
            return &EMPTY;
        }
        let character = row.as_bytes()[position.x as usize] as char;
        TILES.get(&character).unwrap_or(&EMPTY)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map {
        lines: input.lines().collect(),
    };

    let path = collect_path(&map);
    Some((path.len() / 2) as u32)
}

fn collect_path(map: &Map<'_>) -> HashSet<Vec2D> {
    let start = map
        .lines
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.find('S').map(|x| Vec2D::new(x as i32, y as i32)))
        .expect("no start point found");

    let mut path = HashSet::new();
    path.insert(start);

    // find next step
    let directions = vec![*N, *S, *E, *W];
    let mut pos = start;

    // help with first step from Start
    let step = directions
        .iter()
        .find(|dir| {
            let connectors = map.get_connectors(&pos.add(dir));
            // see if it would connect back to us
            connectors.iter().any(|d| d.x == -dir.x && d.y == -dir.y)
        })
        .expect("no first step found");

    pos = pos.add(step);
    path.insert(pos);
    while !path.len() > 1 && pos != start {
        //find connected tile
        let connectors = map.get_connectors(&pos);
        pos = connectors
            .iter()
            .map(|dir| pos.add(dir)) // connected neighbours
            .find(|next_pos| {
                // find the one that is not in path
                !path.contains(next_pos) || (next_pos == &start && path.len() > 2)
            })
            .unwrap_or_else(|| panic!("no next step found for {:?}", pos));

        path.insert(pos);
    }
    path
}

fn count_walls(map: &Map<'_>, path: &HashSet<Vec2D>, x: i32, y: i32, delta_x: i32) -> i32 {
    let line = map.lines[y as usize];

    let mut count = 0;
    let mut x = x;
    while x >= 0 && x < line.len() as i32 {
        if path.contains(&Vec2D::new(x, y)) {
            let mut char = map.lines[y as usize].as_bytes()[x as usize] as char;
            if char == 'S' {
                char = '|'; // todo specific for my data!
            }
            count += match char {
                '|' | 'L' | 'J' => 1, // LJ count both, but the %2 will fix that
                _ => 0,
            };
        }
        x += delta_x;
    }
    count
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map {
        lines: input.lines().collect(),
    };

    let path = collect_path(&map);
    //repair the start

    let mut count = 0u32;
    //horizontal ray casting
    for (y, line) in map.lines.iter().enumerate() {
        for (x, _) in line.chars().enumerate() {
            if !path.contains(&Vec2D::new(x as i32, y as i32)) {
                let walls = count_walls(&map, &path, x as i32, y as i32, 1);

                if walls % 2 == 1 {
                    count += 1;
                }
            }
        }
    }
    Some(count)
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
