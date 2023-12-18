use core::panic;

advent_of_code::solution!(13);

const JOKER: u8 = 0;

struct Map {
    bytes: Vec<Vec<u8>>,
    transposed: bool,
}

impl Map {
    fn new(str: &str, transposed: bool) -> Map {
        let bytes = str.lines().map(|l| l.as_bytes().to_vec()).collect();
        Map { bytes, transposed }
    }

    fn get(&self, row: i32, col: i32) -> u8 {
        if self.transposed {
            self.do_get(col, row)
        } else {
            self.do_get(row, col)
        }
    }

    fn get_width(&self) -> usize {
        if self.transposed {
            self.bytes.len()
        } else {
            self.bytes[0].len()
        }
    }

    fn get_height(&self) -> usize {
        if self.transposed {
            self.bytes[0].len()
        } else {
            self.bytes.len()
        }
    }

    fn do_get(&self, row: i32, col: i32) -> u8 {
        if col < 0 || col >= self.bytes[0].len() as i32 || row < 0 || row >= self.bytes.len() as i32
        {
            JOKER
        } else {
            self.bytes[row as usize][col as usize]
        }
    }

    fn test(&self, left_start: usize) -> bool {
        if left_start >= self.get_width() - 1 {
            return false;
        }

        for row in 0..self.get_height() {
            let mut left = left_start as i32;
            let mut right = left_start as i32 + 1;
            while left >= 0 && right < self.get_width() as i32 {
                let left_value = self.get(row as i32, left);
                let right_value = self.get(row as i32, right);

                if !(left_value == JOKER || right_value == JOKER || left_value == right_value) {
                    return false;
                }
                left -= 1;
                right += 1;
            }
        }
        true
    }

    fn flip_abs(&mut self, row: usize, col: usize) {
        let value = self.bytes[row][col];
        if value == b'.' {
            self.bytes[row][col] = b'#';
        } else if value == b'#' {
            self.bytes[row][col] = b'.';
        } else {
            panic!("invalid value: {}", value);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .split("\n\n")
        .map(|it| (Map::new(it, false), Map::new(it, true)))
        .map(|(lines, cols)| {
            for l in 0..lines.get_width() {
                if lines.test(l) {
                    return (l + 1) as u32;
                }
            }
            for l in 0..cols.get_width() {
                if cols.test(l) {
                    return (100 * (l + 1)) as u32;
                }
            }
            panic!(
                "no solution found:\n{}",
                lines
                    .bytes
                    .iter()
                    .map(|it| String::from_utf8(it.clone()).unwrap())
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .split("\n\n")
        .map(|it| (Map::new(it, false), Map::new(it, true)))
        .map(|(mut lines, mut map)| {
            for x in 0..lines.get_width() {
                for y in 0..lines.get_height() {
                    lines.flip_abs(y, x);
                    for l in 0..lines.get_width() {
                        if lines.test(l) {
                            return (l + 1) as u32;
                        }
                    }
                    lines.flip_abs(y, x);

                    map.flip_abs(y, x);
                    for l in 0..map.get_width() {
                        if map.test(l) {
                            return (100 * (l + 1)) as u32;
                        }
                    }
                    map.transposed = false;
                    map.flip_abs(x, y);
                }
            }
            unreachable!()
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;

    #[test]
    fn test_example_one() {
        let str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;

        let map = Map::new(str, false);
        vec![
            (0, false),
            (1, false),
            (2, false),
            (3, false),
            (4, true),
            (5, false),
            (6, false),
            (7, false),
            (8, false),
            (9, false),
        ]
        .iter()
        .for_each(|(i, expected)| assert_eq!(*expected, map.test(*i)))
    }

    #[test]
    fn test_bigger_map() {
        let str = r#"
#.#...##..#
####..##.#.
#..#.......
#..#.......
####..##.#.
#.#.#.##..#
...#.#####.
##..#.##...
#.#...#....
##.#.#.#.##
.###...#.##
.#.#.######
.#.##.#..##
.##.##.##.#
.##.##.##.#"#;

        let map = Map::new(str, true);
        assert!(map.test(2))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
