use std::collections::HashSet;

advent_of_code::solution!(14);

struct Dish {
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
}

impl Dish {
    fn solve(&mut self, direction: (isize, isize)) -> Option<u32> {
        self.roll(direction);
        Some(self.weight())
    }

    fn solve_two(&mut self) -> Option<u32> {
        // Find when it repeats itself
        // let mut states = HashSet::new();

        for _ in 0..1_000_000_000 {
            self.roll_cycle();
        }
        Some(self.weight())
    }

    fn roll_cycle(&mut self) -> () {
        self.roll((0, 1));
        self.roll((-1, 0));
        self.roll((0, -1));
        self.roll((1, 0));
    }

    fn roll(&mut self, direction: (isize, isize)) -> () {
        let (row_diff, col_diff) = direction;
        self.rows = self.roll_helper(row_diff, &self.rows);
        self.cols = self.roll_helper(col_diff, &self.cols);
    }

    fn roll_helper(&self, direction: isize, section: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        if direction == 0 {
            return section.clone();
        }
        (0..section[0].len())
            .map(|i| {
                let mut new_sec = vec![];
                for slide in section[i].split(|&c| c == '#') {
                    let rocks = slide.iter().filter(|&&c| c == 'O').count();
                    let mut new_slide = vec![];

                    if direction > 0 {
                        new_slide.extend(vec!['O'; rocks]);
                        new_slide.extend(vec!['.'; slide.len() - rocks]);
                    } else {
                        new_slide.extend(vec!['.'; slide.len() - rocks]);
                        new_slide.extend(vec!['O'; rocks]);
                    }
                    new_sec.extend(new_slide);
                    new_sec.push('#');
                }
                new_sec.pop();
                new_sec
            })
            .collect()
    }

    fn weight(&self) -> u32 {
        let height = self.cols.len();
        self.cols
            .iter()
            .map(|c| {
                c.iter()
                    .enumerate()
                    .map(|(i, &c)| match c {
                        'O' => height - i,
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut dish = parse_input(input);
    dish.solve((0, 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut dish = parse_input(input);
    dish.solve_two()
}

fn parse_input(input: &str) -> Dish {
    let rows: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let cols = (0..rows[0].len())
        .map(|col| (0..rows.len()).map(|row| rows[row][col]).collect())
        .collect();

    Dish { rows, cols }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.cols.len(), 10);
    }
}
