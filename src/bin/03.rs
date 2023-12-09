advent_of_code::solution!(3);

use regex::Regex;

#[derive(Debug)]
struct Part {
    line: i32,
    start: i32,
    end: i32,
    value: u32,
}

fn get_char(lines: &[&str], line_no: i32, pos: i32) -> char {
    if line_no < 0
        || line_no >= lines.len() as i32
        || pos < 0
        || pos >= lines[line_no as usize].len() as i32
    {
        '.'
    } else {
        lines[line_no as usize].as_bytes()[pos as usize] as char
    }
}

fn parse_parts(lines: &Vec<&str>) -> Vec<Part> {
    let pattern = Regex::new(r"\d+").unwrap();
    lines
        .iter()
        .enumerate()
        .flat_map(|(line_no, line)| {
            let matches = pattern.find_iter(line);
            matches.map(move |m| Part {
                line: line_no as i32,
                start: m.start() as i32,
                end: m.end() as i32,
                value: m.as_str().parse::<u32>().unwrap(),
            })
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let parts = parse_parts(&lines);

    let valid_parts = parts.iter().filter(|p| {
        for line_no in p.line - 1..=p.line + 1 {
            for pos in p.start - 1..=p.end {
                let ch = get_char(&lines, line_no, pos);
                if ch != '.' && !ch.is_ascii_digit() {
                    return true;
                }
            }
        }
        false
    });

    valid_parts.map(|p| p.value).sum::<u32>().into()
}



pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let parts = parse_parts(&lines);

    let gears = lines.iter().enumerate().flat_map(|(line_no, l)| {
        l.chars().enumerate().flat_map(move |(pos, ch)| {
            if ch == '*' {
                Some((line_no as i32, pos as i32))
            } else {
                None
            }
        })
    }).collect::<Vec<_>>();

    let gear_pairs = gears.iter().map(|(y, x)| {
        parts.iter().filter(|p|
                    (p.line-1 ..=(p.line+1)).contains(y)
                     && (p.start-1 .. p.end+1).contains(x)
                ).collect::<Vec<_>>()
    });

    let result = gear_pairs.filter(|it| it.len() == 2)
        .map(|it| it[0].value * it[1].value).sum::<u32>();

    result.into()

   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
