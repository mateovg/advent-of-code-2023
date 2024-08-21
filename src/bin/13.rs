use itertools::Itertools;

advent_of_code::solution!(13);

// To *summarize* your pattern notes, add up *the number of columns* to the left of each vertical line of reflection;
// to that, also add *100 multiplied by the number of rows* above each horizontal line of reflection.
// In the above example, the first pattern's vertical line has `5` columns to its left and the second
// pattern's horizontal line has `4` rows above it, a total of `*405*`.
#[derive(Debug)]
struct Pattern {
    pattern: Vec<Vec<char>>,
}
impl Pattern {
    fn find_reflection(&self) -> Option<Vec<u32>> {
        // Only have to find horizontal reflections now
        let reflections: Vec<u32> = (1..self.pattern.len())
            .filter(|i| self.valid_reflection(*i))
            .map(|i| i as u32)
            .collect();
        return if reflections.is_empty() {
            None
        } else {
            Some(reflections)
        };
    }

    fn find_vert_reflection(&self) -> Option<Vec<u32>> {
        let vert = self.transpose();
        vert.find_reflection()
    }

    fn valid_reflection(&self, index: usize) -> bool {
        let rows_above = index;
        let rows_below = self.pattern.len() - index;
        let rows_to_check = rows_above.min(rows_below);

        for i in 0..rows_to_check {
            let top_row = &self.pattern[index - 1 - i];
            let bottom_row = &self.pattern[index + i];
            if top_row != bottom_row {
                return false;
            }
        }
        true
    }

    fn transpose(&self) -> Pattern {
        let transposed_pattern = self.pattern.clone();
        let transposed_pattern = (0..transposed_pattern[0].len())
            .map(|i| {
                transposed_pattern
                    .iter()
                    .map(|inner| inner[i].clone())
                    .collect()
            })
            .collect();
        Pattern {
            pattern: transposed_pattern,
        }
    }

    fn get_reflections(&self) -> (u32, u32) {
        let horizontal = if let Some(h) = self.find_reflection() {
            h[0] as u32
        } else {
            0
        };
        let vertical = if let Some(v) = self.find_vert_reflection() {
            v[0] as u32
        } else {
            0
        };
        (horizontal, vertical)
    }

    fn solve_one(&self) -> u32 {
        let (h, v) = self.get_reflections();
        v + h * 100
    }

    fn solve_two(&self) -> u32 {
        // try changing every #/. until you get a new answer
        let (h, v) = self.get_reflections();
        for (x, y) in (0..self.pattern.len()).cartesian_product(0..self.pattern[0].len()) {
            let mut new_pattern = self.pattern.clone();
            new_pattern[x][y] = match new_pattern[x][y] {
                '#' => '.',
                '.' => '#',
                _ => unreachable!(),
            };
            let new_pattern = Pattern {
                pattern: new_pattern,
            };

            let new_h = if let Some(new_h) = new_pattern.find_reflection() {
                *new_h.iter().filter(|&i| *i != h).next().unwrap_or(&0)
            } else {
                0 as u32
            };

            let new_v = if let Some(new_v) = new_pattern.find_reflection() {
                *new_v.iter().filter(|&i| *i != v).next().unwrap_or(&0)
            } else {
                0 as u32
            };

            let score = new_v + new_h * 100;
            if score != 0 {
                return score;
            }
        }
        0
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let patterns = parse_input(input);
    let ans = patterns.iter().map(|p| p.solve_one()).sum();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let patterns = parse_input(input);
    let ans = patterns.iter().map(|p| p.solve_two()).sum();
    Some(ans)
}

fn parse_input(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|p| Pattern {
            pattern: p.lines().map(|l| l.to_string().chars().collect()).collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_transpose() {
        let result = Pattern {
            pattern: vec!["abccba".to_string().chars().collect()],
        };
        assert_eq!(result.transpose().pattern.len(), 6);
    }

    #[test]
    fn test_mirror() {
        let pattern = Pattern {
            pattern: vec!["abccba".to_string().chars().collect()],
        };
        // assert_eq!(pattern.transpose().find_reflection(), Some(3));
    }
}
