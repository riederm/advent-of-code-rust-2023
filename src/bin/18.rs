use std::{str::FromStr, collections::HashSet};

advent_of_code::solution!(18);

#[derive(Debug)]
struct Line {
    direction: char,
    number: i32,
    color: String,
}

impl FromStr for Line {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let letter = parts[0].chars().next().unwrap();
        let number = parts[1].parse::<i32>()?;
        let color = parts[2].trim_matches(')').trim_start_matches("(#").to_string();

        Ok(Line {
            direction: letter,
            number,
            color,
        })
    }
}

fn get_direction(dir: char) -> (i32, i32) {
    match dir {
        'D' => (0, -1),
        'U' => (0, 1),
        'R' => (1, 0),
        'L' => (-1, 0),
        _ => panic!("Unknown direction {}", dir),
    }
}

fn dig(lines: &[Line], pos: (i32, i32)) -> Vec<(i32, i32)> {
    let mut points = vec![pos];
    let mut pos = pos;

    for l in lines {
        let (dx, dy) = get_direction(l.direction);
        pos.0 += dx*l.number;
        pos.1 += dy*l.number;
        points.push(pos);
    }

    points
    // let mut holes = HashSet::new();
    // holes.insert(pos);
    // let mut pos = pos;  
    // for l in lines {
    //     let (dx, dy) = get_direction(l.direction);
    //     for _ in 0..l.number {
    //         pos.0 += dx;
    //         pos.1 += dy;
    //         holes.insert(pos);
    //     }
    // }
    // holes
}

fn calculate_area2(points: &[(i32, i32)]) -> f64 {
    let mut sum = 0;
    let mut lines = 0;
    for i in 0..points.len() {
        let i = i % points.len();
        let j = (i + 1) % points.len();

        let (x,y) = points[i];
        let (x2,y2) = points[j];

        sum += (y+y2)  * (x-x2);

        lines += (x-x2).abs() + (y-y2).abs();
        
    }

    0.5 * sum.abs() as f64 + lines as f64
}

fn calculate_area(points: &[(i32, i32)]) -> f64 {
    let mut sum = 0.0;
    for i in 0..points.len()-1 {
        let j = (i + 1) % points.len();
        sum += (points[i].0 * points[j].1) as f64 - (points[j].0 * points[i].1) as f64;

        // length of line (its either horizontal or vertical)
        sum += ((points[i].0 - points[1].0).abs() + (points[i].1 - points[j].1).abs()) as f64;
    }
    sum.abs() / 2.0 + 1f64 
} 

pub fn part_one(input: &str) -> Option<u64> {
    // let lines: Vec<Line> = input.lines().map(|line| line.parse().unwrap()).collect();
    // let polygon = dig(&lines, (0, 1));
    // Some(calculate_area(&polygon) as u32)    

     let p1 = input.split('\n').map(|l| {
    let (n, _) = l[2..].split_once(' ').unwrap();
    (l.as_bytes()[0], n.parse::<isize>().unwrap())
  });
  let p2 = input.split('\n').map(|l| {
    let (_, color) = l.split_once(" (#").unwrap();
    let d = match color.as_bytes()[color.len()-2] {
      b'0' => b'R',
      b'1' => b'D',
      b'2' => b'L',
      b'3' => b'U',
      _ => unreachable!(),
    };
    (d, isize::from_str_radix(&color[0..color.len()-2], 16).unwrap())
  });
  Some(calc_area(p1) as u64)

}

fn calc_area(instructions: impl Iterator<Item=(u8, isize)>) -> isize {
  let (mut a, mut r, mut c) = (0,0,0);
  for (d, n) in instructions {
    let (rr, cc) = (r,c);
    match d {
      b'U' => r -= n,
      b'R' => c += n,
      b'D' => r += n,
      b'L' => c -= n,
      _ => unreachable!()
    };
    a += (c + cc) * (r - rr) + n;
  }
  a / 2 + 1
}

pub fn part_two(input: &str) -> Option<u64> {
     let p1 = input.split('\n').map(|l| {
    let (n, _) = l[2..].split_once(' ').unwrap();
    (l.as_bytes()[0], n.parse::<isize>().unwrap())
  });
  let p2 = input.split('\n').map(|l| {
    let (_, color) = l.split_once(" (#").unwrap();
    let d = match color.as_bytes()[color.len()-2] {
      b'0' => b'R',
      b'1' => b'D',
      b'2' => b'L',
      b'3' => b'U',
      _ => unreachable!(),
    };
    (d, isize::from_str_radix(&color[0..color.len()-2], 16).unwrap())
  });
  Some(calc_area(p2) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_are() {
        let result = calculate_area(&vec![(0, 0), (0, 1), (1, 1), (1, 0)]) as u32 ;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
