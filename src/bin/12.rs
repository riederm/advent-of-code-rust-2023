use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(12);

#[derive(Debug)]
struct Line {
    line: String,
    groups: Vec<usize>,
}

const FREE: char = '.';
const SPRING: char = '#';

fn parse_line(line: &str) -> Line {
    let (springs, groups_str) = line.split_once(' ').unwrap();
    let groups = groups_str
        .split(',')
        .map(|it| it.parse::<usize>().unwrap())
        .collect();
    Line {
        line: springs.to_string(),
        groups,
    }
}

type Memento = (usize, usize);

fn count_groups(
    line: &str,
    groups: &Vec<usize>,            // the groups we need to place
    g: usize,                       // the group we are currently placing
    pos_in_string: usize, // the position in the string we are currently at (we move to the right)
    dp: &mut HashMap<Memento, u64>, // intermediate results
) -> u64 {
    if let Some(count) = dp.get(&(g, pos_in_string)) {
        return *count;
    }

    // cancel if we just skipped a spring
    if (1..=line.len()).contains(&pos_in_string)
        && line.as_bytes()[pos_in_string - 1] as char == SPRING
    {
        return 0;
    } else if g == groups.len() {
        // cancel if we left out a spring
        if pos_in_string < line.len() && line[pos_in_string..].chars().any(|c| c == SPRING) {
            return 0;
        }
        // valid solution
        return 1;
    }

    let mut count = 0;
    let mut pos = pos_in_string;
    while pos + groups[g] <= line.len() {
        //grab next candidate
        let candidate = &line[pos..pos + groups[g]];

        // make sure we left no springs behind
        if line[pos_in_string..pos].chars().any(|c| c == SPRING) {
            // no need to try further
            return count;
        }

        // lets see if candidate is valid (all ? or #)
        if candidate.chars().all(|c| c != FREE) {
            // lock this group (g+1) and continue search from right of our hit (pos + groups[g] + 1)
            count += count_groups(line, groups, g + 1, pos + groups[g] + 1, dp); // +1 to leave room between groups
                                                                                 //dynamic programming
            dp.insert((g, pos_in_string), count);
        }
        pos += 1;
    }
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().map(parse_line).collect::<Vec<_>>();
    let sum = lines
        .iter()
        .map(|l| count_groups(l.line.as_str(), &l.groups, 0, 0, &mut HashMap::new()))
        .collect::<Vec<_>>();

    Some(sum.iter().sum())
}

fn expand(line: &Line) -> Line {
    let expanded_line = (0..5).map(|_| line.line.as_str()).join("?");
    let expanded_group = (0..5)
        .flat_map(|_| line.groups.iter())
        .cloned()
        .collect::<Vec<_>>();
    Line {
        line: expanded_line,
        groups: expanded_group,
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let expanded_lines = input.lines().map(parse_line).map(|it| expand(&it));
    let sum = expanded_lines
        .map(|l| count_groups(l.line.as_str(), &l.groups, 0, 0, &mut HashMap::new()))
        .collect::<Vec<_>>();
    Some(sum.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_line_1() {
        let line = parse_line("???.### 1,1,3");
        assert_eq!(
            count_groups(line.line.as_str(), &line.groups, 0, 0, &mut HashMap::new()),
            1
        );
    }

    #[test]
    fn test_line_2() {
        let line = parse_line(".??..??...?##. 1,1,3");
        assert_eq!(
            count_groups(line.line.as_str(), &line.groups, 0, 0, &mut HashMap::new()),
            4
        );
    }

    #[test]
    fn test_line_3() {
        let line = parse_line("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(
            count_groups(line.line.as_str(), &line.groups, 0, 0, &mut HashMap::new()),
            1
        );
    }

    #[test]
    fn test_line_4() {
        let line = parse_line("????.#...#... 4,1,1");
        assert_eq!(
            count_groups(line.line.as_str(), &line.groups, 0, 0, &mut HashMap::new()),
            1
        );
    }

    #[test]
    fn test_line_5() {
        let line = parse_line("????.######..#####. 1,6,5");
        assert_eq!(
            count_groups(line.line.as_str(), &line.groups, 0, 0, &mut HashMap::new()),
            4
        );
    }

    #[test]
    fn test_line_6() {
        let line = parse_line("?###???????? 3,2,1");
        assert_eq!(
            count_groups(line.line.as_str(), &line.groups, 0, 0, &mut HashMap::new()),
            10
        );
    }

    #[test]
    fn test_line_7() {
        let line = parse_line("?.?#????#??## 1,2,7");
        assert_eq!(
            count_groups(line.line.as_str(), &line.groups, 0, 0, &mut HashMap::new()),
            2
        );
    }

    #[test]
    fn test_b_() {
        let line = expand(&parse_line("???.### 1,1,3"));
        assert_eq!(
            count_groups(line.line.as_str(), &line.groups, 0, 0, &mut HashMap::new()),
            1
        );
        let line = expand(&parse_line(".??..??...?##. 1,1,3"));
        assert_eq!(
            count_groups(line.line.as_str(), &line.groups, 0, 0, &mut HashMap::new()),
            16384
        );
        let line = expand(&parse_line("?#?#?#?#?#?#?#? 1,3,1,6"));
        assert_eq!(
            count_groups(line.line.as_str(), &line.groups, 0, 0, &mut HashMap::new()),
            1
        );
        let line = expand(&parse_line("????.#...#... 4,1,1"));
        assert_eq!(
            count_groups(line.line.as_str(), &line.groups, 0, 0, &mut HashMap::new()),
            16
        );
        let line = expand(&parse_line("????.######..#####. 1,6,5"));
        assert_eq!(
            count_groups(line.line.as_str(), &line.groups, 0, 0, &mut HashMap::new()),
            2500
        );
        let line = expand(&parse_line("?###???????? 3,2,1"));
        assert_eq!(
            count_groups(line.line.as_str(), &line.groups, 0, 0, &mut HashMap::new()),
            506250
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
