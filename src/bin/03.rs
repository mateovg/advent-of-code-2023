advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, PartialEq)]
struct EngineSymbol {
    symbol_type: EngineSymbolType,
}

#[derive(Debug, PartialEq)]
enum EngineSymbolType {
    Number(u32),
    Symbol(char),
}

fn parse_input(input: &str) -> Vec<Vec<EngineSymbol>> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Vec<EngineSymbol> {
    line.split('.')
        .filter(|&s| s.len() > 0)
        .map(|s| match s {
            s if !s.chars().next().unwrap_or('N').is_digit(10) => EngineSymbol {
                symbol_type: EngineSymbolType::Symbol(s.chars().next().unwrap()),
            },
            s => EngineSymbol {
                symbol_type: EngineSymbolType::Number(s.parse::<u32>().unwrap()),
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

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
        let line = "...*..300...#";
        let expected = vec![
            EngineSymbol {
                symbol_type: EngineSymbolType::Symbol('*'),
            },
            EngineSymbol {
                symbol_type: EngineSymbolType::Number(300),
            },
            EngineSymbol {
                symbol_type: EngineSymbolType::Symbol('#'),
            },
        ];
        let result = parse_line(line);
        assert_eq!(result, expected);
    }
}
