advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let sum: u32 = input
        .lines()
        .map(|l| read_digits(l))
        .map(|nums| get_number(&nums))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

pub fn read_digits(input: &str) -> Vec<u32> {
    input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

pub fn get_number(nums: &Vec<u32>) -> u32 {
    let first = nums.first().unwrap();
    let last = nums.last().unwrap_or(first);
    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part_one(input);
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

",
        );
        assert_eq!(result, Some(281));
    }

    #[test]
    fn test_read_digits() {
        let result = read_digits("1abc2");
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_get_numbers() {
        let result = get_number(&vec![1, 2, 0, 9, 2]);
        assert_eq!(result, 12);
    }
}
