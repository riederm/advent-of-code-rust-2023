use std::{collections::{HashMap, VecDeque}, ops::Not};

use itertools::Itertools;

advent_of_code::solution!(20);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
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
                Module::Conjunction(HashMap::new()),
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
            if let Some(Module::Conjunction(state)) = modules.get_mut(m) {
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
    counter: &mut Counter
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
                Module::Conjunction(state) => {
                    if let Some(memory) = state.get_mut(from) {
                        *memory = sig;
                    }else{
                        unreachable!()
                    }
                    let next_sig = state.values().all(|it| *it).not();
                    for t in connected {
                        pending.push_back((to, t, next_sig));
                        
                    }
                    counter.count(&next_sig, connected.len());
                }
                Module::Broadcaster() => {
                    connected.iter().for_each(|it| pending.push_back((to, it, sig)));
                    counter.count(&sig, connected.len());
                }
                _ => {}//println!("IGNORED {} -> {} = {}", from, to, sig),
            }
        }else if to == "rx" && !sig {
            lows_to_rx += 1;
        }
    }
    lows_to_rx
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut modules, adjacencies) = parse(input);
    let mut counter = Counter { lows: 0, highs: 0 };
    (0..1000).for_each(|i| {
        simulate(&mut modules, &adjacencies, &mut counter);
        // dbg!(&modules);
        // println!("-------------------");
    });
    // dbg!(&counter);
    Some(counter.highs as u64 * counter.lows as u64)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut modules, adjacencies) = parse(input);
    let mut counter = Counter { lows: 0, highs: 0 };
    let mut rx = 0;
    let mut runs = 0u32;
    let (mut min, mut max) = (0, 0);
    while rx != 1 {
        rx = simulate(&mut modules, &adjacencies, &mut counter);
        runs += 1;
        
        for m in modules.values() {
            
        }

        
        println!("-------------------");

    }
    Some(runs)
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
