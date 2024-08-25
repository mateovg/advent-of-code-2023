advent_of_code::solution!(12);

#[derive(Debug)]
struct SpringSchematic {
    groups: Vec<char>,
    sizes: Vec<usize>,
}
impl SpringSchematic {
    fn combinations(&self) -> u64 {
        let line = self.groups.clone();
        let n = line.len();
        let m = self.sizes.len();
        let mut dp = &mut vec![vec![0; n + 1]; m + 1];
        let mut next_dp = &mut vec![vec![0; n + 1]; m + 1];

        dp[m][0] = 1;
        dp[m - 1][self.sizes[m - 1]] = 1;

        for pos in (0..n).rev() {
            for group in 0..=m {
                let max_count = if group == m { 0 } else { self.sizes[group] };
                for count in 0..=max_count {
                    next_dp[group][count] = 0;
                    if matches!(line[pos], '#' | '?') {
                        next_dp[group][count] += dp[group][count + 1];
                    }
                }
                if matches!(line[pos], '.' | '?') {
                    next_dp[group][0] += dp[group][0];
                    if group < m {
                        next_dp[group][max_count] += dp[group + 1][0];
                    }
                }
            }
            std::mem::swap(&mut dp, &mut next_dp);
        }

        dp[0][0]
    }

    fn part_two(&self) -> SpringSchematic {
        let mut groups = self.groups.clone();
        groups.push('?');

        let mut new_groups: Vec<char> = vec![groups; 5].into_iter().flatten().collect();
        new_groups.pop();
        let new_sizes = vec![self.sizes.clone(); 5].into_iter().flatten().collect();

        SpringSchematic {
            groups: new_groups,
            sizes: new_sizes,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = parse_input(input);
    Some(result.iter().map(|x| x.combinations()).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = parse_input(input);
    let result = result
        .iter()
        .map(|s| s.part_two())
        .map(|x| x.combinations())
        .sum();
    Some(result)
}

fn parse_input(input: &str) -> Vec<SpringSchematic> {
    input
        .lines()
        .map(|line| {
            let parts: &str = line.split(" ").collect::<Vec<&str>>()[0];
            let condition = parts.chars().collect();
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
        assert_eq!(result, Some(525152));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.len(), 6);
    }
}
