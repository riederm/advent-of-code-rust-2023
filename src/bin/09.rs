advent_of_code::solution!(9);

fn split_line(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(' ')
        .map(|it| it.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn get_next_element(elements: &[i32]) -> i32 {
    if elements.iter().all(|it| *it == 0) {
        0
    } else {
        let next_level = elements
            .windows(2)
            .map(|it| it[1] - it[0])
            .collect::<Vec<_>>();
        let next = get_next_element(&next_level);
        elements.last().unwrap() + next
    }
}

fn get_prev_element(elements: &[i32]) -> i32 {
    if elements.iter().all(|it| *it == 0) {
        0
    } else {
        let next_level = elements
            .windows(2)
            .map(|it| it[1] - it[0])
            .collect::<Vec<_>>();
        elements.first().unwrap() - get_prev_element(&next_level)
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(split_line)
            .map(|it| get_next_element(it.as_slice()))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(split_line)
            .map(|it| get_prev_element(it.as_slice()))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
