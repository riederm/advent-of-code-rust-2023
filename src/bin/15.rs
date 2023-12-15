advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split(',')
            .map(|t| t.trim())
            .map(hash)
            .map(|it| it as u32)
            .sum::<u32>(),
    )
}

#[derive(Debug, Default)]
struct LenseBox {
    lenses: Vec<(String, u32)>,
}

impl LenseBox {
    fn add(&mut self, name: &str, focal_len: u32) {
        for i in &mut self.lenses {
            if i.0 == name {
                i.1 = focal_len;
                return;
            }
        }
        self.lenses.push((name.to_string(), focal_len));
    }

    fn remove(&mut self, name: &str) {
        self.lenses.retain(|it| it.0 != name);
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lenses = (0..256).map(|_| LenseBox::default()).collect::<Vec<_>>();

    let binding = input.replace('\n', "");
    let commands = binding
        .split(',')
        .map(|it| it.split_once(['=', '-']).unwrap())
        .map(|(cmd, args)| (cmd.trim(), args.trim().parse::<u32>().ok()))
        .collect::<Vec<_>>();

    for (cmd, arg) in commands {
        let lense_box = &mut lenses.get_mut(hash(cmd) as usize).unwrap();
        if let Some(arg) = arg {
            //add
            lense_box.add(cmd, arg);
        } else {
            //remove
            lense_box.remove(cmd);
        }
    }

    let mut sum = 0;
    for (box_no, lense_box) in lenses.iter().enumerate() {
        for (slot_no, (_, focal_len)) in lense_box.lenses.iter().enumerate() {
            sum += (box_no as u32 + 1) * (slot_no as u32 + 1) * focal_len;
        }
    }

    Some(sum)
}

fn hash(t: &str) -> u8 {
    t.chars().fold(0, hash_char)
}

#[allow(arithmetic_overflow)]
fn hash_char(h: u8, c: char) -> u8 {
    let h = h as u32;
    let c = c as u32;
    let i = h + c;
    let m = i * 17;
    (m % 256) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
