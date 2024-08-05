use regex::Regex;
use std::{collections::HashSet, default, ops::Range};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    parse_input(input)
        .iter()
        .filter_map(|s| is_adj(&s))
        .map(|s| s.symbol_type)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, PartialEq, Hash, Default, Eq)]
struct EngineSymbol {
    symbol_type: EngineSymbolType,
    value: <char, u32>,
    position: Range<usize>,
    pub line: Option<usize>,
}

#[derive(Debug, PartialEq, Hash, Default, Eq)]
enum EngineSymbolType {
    Number,
    Symbol,
    #[default]
    Empty,
}

fn parse_input(input: &str) -> HashSet<EngineSymbol> {
    input
        .lines()
        .flat_map(|line| parse_line(line))
        .enumerate()
        .map(|(i, mut s)| {
            s.line = Some(i);
            s
        })
        .collect()
}

fn parse_line(line: &str) -> Vec<EngineSymbol> {
    let re = Regex::new(r"(\d+)|([^\d.])").unwrap();

    re.captures_iter(line)
        .filter_map(|cap| {
            cap.get(1)
                .map(|num_match| EngineSymbol {
                    symbol_type: EngineSymbolType::Number(num_match.as_str().parse().unwrap()),
                    position: num_match.start()..num_match.end(),
                    ..EngineSymbol::default()
                })
                .or_else(|| {
                    cap.get(2).map(|sym_match| EngineSymbol {
                        symbol_type: EngineSymbolType::Symbol(
                            sym_match.as_str().chars().next().unwrap(),
                        ),
                        position: sym_match.start()..sym_match.start() + 1,
                        ..EngineSymbol::default()
                    })
                })
        })
        .collect()
}

fn is_adj(part: EngineSymbol, schematic: HashSet<EngineSymbol>) -> Option<EngineSymbol> {
    if part.symbol_type == EngineSymbolType::Symbol {
        return None;
    };
    Some(part)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }

    #[test]
    fn parse_line_test() {
        let line = "467..114..";
        let expected = vec![
            EngineSymbol {
                symbol_type: EngineSymbolType::Number(467),
                position: 0..3,
                ..EngineSymbol::default()
            },
            EngineSymbol {
                symbol_type: EngineSymbolType::Number(114),
                position: 5..8,
                ..EngineSymbol::default()
            },
        ];
        let result = parse_line(line);
        dbg!(&result);
        assert_eq!(result, expected);
    }
}
