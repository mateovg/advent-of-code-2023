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
    let sum= input
        .lines()
        .map(|l| read_digits_two(l))
        .map(|nums| get_number(&nums))
        .sum();
    Some(sum)
}

pub fn read_digits(input: &str) -> Vec<u32> {
    input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

pub fn read_digits_two(input: &str) -> Vec<u32>{
    let mut digits = vec![];
    for i in 0..input.len(){
        match &input[i..] {
            l if l.starts_with("1") || l.starts_with("one") => digits.push(1),
            l if l.starts_with("2") || l.starts_with("two") => digits.push(2),
            l if l.starts_with("3") || l.starts_with("three") => digits.push(3),
            l if l.starts_with("4") || l.starts_with("four") => digits.push(4),
            l if l.starts_with("5") || l.starts_with("five") => digits.push(5),
            l if l.starts_with("6") || l.starts_with("six") => digits.push(6),
            l if l.starts_with("7") || l.starts_with("seven") => digits.push(7),
            l if l.starts_with("8") || l.starts_with("eight") => digits.push(8),
            l if l.starts_with("9") || l.starts_with("nine") => digits.push(9),
            _ => continue,
        }
    }
    digits
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

    #[test]
    fn test_read_digits_two() {
        let result = read_digits_two("one3ad931klanine");
        assert_eq!(result, vec![1,3,9,3,1,9]);
    }
}
