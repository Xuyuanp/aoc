pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
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
            first * 10 + last
        })
        .sum()
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

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first = find_first_digit(line).unwrap();
            let last = find_last_digit(line).unwrap();
            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = include_str!("input.txt");
        assert_eq!(55971, part1(input));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("input.txt");
        assert_eq!(54719, part2(input));
    }
}
