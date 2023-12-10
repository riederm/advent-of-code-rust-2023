use std::collections::HashMap;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<HashMap<String, u32>>,
}

fn parse(input: &str) -> Vec<Game> {
    let mut games = Vec::new();
    for line in input.lines() {
        let parts = line.split(':').collect::<Vec<_>>();
        let name = parts[0].to_string();

        let id = name.split(' ').nth(1).unwrap().parse::<u32>().unwrap();
        let rounds = parts[1]
            .trim()
            .split(';')
            .map(|set| {
                set.trim()
                    .split(',')
                    .map(|ball| {
                        let mut parts = ball.trim().split(' ').map(|it| it.trim());
                        let count = parts.next().unwrap().trim().parse::<u32>().unwrap();
                        let color = parts.next().unwrap().trim().to_owned();
                        (color, count)
                    })
                    .collect::<HashMap<_, _>>()
            })
            .collect::<Vec<_>>();
        games.push(Game { rounds, id });
    }
    games
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse(input);
    let valid_games = games.iter().filter(|g| {
        g.rounds.iter().all(|r| {
            r.get("red").cloned().unwrap_or_default() <= 12
                && r.get("green").cloned().unwrap_or_default() <= 13
                && r.get("blue").cloned().unwrap_or_default() <= 14
        })
    });

    let sum = valid_games.map(|g| g.id).sum::<u32>();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse(input);
    let minimum_games = games
        .iter()
        .map(|g| {
            let red = g
                .rounds
                .iter()
                .filter_map(|r| r.get("red"))
                .max()
                .unwrap_or(&1);
            let green = g
                .rounds
                .iter()
                .filter_map(|r| r.get("green"))
                .max()
                .unwrap_or(&1);
            let blue = g
                .rounds
                .iter()
                .filter_map(|r| r.get("blue"))
                .max()
                .unwrap_or(&1);
            red * green * blue
        })
        .collect::<Vec<_>>();
    Some(minimum_games.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
