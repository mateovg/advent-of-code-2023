use regex::Regex;
use std::collections::HashSet;
use std::ops::Range;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    let schematic = parse_input(input);
    schematic
        .iter()
        .filter_map(|s| is_adj(s, &schematic))
        .filter_map(|s| match s.value {
            EngineSymbolValue::Number(n) => Some(n),
            _ => None,
        })
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let schematic = parse_input(input);
    schematic
        .iter()
        .filter_map(|s| gear_ratio(s, &schematic))
        .sum::<usize>()
        .into()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct EngineSymbol {
    symbol_type: EngineSymbolType,
    value: EngineSymbolValue,
    position: Range<usize>,
    line: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum EngineSymbolType {
    Number,
    Symbol,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum EngineSymbolValue {
    Symbol(char),
    Number(usize),
}

fn parse_input(input: &str) -> HashSet<EngineSymbol> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            parse_line(line).into_iter().map(move |mut s| {
                s.line = i;
                s
            })
        })
        .collect()
}

fn parse_line(line: &str) -> Vec<EngineSymbol> {
    let re = Regex::new(r"(\d+)|([^\d.])").unwrap();
    re.captures_iter(line)
        .filter_map(|cap| {
            cap.get(1)
                .map(|num_match| EngineSymbol {
                    symbol_type: EngineSymbolType::Number,
                    value: EngineSymbolValue::Number(num_match.as_str().parse().unwrap()),
                    position: num_match.start()..num_match.end(),
                    line: 0, // Will be set correctly in parse_input
                })
                .or_else(|| {
                    cap.get(2).map(|sym_match| EngineSymbol {
                        symbol_type: EngineSymbolType::Symbol,
                        value: EngineSymbolValue::Symbol(
                            sym_match.as_str().chars().next().unwrap(),
                        ),
                        position: sym_match.start()..sym_match.end(),
                        line: 0, // Will be set correctly in parse_input
                    })
                })
        })
        .collect()
}

fn is_adj(part: &EngineSymbol, schematic: &HashSet<EngineSymbol>) -> Option<EngineSymbol> {
    if part.symbol_type != EngineSymbolType::Number {
        return None;
    }
    let x_range = part.position.start.saturating_sub(1)..=part.position.end;
    let y_range = part.line.saturating_sub(1)..=part.line + 1;

    for y in y_range {
        for x in x_range.clone() {
            if schematic.iter().any(|s| {
                s.symbol_type == EngineSymbolType::Symbol && s.line == y && s.position.contains(&x)
            }) {
                return Some(part.clone());
            }
        }
    }
    None
}

fn gear_ratio(part: &EngineSymbol, schematic: &HashSet<EngineSymbol>) -> Option<usize> {
    if let EngineSymbolValue::Symbol('*') = part.value {
        let x_range = part.position.start.saturating_sub(1)..=part.position.end;
        let y_range = part.line.saturating_sub(1)..=part.line + 1;

        let adjacent_numbers: Vec<usize> = schematic
            .iter()
            .filter(|s| s.symbol_type == EngineSymbolType::Number)
            .filter(|s| {
                y_range.contains(&s.line)
                    && (x_range.contains(&s.position.start)
                        || x_range.contains(&(s.position.end - 1)))
            })
            .filter_map(|s| {
                if let EngineSymbolValue::Number(n) = s.value {
                    Some(n)
                } else {
                    None
                }
            })
            .collect();

        if adjacent_numbers.len() == 2 {
            return Some(adjacent_numbers[0] * adjacent_numbers[1]);
        }
    }
    None
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
                symbol_type: EngineSymbolType::Number,
                value: EngineSymbolValue::Number(467),
                position: 0..3,
                line: 0,
            },
            EngineSymbol {
                symbol_type: EngineSymbolType::Number,
                value: EngineSymbolValue::Number(114),
                position: 5..8,
                line: 0,
            },
        ];
        let result = parse_line(line);
        assert_eq!(result, expected);
    }
}
