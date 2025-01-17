use std::{collections::HashMap, str::FromStr, string::ParseError};

use regex::Regex;

advent_of_code::solution!(19);

#[derive(PartialEq, Debug)]
enum Rule {
    Condition(String, String, i32, String),
    Jump(String),
}

#[derive(PartialEq, Debug)]
struct Instruction {
    name: String,
    rules: Vec<Rule>,
}

#[derive(PartialEq, Debug)]
struct Part {
    variables: HashMap<String, i32>,
}

impl Part {
    fn score(&self) -> i32 {
        self.variables.get("x").unwrap()
            + self.variables.get("m").unwrap()
            + self.variables.get("a").unwrap()
            + self.variables.get("s").unwrap()
    }
}

impl FromStr for Part {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)").unwrap();
        let captures = regex.captures(s).unwrap();
        let x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let m = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let a = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let s = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
        Ok(Part {
            variables: HashMap::from_iter([
                ("x".to_string(), x),
                ("m".to_string(), m),
                ("a".to_string(), a),
                ("s".to_string(), s),
            ]),
        })
    }
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"(\w+)(.)(\d+):(\w+)").unwrap();

        if let Some(captures) = regex.captures(s) {
            let variable = captures.get(1).unwrap().as_str().to_string();
            let operator = captures.get(2).unwrap().as_str().to_string();
            let value = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let action = captures.get(4).unwrap().as_str().to_string();
            Ok(Rule::Condition(variable, operator, value, action))
        } else {
            Ok(Rule::Jump(s.to_string()))
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim_matches('}').split('{').collect();
        let name = parts[0].to_string();
        let rules_str = parts[1].split(',').collect::<Vec<&str>>();
        let mut rules = Vec::new();
        for rule_str in rules_str {
            rules.push(rule_str.parse::<Rule>().unwrap());
        }

        Ok(Instruction { name, rules })
    }
}

fn process(part: &Part, instructions: &HashMap<String, Instruction>) -> bool {
    let mut inst = instructions.get("in");
    while let Some(i) = inst {
        for rule in &i.rules {
            match rule {
                Rule::Condition(variable, operator, value, action) => {
                    let variable_value = part.variables.get(variable).unwrap();
                    let target = match operator.as_str() {
                        ">" => {
                            if variable_value > value {
                                Some(action)
                            } else {
                                None
                            }
                        }
                        "<" => {
                            if variable_value < value {
                                Some(action)
                            } else {
                                None
                            }
                        }
                        _ => panic!("Unknown operator {}", operator),
                    };

                    if let Some(action) = target {
                        match action.as_str() {
                            "A" => return true,
                            "R" => return false,
                            _ => inst = instructions.get(action),
                        }
                        break;
                    }
                }
                Rule::Jump(action) => {
                    match action.as_str() {
                        "A" => return true,
                        "R" => return false,
                        _ => inst = instructions.get(action),
                    }
                    break;
                }
            }
        }
    }
    unreachable!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions_str, parts_str) = input.split_once("\n\n").unwrap();

    let instructions = instructions_str
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();

    let parts = parts_str
        .lines()
        .map(|line| line.parse::<Part>().unwrap())
        .collect::<Vec<Part>>();

    let instructions = instructions
        .into_iter()
        .map(|i| (i.name.clone(), i))
        .collect::<HashMap<String, Instruction>>();
    let accepted = parts.iter().filter(|part| process(part, &instructions));
    let result: u32 = accepted.map(|part| part.score() as u32).sum();
    Some(result)
}

#[derive(PartialEq, Debug, Clone)]
struct SolutionSpace {
    variables: HashMap<String, (i32, i32)>,
}

impl SolutionSpace {
    fn new() -> Self {
        SolutionSpace {
            variables: HashMap::from_iter([
                ("x".to_string(), (1, 4000)),
                ("m".to_string(), (1, 4000)),
                ("a".to_string(), (1, 4000)),
                ("s".to_string(), (1, 4000)),
            ]),
        }
    }

    fn get_number_of_solutions(&self) -> u64 {
        self.variables
            .values()
            .map(|(l, u)| ((u - l + 1) as u64).max(0u64))
            .product()
    }
}

fn reduce(
    mut space: SolutionSpace,
    instructions: &HashMap<String, Instruction>,
    current: &str,
    path: Vec<String>,
) -> u64 {
    match current {
        "A" => space.get_number_of_solutions(),
        "R" => 0,
        rule => {
            let instruction = instructions.get(rule).unwrap();
            let mut sum = 0u64;
            for r in &instruction.rules {
                match r {
                    Rule::Condition(variable, operator, value, target) => {
                        let mut yes_space = space.clone();
                        let yes_range = yes_space.variables.get_mut(variable).unwrap();
                        match operator.as_str() {
                            ">" => {
                                yes_range.0 = std::cmp::max(yes_range.0, *value + 1);
                                let no_range = space.variables.get_mut(variable).unwrap();
                                no_range.1 = std::cmp::min(no_range.1, *value);
                            }
                            "<" => {
                                yes_range.1 = std::cmp::min(yes_range.1, *value - 1);
                                let no_range = space.variables.get_mut(variable).unwrap();
                                no_range.0 = std::cmp::max(no_range.0, *value);
                            }
                            _ => panic!("Unknown operator {}", operator),
                        }

                        // yes jump
                        let mut path = path.clone();
                        path.push(target.clone());
                        sum += reduce(yes_space, instructions, target.as_str(), path);
                    }
                    Rule::Jump(action) => {
                        let mut path = path.clone();
                        path.push(action.clone());
                        sum += reduce(space.clone(), instructions, action.as_str(), path);
                    }
                }
            }
            sum
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions_str, _) = input.split_once("\n\n").unwrap();
    let instructions = instructions_str
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();

    let instructions = instructions
        .into_iter()
        .map(|i| (i.name.clone(), i))
        .collect::<HashMap<String, Instruction>>();

    let path = vec!["in".to_string()];
    Some(reduce(SolutionSpace::new(), &instructions, "in", path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "ex{x>10:one,m<20:two,a>30:R,A,R}";
        let instruction = line.parse::<Instruction>().unwrap();

        assert_eq!(
            instruction,
            Instruction {
                name: "ex".to_string(),
                rules: vec![
                    Rule::Condition("x".to_string(), ">".to_string(), 10, "one".to_string()),
                    Rule::Condition("m".to_string(), "<".to_string(), 20, "two".to_string()),
                    Rule::Condition("a".to_string(), ">".to_string(), 30, "R".to_string()),
                    Rule::Jump("A".to_string()),
                    Rule::Jump("R".to_string()),
                ]
            }
        );
    }

    #[test]
    fn test_parse_part() {
        let line = "{x=787,m=2655,a=1222,s=2876}";
        let part = line.parse::<Part>().unwrap();

        assert_eq!(
            part,
            Part {
                variables: HashMap::from_iter([
                    ("x".to_string(), 787),
                    ("m".to_string(), 2655),
                    ("a".to_string(), 1222),
                    ("s".to_string(), 2876),
                ])
            }
        );
    }

    // 10354173786531281 too high

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }

    #[test]
    fn xxx() {
        let r = 4000u32..4001u32;
        println!("{:?}", r.len());
        println!("{:?}", r.end - r.start);
    }
}
