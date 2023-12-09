advent_of_code::solution!(4);

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
struct Card {
    card_number: u32,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('|').collect();
        let card_and_winning_numbers: Vec<&str> = parts[0].split(':').collect();
        let card_number = card_and_winning_numbers[0]
            .trim()
            .trim_start_matches("Card")
            .trim()
            .parse::<u32>()?;
        let winning_numbers = card_and_winning_numbers[1]
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<HashSet<u32>, Self::Err>>()?;
        let numbers = parts[1]
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, Self::Err>>()?;

        Ok(Card {
            card_number,
            winning_numbers,
            numbers,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = input
        .lines()
        .map(|it| it.parse::<Card>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let number_of_matches = cards
        .iter()
        .map(|c| {
            c.numbers
                .iter()
                .filter(|n| c.winning_numbers.contains(n))
                .count() as u32
        })
        .filter(|it| *it > 0);

    let points = number_of_matches.map(|it| it - 1).map(|it| 2u32.pow(it));

    Some(points.sum())
}

fn count_cards(cards: &HashMap<u32, Card>, number: u32, memory: &mut HashMap<u32, u32>) -> u32 {
    if let Some(result) = memory.get(&number) {
        return *result;
    }

    let card = cards.get(&number).unwrap();
    let number_of_matches = card
        .numbers
        .iter()
        .filter(|n| card.winning_numbers.contains(n))
        .count() as u32;

    let result = 1
        + (number + 1..=(number + number_of_matches))
            .map(|c| count_cards(cards, c, memory))
            .sum::<u32>();

    memory.insert(number, result);
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input
        .lines()
        .map(|it| it.parse::<Card>().unwrap())
        .map(|it| (it.card_number, it))
        .collect::<HashMap<_, _>>();

    let mut memory = HashMap::new();
    Some(
        cards
            .values()
            .map(|c| count_cards(&cards, c.card_number, &mut memory))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
