use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Beam {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Beam {
    fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    fn step(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

struct Map {
    width: i32,
    height: i32,
    data: Vec<u8>,
}

impl Map {
    fn new(data: &str) -> Self {
        let width = data.lines().next().unwrap().len();
        let height = data.lines().count();
        let map = Self {
            width: width as i32,
            height: height as i32,
            data: data.lines().flat_map(|line| line.bytes()).collect(),
        };
        map
    }

    fn get(&self, x: i32, y: i32) -> u8 {
        if self.is_inside(x, y) {
            return self.data[(y * self.width + x) as usize];
        }
        b'.'
    }

    fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

}

pub fn part_one(input: &str) -> Option<u32> {
    let m = Map::new(input);
    Some(simulate(Beam::new(-1, 0, Direction::Right), &m))
}

fn simulate(beam: Beam, m: &Map) -> u32 {
    let mut beams: Vec<Beam> = vec![beam];
    let mut energized = HashSet::new();
    let mut seen = HashSet::new();

    while !beams.is_empty() {
        let mut next_beams = Vec::with_capacity(beams.len()*2);
        beams.into_iter().for_each(|mut beam| {
            beam.step();
            match m.get(beam.x, beam.y) {
                // bounce accordingly
                b'/' if beam.direction == Direction::Right => beam.direction = Direction::Up,
                b'/' if beam.direction == Direction::Left => beam.direction = Direction::Down,
                b'/' if beam.direction == Direction::Up => beam.direction = Direction::Right,
                b'/' if beam.direction == Direction::Down => beam.direction = Direction::Left,

                b'|' if beam.direction == Direction::Right || beam.direction == Direction::Left => {
                    // create one up, one down
                    beam.direction = Direction::Up;
                    next_beams.push(Beam::new(beam.x, beam.y, Direction::Down));
                }

                b'-' if beam.direction == Direction::Up || beam.direction == Direction::Down => {
                    // create one left one write
                    beam.direction = Direction::Right;
                    next_beams.push(Beam::new(beam.x, beam.y, Direction::Left));
                }

                // bounce accordingly
                b'\\' if beam.direction == Direction::Right => beam.direction = Direction::Down,
                b'\\' if beam.direction == Direction::Left => beam.direction = Direction::Up,
                b'\\' if beam.direction == Direction::Up => beam.direction = Direction::Left,
                b'\\' if beam.direction == Direction::Down => beam.direction = Direction::Right,

                _ => {}
            }
            next_beams.push(beam);
        });

        next_beams.retain(|b| m.is_inside(b.x, b.y) && !seen.contains(&(b.x, b.y, b.direction)));

        next_beams.iter().for_each(|p| {
            energized.insert((p.x, p.y));
            seen.insert((p.x, p.y, p.direction));
        });
        beams = next_beams;
    }
    // m.print(&energized);
    energized.len() as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let m = Map::new(input);
    
    let left_to_right = (0..m.height).map(|y| Beam::new(-1, y, Direction::Right));
    let rigth_to_left = (0..m.height).map(|y| Beam::new(m.width, y, Direction::Left));
    let top_down = (0..m.width).map(|x| Beam::new(x, -1, Direction::Down));
    let bottom_up = (0..m.width).map(|x| Beam::new(x, m.height, Direction::Up));

    let all = left_to_right.chain(rigth_to_left).chain(top_down).chain(bottom_up).collect_vec();
    dbg!(all.len());
    let max = all.into_iter().map(|it| simulate(it, &m)).max().unwrap_or(0);
    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
