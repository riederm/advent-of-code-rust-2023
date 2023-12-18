use std::{
    collections::{BTreeSet, HashMap, HashSet},
    iter::Filter,
};

use itertools::Itertools;

advent_of_code::solution!(17);

type AbsPoint = Point;
type Direction = &'static Point;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Step {
    direction: Direction,
    position: AbsPoint,
    options: Vec<Direction>,
    same_dir_steps: i32,
}

impl Step {
    fn new(direction: Direction, position: AbsPoint, options: Vec<Direction>, same_dir_steps: i32) -> Self {
        Self {
            direction,
            position,
            options,
            same_dir_steps,
        }
    }
}

static NORTH: Point = Point { x: 0, y: -1};
static SOUTH: Point = Point { x: 0, y: 1 };
static WEST: Point = Point { x: -1, y: 0 };
static EAST: Point = Point { x: 1, y: 0 };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Default, Debug, Clone)]
struct Path {
    elements: Vec<Step>,
    elements_set: HashSet<Step>,
}

impl Path {
    fn add(&mut self, element: Step) {
        self.elements.push(element.clone());
        self.elements_set.insert(element);
    }

    fn remove_last(&mut self) {
        let e = self.elements.pop();
        if let Some(e) = e {
            self.elements_set.remove(&e);
        }
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn iter(&self) -> std::slice::Iter<Step> {
        self.elements.iter()
    }

    fn get_number_of_same_directions(&self) -> i32 {
        let mut count = 0;
        let mut last_direction = None;
        for e in self.elements.iter().rev().take(3) {
            if let Some(last_direction) = last_direction {
                if last_direction == e.direction {
                    count += 1;
                } else {
                    break;
                }
            } else {
                last_direction = Some(e.direction);
                count += 1;
            }
        }
        count
    }
}

static DIRECTIONS: [Point; 4] = [NORTH, SOUTH, WEST, EAST];

struct Walker {
    position: Point,
    path: Path,
    heat: u32,
    best_heat: u32,
    best_path: Option<Path>,
}

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn dist(&self, other: &Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    fn orientation(&self) -> Orientation {
        if self.x == 0 {
            Orientation::Vertical
        } else {
            Orientation::Horizontal
        }
    }

    fn is_opposite_of(&self, direction: &Point) -> bool {
        self.x == -direction.x && self.y == -direction.y
    }

    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn get_name(&self) -> String {
        match self {
            _ if self.eq(&NORTH) => "^",
            _ if self.eq(&SOUTH) => "v",
            _ if self.eq(&EAST) => ">",
            _ if self.eq(&WEST) => "<",
            _  => "X",
        }.to_string()
    }
}

impl Walker {
    fn new(position: Point) -> Self {
        Self {
            position,
            path: Path::default(),
            heat: 0,
            best_heat: u32::MAX,
            best_path: None
        }
    }

    fn get_illegal_orientation(&self) -> Option<Orientation> {
        if self.path.len() < 3 {
            return None;
        }
        let mut i = self.path.iter().rev();
        let (l1, l2, l3) = (
            i.next().unwrap().direction.orientation(),
            i.next().unwrap().direction.orientation(),
            i.next().unwrap().direction.orientation(),
        );

        if l1 == l2 && l2 == l3 {
            Some(l1)
        } else {
            None
        }
    }

    fn get_options(&self, map: &Map) -> Vec<Direction> {
        let illegal: Option<Orientation> = self.get_illegal_orientation();
        let last_direction = self.path.iter().rev().last().map(|s| s.direction);
        DIRECTIONS
            .iter()
            .filter(|d| illegal.map(|id| d.orientation() != id).unwrap_or(true))
            .filter(|d| map.is_inside(&self.position.add(d)))
            // .filter(|d| {
            //     last_direction
            //         .map(|p| !p.is_opposite_of(d))
            //         .unwrap_or(true)
            // })
            .collect_vec()
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
        unreachable!()
    }

    fn is_finished(&self, p: &Point) -> bool {
        p.x == self.width - 1 && p.y == self.height - 1
    }
}

fn walk(walker: &mut Walker, map: &Map, dp: &mut HashMap<Step, u32>) {
    if map.is_finished(&walker.position) && walker.heat < walker.best_heat {
        println!(
            "Found path with {} steps, heat {}",
            walker.path.len(),
            walker.heat
        );
        walker.best_heat = walker.heat.min(walker.best_heat);
        walker.best_path = Some(walker.path.clone());
    }

    let options = walker.get_options(map);
    let same_direction = walker.path.get_number_of_same_directions();
    for direction in &options {
        //prepare step
        let next_pos = walker.position.add(direction);
        let next_step = Step::new(direction, next_pos, options.clone(), same_direction);

        let h = map.get(&walker.position) as u32;
        let next_heat = h + walker.heat;
        let prev_pos = walker.path.iter().rev().nth(1).map(|it| it.position).unwrap_or_else(|| Point::new(-1,-1));

        if next_pos.ne(&prev_pos) && next_heat < walker.best_heat && *dp.get(&next_step).unwrap_or(&u32::MAX) > next_heat
        {
            walker.heat = next_heat;
            
            dp.insert(next_step.clone(), walker.heat);
            walker.position = next_step.position;
            walker.path.add(next_step);

            walk(walker, map, dp);

            walker.heat -= h;
            walker.path.remove_last();
            walker.position = walker.position.sub(direction);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let m = Map::new(input);
    let mut w = Walker::new(Point { x: 0, y: 0 });
    let mut dp = HashMap::new();
    walk(&mut w, &m, &mut dp);
    dbg!(w.best_path.map(|p| p.elements.iter().map(|it| it.direction.get_name()).collect_vec()));
    Some(w.best_heat)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_illegal_moves() {
        let data = advent_of_code::template::read_file("examples", DAY);
        let m = Map::new(data.as_str());
        let w = Walker::new(Point { x: 0, y: 0 });
        let options = w.get_options(&m);

        assert_eq!(vec![&SOUTH, &EAST], options);
    }

    #[test]
    fn test_illegal_moves2() {
        let data = advent_of_code::template::read_file("examples", DAY);
        let m = Map::new(data.as_str());
        let w = Walker::new(Point { x: 2, y: 2 });
        let options = w.get_options(&m);

        assert_eq!(vec![&NORTH, &SOUTH, &WEST, &EAST], options);
    }

    #[test]
    fn test_illegal_moves3() {
        let data = advent_of_code::template::read_file("examples", DAY);
        let m = Map::new(data.as_str());
        let mut w = Walker::new(Point { x: 2, y: 2 });

        w.path.add(Step::new(&WEST, w.position.add(&WEST), vec![], 1)); // 1,2
        let options = w.get_options(&m);
        // make sure that back to north is not part of it
        assert_eq!(vec![&NORTH, &SOUTH, &WEST], options);
    }

    // #[test]
    // fn test_illegal_moves4() {
    //     let data = advent_of_code::template::read_file("examples", DAY);
    //     let m = Map::new(data.as_str());
    //     let mut w = Walker::new(Point { x: 5, y: 5 });

    //     w.path.add(Step::new(&WEST, w.position.add(&WEST), vec![])); 
    //     w.path.add(Step::new(&WEST, w.position.add(&WEST), vec![])); 
    //     let options = w.get_options(&m);
    //     // make sure that back to north is not part of it
    //     assert_eq!(vec![&NORTH, &SOUTH], options);
    // }

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
