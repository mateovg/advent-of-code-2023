advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<isize> {
    let data = parse_input(input);
    let sum = data.iter().map(|d| extrapolate_forward(d.clone())).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<isize> {
    let data = parse_input(input);
    let sum = data.iter().map(|d| extrapolate_backward(d.clone())).sum();
    Some(sum)
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>()
}

fn extrapolate_forward(line: Vec<isize>) -> isize {
    // Reduce to 0s and add the last number in each line
    let mut data = vec![line];
    while data.last().unwrap().iter().any(|&x| x != 0) {
        let last = data.last().unwrap();
        let new = last.windows(2).map(|w| w[1] - w[0]).collect();
        data.push(new);
    }
    data.iter()
        .rev()
        .filter(|&x| !x.is_empty())
        .fold(0, |acc, seq| acc + seq.last().unwrap())
}

fn extrapolate_backward(line: Vec<isize>) -> isize {
    // Reduce to 0s and add the last number in each line
    let mut data = vec![line.clone()];
    while data.last().unwrap().iter().any(|&x| x != 0) {
        let last = data.last().unwrap();
        let new = last.windows(2).map(|w| w[0] - w[1]).collect();
        data.push(new);
    }
    dbg!(&data);
    data.iter()
        .rev()
        .fold(0, |acc, seq| acc + seq.first().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_extrapolate_forward() {
        assert_eq!(extrapolate_forward(vec![0, 3, 6, 9, 12]), 15);
        assert_eq!(extrapolate_forward(vec![1, 3, 6, 10, 15, 21]), 28);
    }
}
