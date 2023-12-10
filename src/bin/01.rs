advent_of_code::solution!(1);

#[derive(Clone)]
struct Digit {
    text: String,
    value: u32,
}

impl Digit {
    fn new(text: String, value: u32) -> Self {
        Self { text, value }
    }

    fn create_digits() -> Vec<Self> {
        vec![
            Digit::new("1".to_owned(), 1),
            Digit::new("2".to_owned(), 2),
            Digit::new("3".to_owned(), 3),
            Digit::new("4".to_owned(), 4),
            Digit::new("5".to_owned(), 5),
            Digit::new("6".to_owned(), 6),
            Digit::new("7".to_owned(), 7),
            Digit::new("8".to_owned(), 8),
            Digit::new("9".to_owned(), 9),
        ]
    }
}

fn find_digit(text: &str, digits: &[Digit]) -> Option<u32> {
    return digits
        .iter()
        .map(|digit| (digit.value, text.find(&digit.text)))
        .filter(|(_, index)| index.is_some())
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(value, _)| value);
}

pub fn part_one(input: &str) -> Option<u32> {
    let digits = Digit::create_digits();
    let result = input
        .lines()
        .map(|it| {
            (
                find_digit(it, &digits).unwrap(),
                find_digit(it.chars().rev().collect::<String>().as_str(), &digits).unwrap(),
            )
        })
        .map(|fix| fix.0 * 10 + fix.1)
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut digits2 = Digit::create_digits();
    digits2.extend(vec![
        Digit::new("one".to_owned(), 1),
        Digit::new("two".to_owned(), 2),
        Digit::new("three".to_owned(), 3),
        Digit::new("four".to_owned(), 4),
        Digit::new("five".to_owned(), 5),
        Digit::new("six".to_owned(), 6),
        Digit::new("seven".to_owned(), 7),
        Digit::new("eight".to_owned(), 8),
        Digit::new("nine".to_owned(), 9),
    ]);

    let digits_rev = digits2
        .iter()
        .map(|it| Digit::new(it.text.chars().rev().collect::<String>(), it.value))
        .collect::<Vec<_>>();

    let result = input
        .lines()
        .map(|it| {
            (
                find_digit(it, digits2.as_slice()).unwrap(),
                find_digit(
                    it.chars().rev().collect::<String>().as_str(),
                    digits_rev.as_slice(),
                )
                .unwrap(),
            )
        })
        .map(|fix| fix.0 * 10 + fix.1)
        .sum();
    Some(result)
}
