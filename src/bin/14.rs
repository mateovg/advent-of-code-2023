use std::collections::HashMap;

advent_of_code::solution!(14);

struct Dish {
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
}
impl Dish {
    fn solve(&mut self, direction: (isize, isize)) -> Option<usize> {
        self.roll(direction);
        Some(self.weight())
    }

    fn solve_two(&mut self) -> Option<usize> {
        let cycles_left = self.find_cycles_left();
        for _ in 0..cycles_left {
            self.roll_cycle();
        }
        Some(self.weight())
    }

    fn find_cycles_left(&mut self) -> usize {
        let (start, cycles) = self.find_cycle_start();
        let cycle_length = cycles - start;
        (1_000_000_000 - cycles) % (cycle_length)
    }

    fn find_cycle_start(&mut self) -> (usize, usize) {
        let mut states = HashMap::new();

        for cycles in 1..1_000_000_000 {
            self.roll_cycle();
            let current_state = self.get_state();
            if let Some(start) = states.get(&current_state) {
                return (*start as usize, cycles as usize);
            }
            states.insert(current_state.clone(), cycles);
        }
        (0, 1_000_000_000)
    }

    fn roll_cycle(&mut self) -> () {
        self.roll((0, 1));
        self.roll((1, 0));
        self.roll((0, -1));
        self.roll((-1, 0));
    }

    fn roll(&mut self, (row_diff, col_diff): (isize, isize)) -> () {
        if row_diff != 0 {
            self.rows = self.roll_helper(row_diff, &self.rows);
            self.cols = transpose(&self.rows);
        } else {
            self.cols = self.roll_helper(col_diff, &self.cols);
            self.rows = transpose(&self.cols);
        }

        assert_eq!(self.rows, transpose(&self.cols));
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

    fn get_state(&self) -> Vec<Vec<char>> {
        self.rows.clone()
    }

    fn weight(&self) -> usize {
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
            .sum::<usize>() as usize
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut dish = parse_input(input);
    dish.solve((0, 1))
}

pub fn part_two(input: &str) -> Option<usize> {
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

fn transpose(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..v[0].len())
        .map(|col| (0..v.len()).map(|row| v[row][col]).collect())
        .collect()
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
