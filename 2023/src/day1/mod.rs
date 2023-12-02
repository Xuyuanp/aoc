use crate::Solution;

struct Part1;

impl Solution for Part1 {
    type Item = &'static str;
    type Output = usize;
    const EXAMPLE: &'static str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    const INPUT: &'static str = include_str!("input.txt");

    fn calculate(init: Self::Output, line: Self::Item) -> Self::Output {
        let first = line
            .bytes()
            .filter(|c| c.is_ascii_digit())
            .map(|c| (c - b'0') as usize)
            .next()
            .unwrap();
        let last = line
            .bytes()
            .rev()
            .filter(|c| c.is_ascii_digit())
            .map(|c| (c - b'0') as usize)
            .next()
            .unwrap();
        init + first * 10 + last
    }
}

static NUMBERS: [&str; 10] = [
    "zero", // placeholder
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_first_digit(line: &str) -> Option<usize> {
    line.bytes()
        .enumerate()
        .map(|(i, c)| {
            NUMBERS
                .iter()
                .enumerate()
                .map(|(digit, prefix)| (b'0' + digit as u8, prefix))
                .find_map(|(digit, prefix)| {
                    if line[i..].starts_with(prefix) {
                        Some(digit)
                    } else {
                        None
                    }
                })
                .unwrap_or(c)
        })
        .filter(|c| c.is_ascii_digit())
        .map(|c| (c - b'0') as usize)
        .next()
}

fn find_last_digit(line: &str) -> Option<usize> {
    line.bytes()
        .enumerate()
        .rev()
        .map(|(i, c)| {
            NUMBERS
                .iter()
                .enumerate()
                .map(|(digit, prefix)| (b'0' + digit as u8, prefix))
                .find_map(|(digit, prefix)| {
                    if line[i..].starts_with(prefix) {
                        Some(digit)
                    } else {
                        None
                    }
                })
                .unwrap_or(c)
        })
        .filter(|c| c.is_ascii_digit())
        .map(|c| (c - b'0') as usize)
        .next()
}

struct Part2;

impl Solution for Part2 {
    type Item = &'static str;
    type Output = usize;
    const EXAMPLE: &'static str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    const INPUT: &'static str = Part1::INPUT;

    fn calculate(init: Self::Output, line: Self::Item) -> Self::Output {
        let first = find_first_digit(line).unwrap();
        let last = find_last_digit(line).unwrap();
        init + first * 10 + last
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sol = Part1;
        assert_eq!(142, sol.run_example());
        assert_eq!(55971, sol.run());
    }

    #[test]
    fn test_part2() {
        let sol = Part2;
        assert_eq!(281, sol.run_example());
        assert_eq!(54719, sol.run());
    }
}
