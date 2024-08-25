use std::collections::HashMap;

advent_of_code::solution!(15);

#[derive(Debug)]
struct Instruction {
    box_number: u32,
    label: String,
    operation: char,
    focus: Option<u32>,
    hash: u32,
}
impl Instruction {
    fn new(input: &str) -> Instruction {
        let hash = hash_instruction(input);
        let operation = if input.contains('=') { '=' } else { '-' };
        let operation_idx = input.find(|c| c == '=' || c == '-');
        let focus = if operation == '=' {
            input.chars().filter_map(|c| c.to_digit(10)).last()
        } else {
            None
        };
        let label = input[0..operation_idx.unwrap()].to_string();
        let box_number = hash_instruction(&label);
        Instruction {
            box_number,
            label,
            operation,
            focus,
            hash,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_input(input).iter().map(|ins| ins.hash).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(parse_input(input));
    let res = map
        .iter()
        .map(|(box_number, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(i, ins)| (box_number + 1) * (i as u32 + 1) * ins.focus.unwrap())
                .sum::<u32>()
        })
        .sum();
    Some(res)
}

fn parse_map(input: Vec<Instruction>) -> HashMap<u32, Vec<Instruction>> {
    let mut map: HashMap<u32, Vec<Instruction>> = HashMap::new();
    for ins in input {
        match ins.operation {
            '-' => {
                if let Some(x) = map.get_mut(&ins.box_number) {
                    for i in 0..x.len() {
                        if x[i].label == ins.label {
                            x.remove(i);
                            break;
                        }
                    }
                }
            }
            '=' => {
                if let Some(x) = map.get_mut(&ins.box_number) {
                    let mut updated = false;
                    for i in 0..x.len() {
                        if x[i].label == ins.label {
                            x[i].focus = ins.focus;
                            updated = true;
                        }
                    }
                    if !updated {
                        x.push(ins);
                    }
                } else {
                    map.insert(ins.box_number, vec![ins]);
                }
            }
            _ => unreachable!(),
        }
    }

    map
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .split(&[',', '\n'])
        .filter(|&s| !s.is_empty())
        .map(|s| Instruction::new(s))
        .collect()
}

fn hash_instruction(input: &str) -> u32 {
    let ascii_str = input.as_bytes();
    ascii_str
        .iter()
        .fold(0, |hash, curr| hash_algo(hash, *curr))
}

fn hash_algo(curr_value: u32, curr_char: u8) -> u32 {
    let hash = curr_value + curr_char as u32;
    let hash = hash * 17;
    let hash = hash % 256;
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }

    #[test]
    fn test_hash_algo() {
        assert_eq!(hash_algo(0, b'H'), 200);
        assert_eq!(hash_algo(200, b'A'), 153);
        assert_eq!(hash_algo(153, b'S'), 172);
        assert_eq!(hash_algo(172, b'H'), 52);
    }

    #[test]
    fn test_hash_instruction() {
        assert_eq!(hash_instruction("HASH"), 52);
    }

    #[test]
    fn test_instruction() {
        let input = "HASH=9";
        let instruction = Instruction::new(input);
        assert_eq!(instruction.operation, '=');
        assert_eq!(instruction.box_number, 52);
        assert_eq!(instruction.label, "HASH".to_string());
        assert_eq!(instruction.focus, Some(9));
        let input = "HASH-";
        let instruction = Instruction::new(input);
        assert_eq!(instruction.operation, '-');
        assert_eq!(instruction.focus, None);
    }

    #[test]
    fn parse_map_test() {
        let input = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let map = parse_map(input);
        assert_eq!(map.keys().len(), 3);
    }
}
