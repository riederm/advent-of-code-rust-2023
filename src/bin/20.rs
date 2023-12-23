use std::{
    collections::{HashMap, VecDeque},
    ops::Not,
};

use itertools::Itertools;

advent_of_code::solution!(20);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    FlipFlop(bool),
    Conjunction(String, HashMap<String, bool>, Option<bool>),
    Broadcaster(),
}

fn parse(input: &str) -> (HashMap<String, Module>, HashMap<String, Vec<String>>) {
    let mut modules = HashMap::new();
    let mut adjacencies = HashMap::new();
    for l in input.lines() {
        let (l, r) = l.split_once("->").unwrap();
        let (l, r) = (l.trim(), r.trim());
        let (name, module) = match l {
            "broadcaster" => ("broadcaster".to_string(), Module::Broadcaster()),
            name if name.starts_with('%') => (
                name.trim_start_matches('%').to_string(),
                Module::FlipFlop(false),
            ),
            name if name.starts_with('&') => (
                name.trim_start_matches('&').to_string(),
                Module::Conjunction(
                    name.trim_start_matches('&').to_string(),
                    HashMap::new(),
                    Some(false),
                ),
            ),
            _ => {
                dbg!(&l);
                unreachable!();
            }
        };
        adjacencies.insert(
            name.to_string(),
            r.split(',').map(|it| it.trim().to_string()).collect_vec(),
        );
        modules.insert(name, module);
    }

    for (name, connected) in &adjacencies {
        for m in connected {
            if let Some(Module::Conjunction(_, state, _)) = modules.get_mut(m) {
                state.insert(name.to_string(), false);
            }
        }
    }

    (modules, adjacencies)
}

#[derive(Debug)]
struct Counter {
    lows: u32,
    highs: u32,
}

impl Counter {
    fn count(&mut self, state: &bool, times: usize) {
        if *state {
            self.highs += times as u32;
        } else {
            self.lows += times as u32;
        }
    }
}

fn simulate(
    modules: &mut HashMap<String, Module>,
    adjacencies: &HashMap<String, Vec<String>>,
    counter: &mut Counter,
    i: u32,
    memory: &mut HashMap<&str, Vec<u32>>,
) -> u32 {
    // from, to, signal
    let mut pending: VecDeque<(&str, &str, bool)> = VecDeque::new();
    pending.push_back(("button", "broadcaster", false));
    counter.count(&false, 1);
    let mut lows_to_rx = 0;
    while let Some((from, to, sig)) = pending.pop_front() {
        // println!("{} -{} -> {}", from, sig, to);
        if let Some(module) = modules.get_mut(to) {
            let connected = adjacencies.get(to).expect(to);
            match module {
                Module::FlipFlop(state) if !sig => {
                    *state = state.not();
                    for t in connected {
                        pending.push_back((to, t, *state));
                    }
                    counter.count(state, connected.len());
                }
                Module::Conjunction(name, state, output) => {
                    if let Some(memory) = state.get_mut(from) {
                        *memory = sig;
                    } else {
                        unreachable!()
                    }
                    let next_sig = state.values().all(|it| *it).not();

                    match name.as_str() {
                        "rz" | "kv" | "jg" | "mr" if next_sig => {
                            if let Some(existing) = memory.get_mut(name.as_str()) {
                                existing.push(i);
                            }
                        }
                        _ => {}
                    }

                    output.replace(next_sig);
                    for t in connected {
                        pending.push_back((to, t, next_sig));
                    }
                    counter.count(&next_sig, connected.len());
                }
                Module::Broadcaster() => {
                    connected
                        .iter()
                        .for_each(|it| pending.push_back((to, it, sig)));
                    counter.count(&sig, connected.len());
                }
                _ => {} //println!("IGNORED {} -> {} = {}", from, to, sig),
            }
        } else if to == "rx" && !sig {
            lows_to_rx += 1;
        }
    }
    lows_to_rx
}

fn mermaid(adjacencies: &HashMap<String, Vec<String>>) -> String {
    let mut result = String::new();
    for (name, connected) in adjacencies {
        for m in connected {
            result.push_str(&format!("{} --> {}\n", name, m));
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut modules, adjacencies) = parse(input);
    let mut counter = Counter { lows: 0, highs: 0 };
    (0..1000).for_each(|_| {
        simulate(
            &mut modules,
            &adjacencies,
            &mut counter,
            0,
            &mut HashMap::new(),
        );
    });
    Some(counter.highs as u64 * counter.lows as u64)
}

fn lcm(elements: Vec<u32>) -> u64 {
    let mut elements = elements;
    let mut result = 1;
    let mut divisor = 2;

    loop {
        let mut divisible = false;
        for i in 0..elements.len() {
            if elements[i] % divisor == 0 {
                divisible = true;
                elements[i] = elements[i] / divisor;
            }
        }

        if divisible {
            result = result * (divisor as u64);
        } else {
            divisor += 1;
        }

        if elements.iter().all(|it| *it == 1) {
            return result;
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut modules, adjacencies) = parse(input);

    // print!("{}", mermaid(&adjacencies));
    let mut counter = Counter { lows: 0, highs: 0 };

    let mut conjunctions: HashMap<&str, Vec<u32>> = HashMap::from_iter([
        ("rz", vec![]),
        ("kv", vec![]),
        ("jg", vec![]),
        ("mr", vec![]),
    ]);

    for i in 1..100000 {
        simulate(
            &mut modules,
            &adjacencies,
            &mut counter,
            i,
            &mut conjunctions,
        );

        if conjunctions.values().all(|it| it.len() >= 2) {
            break;
        }
    }

    dbg!(&conjunctions);
    let steps = conjunctions.values().map(|it| it[1] - it[0]).collect_vec();

    Some(lcm(steps))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
