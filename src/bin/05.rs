use std::collections::HashMap;

advent_of_code::solution!(5);

#[derive(Debug)]
struct ConversionRange {
    destination_start: u32,
    source_start: u32,
    length: u32
}

#[derive(Debug)]
struct Mapping {
    from: String,
    to: String,
    ranges: Vec<ConversionRange> 
}

fn parse_mappings(input: &str) -> (Vec<u32>, HashMap<String, Mapping>) {
    let mut lines = input.lines();

    let seeds_line = lines.next().unwrap();
    let seeds = seeds_line.trim_start_matches("seeds: ") .split(" ").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    lines.next(); //empty line
    let mut mappings = HashMap::new();
    while let Some(line) = lines.next() {
        let mut ranges = Vec::new();
        let (from, to) = line.trim_end_matches(" map:").split_once("-to-").unwrap();
        while let Some(l) = lines.next().filter(|it| !it.trim().is_empty()) {
            let [destination_start, source_start, length] : [&str; 3] = l.splitn(3, ' ').collect::<Vec<_>>().try_into().unwrap();
            let destination_start = destination_start.parse::<u32>().unwrap();
            let source_start = source_start.parse::<u32>().unwrap();
            let length = length.parse::<u32>().unwrap();
            ranges.push(ConversionRange { destination_start, source_start, length });
        }
        mappings.insert(from.to_string(), Mapping{
            from: from.to_string(),
            to: to.to_string(),
            ranges
        });
    }
    
    (seeds, mappings)
}


pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, mappings) = dbg!(parse_mappings(input));


    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
