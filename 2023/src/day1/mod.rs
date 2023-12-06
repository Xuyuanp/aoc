use crate::Solution;

struct Part1;

impl Solution for Part1 {
    type Item = &'static str;
    type Acc = usize;
    type Output = Self::Acc;

    const EXAMPLE: &'static str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    const INPUT: &'static str = include_str!("input.txt");

    fn calculate(acc: Self::Acc, line: Self::Item) -> Self::Acc {
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
        acc + first * 10 + last
    }
}

static NUMBERS: [&str; 10] = [
    "zero", // placeholder
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn replace_number(s: &str) -> Option<u8> {
    NUMBERS
        .iter()
        .enumerate()
        .skip(1) // skip placeholder
        .map(|(digit, prefix)| (b'0' + digit as u8, prefix))
        .find_map(|(digit, prefix)| s.starts_with(prefix).then_some(digit))
}

fn find_first_digit(line: &str, rev: bool) -> Option<usize> {
    fn helper<'a>(iter: impl Iterator<Item = (&'a str, u8)>) -> Option<usize> {
        iter.map(|(s, c)| replace_number(s).unwrap_or(c))
            .filter(|c| c.is_ascii_digit())
            .map(|c| (c - b'0') as usize)
            .next()
    }

    let iter = line.bytes().enumerate().map(|(i, c)| (&line[i..], c));
    if rev {
        helper(iter.rev())
    } else {
        helper(iter)
    }
}

struct Part2;

impl Solution for Part2 {
    type Item = &'static str;
    type Acc = usize;
    type Output = Self::Acc;

    const EXAMPLE: &'static str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    const INPUT: &'static str = Part1::INPUT;

    fn calculate(acc: Self::Acc, line: Self::Item) -> Self::Acc {
        let first = find_first_digit(line, false).unwrap();
        let last = find_first_digit(line, true).unwrap();
        acc + first * 10 + last
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
