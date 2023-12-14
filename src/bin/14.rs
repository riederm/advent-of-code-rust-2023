use std::collections::HashMap;

advent_of_code::solution!(14);

const LOOSE: u8 = b'O';
const FREE: u8 = b'.';

struct Platform {
    bytes: Vec<Vec<u8>>,
    pull: Orientation,
}

#[derive(Copy, Clone)]
enum Orientation {
    Up,
    Left,
    Right,
    Down,
}

impl Platform {
    fn new(str: &str) -> Platform {
        let bytes = str.lines().map(|l| l.as_bytes().to_vec()).collect();
        Platform {
            bytes,
            pull: Orientation::Up,
        }
    }

    fn get_width(&self) -> i32 {
        self.bytes[0].len() as i32
    }

    fn get_height(&self) -> i32 {
        self.bytes.len() as i32
    }

    fn translate(&self, x: i32, y: i32) -> (i32, i32) {
        let w = self.get_width();
        let h = self.get_height();
        match self.pull {
            Orientation::Up => (x, y),
            Orientation::Left => (y, w - x - 1),
            Orientation::Down => (x, h - y - 1),
            Orientation::Right => (h - y - 1, x),
        }
    }

    fn full_rotate(&mut self) {
        let sequence = vec![
            Orientation::Up,
            Orientation::Left,
            Orientation::Down,
            Orientation::Right,
        ];
        sequence.iter().for_each(|p| {
            self.pull = *p;
            self.gravity();
        });
    }

    fn get(&self, x: i32, y: i32) -> u8 {
        let (x, y) = self.translate(x, y);
        if x < 0 || y < 0 || x >= self.get_width() || y >= self.get_height() {
            return 0;
        }
        self.bytes[y as usize][x as usize]
    }

    fn set(&mut self, x: i32, y: i32, val: u8) {
        let (x, y) = self.translate(x, y);
        self.bytes[y as usize][x as usize] = val;
    }

    fn gravity(&mut self) {
        (0..self.get_height()).for_each(|row| {
            (0..self.get_width()).for_each(|x| {
                let mut y = row;
                while y >= 0 && self.get(x, y) == LOOSE && self.get(x, y - 1) == FREE {
                    self.set(x, y, FREE);
                    y -= 1;
                    self.set(x, y, LOOSE);
                }
            })
        })
    }

    fn get_score(&self) -> u32 {
        let mut score = 0u32;
        (0..self.get_height()).for_each(|y| {
            (0..self.get_width()).for_each(|x| {
                if self.bytes[y as usize][x as usize] == LOOSE {
                    score += (self.get_height() - y) as u32;
                }
            });
        });
        score
    }
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        self.bytes
            .iter()
            .map(|l| String::from_utf8(l.clone()).unwrap())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut platform = Platform::new(input);
    platform.gravity();
    Some(platform.get_score())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut platform = Platform::new(input);

    let mut memento = HashMap::new();
    let mut loop_end = 0;
    let mut loop_start = None;

    // look for a loop and record the start and the end of the loop
    while loop_start.is_none() {
        loop_start = memento.insert(platform.to_string(), loop_end);
        if loop_start.is_none() {
            loop_end += 1;
            platform.full_rotate();
        }
    }
    let loop_start = loop_start.unwrap();
    let repeat = loop_end - loop_start;

    // from [loop_start] on, we see the same result every [repeat] times
    let missing = (1000000000 - loop_start) % repeat;
    (0..missing).for_each(|_| {
        platform.full_rotate();
    });

    Some(platform.get_score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tilt_up() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        let mut platform = Platform::new(input);
        platform.pull = Orientation::Up;
        platform.gravity();
        let expected = r#"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."#;

        let actual = platform.to_string();
        println!("{actual}");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tilt_left() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        let mut platform = Platform::new(input);
        platform.pull = Orientation::Left;
        platform.gravity();
        let expected = r#"O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#...."#;

        let actual = platform.to_string();
        println!("{actual}");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tilt_down() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        let mut platform = Platform::new(input);
        platform.pull = Orientation::Down;
        platform.gravity();
        let expected = r#".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O"#;

        let actual = platform.to_string();
        println!("{actual}");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tilt_right() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        let mut platform = Platform::new(input);
        platform.pull = Orientation::Right;
        platform.gravity();
        let expected = r#"....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#...."#;

        let actual = platform.to_string();
        println!("{actual}");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_full_rotation() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        let mut platform = Platform::new(input);
        platform.full_rotate();
        platform.full_rotate();
        platform.full_rotate();
        let expected = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"#;

        let actual = platform.to_string();
        println!("{actual}");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
