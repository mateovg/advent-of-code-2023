advent_of_code::solution!(13);
// To *summarize* your pattern notes, add up *the number of columns* to the left of each vertical line of reflection;
// to that, also add *100 multiplied by the number of rows* above each horizontal line of reflection.
// In the above example, the first pattern's vertical line has `5` columns to its left and the second
// pattern's horizontal line has `4` rows above it, a total of `*405*`.
#[derive(Debug)]
struct Landscape {
    cols: Vec<Vec<u32>>,
    rows: Vec<Vec<u32>>,
}
impl Landscape {
    fn solve_one(&self) -> u32 {
        let (v, h) = self.find_reflections(0);
        if !v.is_empty() {
            return v[0];
        }
        h[0] * 100
    }

    fn solve_two(&self) -> u32 {
        let (v, h) = self.find_reflections(1);
        if !v.is_empty() {
            return v[0];
        }
        h[0] * 100
    }

    fn find_reflections(&self, diffs: u32) -> (Vec<u32>, Vec<u32>) {
        let h = Self::find_reflections_help(&self.rows, diffs);
        let v = Self::find_reflections_help(&self.cols, diffs);
        (v, h)
    }

    fn find_reflections_help(landscape: &Vec<Vec<u32>>, diffs: u32) -> Vec<u32> {
        (1..landscape.len())
            .filter(|i| Self::diffs_from_reflection(landscape, *i) as u32 == diffs)
            .map(|i| i as u32)
            .collect()
    }
    fn diffs_from_reflection(landscape: &Vec<Vec<u32>>, mirror: usize) -> usize {
        // Find how many spots need to be changed for that mirror
        // to be valid. 0 means it's valid
        let mut diffs = 0;
        let length = mirror.min(landscape.len() - mirror);

        for i in 0..length {
            let top_row = &landscape[mirror - 1 - i];
            let bottom_row = &landscape[mirror + i];
            for (a, b) in top_row.iter().zip(bottom_row) {
                if a != b {
                    diffs += 1;
                }
            }
        }
        diffs
    }
    fn new(input: &str) -> Landscape {
        let rows: Vec<Vec<u32>> = input.lines().
            filter(|l| !l.is_empty()).
            map(|l| {
                l.chars().map(|c|
                match c {
                    '#' => 1,
                    _ => 0,
                }).collect()
            }
            ).collect();

        let num_cols = rows[0].len();
        let cols = (0..num_cols).map(|col| {
            (0..rows.len())
                .map(|row| rows[row][col])
                .collect()
        }).collect();

        Landscape {
            rows,
            cols,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let landscapes = parse_input(input);
    Some(landscapes.iter().map(|l| l.solve_one()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let landscapes = parse_input(input);
    Some(landscapes.iter().map(|l| l.solve_two()).sum())
}

fn parse_input(input: &str) -> Vec<Landscape> {
    input
        .split("\n\r\n")
        .map(Landscape::new)
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
}
