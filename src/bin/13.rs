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

        let left_start = left_start as i32;
        for row in 0..self.get_height() as i32 {
            let mut left = left_start;
            let mut right = left_start + 1;
            while left >= 0 && right < self.get_width() as i32 {
                let left_value = self.get(row, left);
                let right_value = self.get(row, right);

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
        .map(|it| Map::new(it, false))
        .map(|mut map| {
            (0..map.get_width())
                // look for mirror
                .find(|it| map.test(*it))
                .map(|it| (it + 1) as u32)
                .unwrap_or_else(|| {
                    // transpose and look for mirror
                    map.transposed = true;
                    (0..map.get_width())
                        .find(|it| map.test(*it))
                        .map(|it| 100 * (it + 1) as u32)
                        .unwrap()
                })
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .split("\n\n")
        .map(|it| Map::new(it, false))
        .map(|mut map| {
            let horizontal_solution = (0..map.get_width())
                // look for mirror
                .find(|it| map.test(*it))
                .map(|it| (map, it, false));
            if horizontal_solution.is_none() {
                // transpose and look for mirror
                map.transposed = true;
                (0..map.get_width())
                .find(|it| map.test(*it))
                .map(|it| (map, it, true))
                .unwrap()
            }else{
                horizontal_solution.unwrap()
            }})
        .map(|(mut map, old, old_transposed)| {
            for x in 0..map.get_height() {
                for y in 0..map.get_width() {
                    map.flip_abs(x, y);
                    for l in 0..map.get_width() {
                        if (old_transposed || old != l) && map.test(l) {
                            return (l + 1) as u32;
                        }
                    }

                    map.transposed = true;
                    for l in 0..map.get_width() {
                        if (!old_transposed || old != l) && map.test(l) {
                            return (100 * (l + 1)) as u32;
                        }
                    }
                    map.transposed = false;
                    map.flip_abs(x, y);
                }
            }
            unreachable!();
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
