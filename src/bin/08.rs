use num::integer::lcm;
use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

pub fn part_one(input: &str) -> Option<usize> {
    let (instructions, map) = parse_input(input);
    let mut steps = 0;
    let mut location = "AAA".to_string();

    while location != "ZZZ" {
        let direction = instructions[steps as usize % instructions.len()];
        location = match direction {
            'L' => map[&location].left.clone(),
            'R' => map[&location].right.clone(),
            _ => unreachable!(),
        };
        steps += 1;
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, map) = parse_input(input);
    let start_locations: Vec<String> = map.keys().filter(|&s| s.ends_with("A")).cloned().collect();

    let cycle_lengths: Vec<u64> = start_locations
        .iter()
        .map(|start| find_cycle_length(start, &instructions, &map))
        .collect();

    Some(cycle_lengths.into_iter().fold(1, lcm))
}

fn find_cycle_length(start: &str, instructions: &Vec<char>, map: &HashMap<String, Node>) -> u64 {
    let mut steps: u64 = 0;
    let mut location = start;
    while !location.ends_with("Z") {
        let direction = instructions[steps as usize % instructions.len()];
        location = match direction {
            'L' => &map[location].left,
            'R' => &map[location].right,
            _ => unreachable!(),
        };
        steps += 1
    }
    steps
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<String, Node>) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect();
    lines.next();
    let node_map = lines
        .map(|l| {
            l.split(&['=', '(', ',', ')'])
                .map(|x| x.trim())
                .filter(|&x| !x.is_empty())
                .collect::<Vec<&str>>()
        })
        .filter(|l| !l.is_empty())
        .map(|l| {
            (
                l[0].to_string(),
                Node {
                    left: l[1].to_string(),
                    right: l[2].to_string(),
                },
            )
        })
        .collect();
    (instructions, node_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_parse_input() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = parse_input(input);
        assert_eq!(result.0, vec!['R', 'L']);
        dbg!(&result.1);
        assert_eq!(result.1.len(), 7);
    }
}
