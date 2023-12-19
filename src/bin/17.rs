use core::panic;
use std::collections::{BinaryHeap, HashMap};
advent_of_code::solution!(17);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

static DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: -1 }, //N
    Point { x: 0, y: 1 },  //S
    Point { x: -1, y: 0 }, //W
    Point { x: 1, y: 0 },  //E 
];

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn is_opposite_of(&self, direction: &Point) -> bool {
        self.x == -direction.x && self.y == -direction.y
    }

    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn scale(&self, factor: i32) -> Point {
        Point {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

struct Map {
    width: i32,
    height: i32,
    data: Vec<Vec<u8>>,
}

impl Map {
    fn new(data: &str) -> Self {
        let width = data.lines().next().unwrap().len();
        let height = data.lines().count();
        let map = Self {
            width: width as i32,
            height: height as i32,
            data: data
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_string().parse::<u8>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        };
        map
    }

    fn is_inside(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height
    }

    fn get(&self, p: &Point) -> u8 {
        if self.is_inside(p) {
            return self.data[p.y as usize][p.x as usize];
        }
        panic!("Point ({},{}) is outside of map", p.x, p.y);
    }
}

static START_CAME_FROM_DIR: Point = Point { x: 0, y: 0 };

// (cost, destination, direction we came from)
type QueueItem = (i32, (Point, &'static Point));

fn a_star(start: Point, end: Point, map: &Map, min_steps: i32, max_steps: i32) -> i32 {
    let mut queue: BinaryHeap<QueueItem> = BinaryHeap::new();

    let mut costs = HashMap::new();
    // special came from for the start
    queue.push((0i32, (start, &START_CAME_FROM_DIR)));
    costs.insert((start, &START_CAME_FROM_DIR), 0);
    let mut best_cost = i32::MAX;
    while let Some((cost, (pos, coming_from))) = queue.pop() {
        let cost = cost.abs();
        if pos == end && cost < best_cost {
            best_cost = cost;
            println!("Found path with cost {}", cost);
            break;
        } else if cost > best_cost {
            continue;
        }

        for d in &DIRECTIONS {
            // if we came from the same direction, we already tried the 1,2,3 steps, so skip
            // and we cannot reverse
            if d.eq(coming_from) || d.is_opposite_of(coming_from) {
                continue;
            }
            let mut next_costs = cost;
            // eagerly try all possible steps into the same direction and keep the direction in mind,
            // so we dont retry the same direction on this field later
            for steps in 1..=max_steps {
                let next_d = d.scale(steps);
                let next_pos = pos.add(&next_d);
                if map.is_inside(&next_pos) {
                    let known_cost = costs.get(&(next_pos, d)).unwrap_or(&i32::MAX).abs();
                    next_costs += map.get(&next_pos) as i32;

                    // skip illegal steps (bart b), skip if we already know a better solution
                    if steps >= min_steps && next_costs < best_cost && known_cost > next_costs {
                        costs.insert((next_pos, d), next_costs);
                        queue.push((-next_costs, (next_pos, d)));
                    }
                }
            }
        }
    }
    best_cost
}

pub fn part_one(input: &str) -> Option<u32> {
    let m = Map::new(input);

    let best = a_star(
        Point::new(0, 0),
        Point::new(m.width - 1, m.height - 1),
        &m,
        1,
        3,
    );
    Some(best as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let m = Map::new(input);
    let best = a_star(
        Point::new(0, 0),
        Point::new(m.width - 1, m.height - 1),
        &m,
        4,
        10,
    );
    Some(best as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
