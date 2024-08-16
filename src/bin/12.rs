advent_of_code::solution!(12);

#[derive(Debug)]
struct SpringSchematic {
    groups: Vec<Vec<char>>,
    sizes: Vec<usize>,
}
impl SpringSchematic {
    fn calculate_combinations(&self, curr_p: usize, curr_n: usize) -> u32 {

    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = parse_input(input);
    dbg!(&result);
    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> Vec<SpringSchematic> {
    input
        .lines()
        .map(|line| {
            let parts: &str = line.split(" ").collect::<Vec<&str>>()[0];
            let condition = parts
                .split('.')
                .map(|g| g.chars().collect())
                .filter(|x: &Vec<char>| !x.is_empty())
                .collect();
            let sizes = line.split(" ").collect::<Vec<&str>>()[1]
                .split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            SpringSchematic {
                groups: condition,
                sizes,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.len(), 6);
        assert_eq!(result[0].groups.len(), 2);
        assert_eq!(result[0].sizes, vec![1, 1, 3]);
    }
}
