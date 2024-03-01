use crate::Solution;

struct Part1;

#[derive(Default)]
struct EnginSchematic {
    matrix: Vec<Vec<char>>,
}

impl EnginSchematic {
    fn height(&self) -> usize {
        self.matrix.len()
    }

    fn width(&self) -> usize {
        self.matrix[0].len()
    }

    fn neighbours(&self, row: usize, range: (usize, usize)) -> Vec<char> {
        let mut result = vec![];
        if row > 0 {
            for i in range.0..range.1 {
                result.push(self.matrix[row - 1][i]);
            }
            if range.0 > 0 {
                result.push(self.matrix[row - 1][range.0 - 1]);
            }
            if range.1 < self.width() {
                result.push(self.matrix[row - 1][range.1]);
            }
        }

        if range.0 > 0 {
            result.push(self.matrix[row][range.0 - 1]);
        }
        if range.1 < self.width() {
            result.push(self.matrix[row][range.1]);
        }

        if row < self.height() - 1 {
            for i in range.0..range.1 {
                result.push(self.matrix[row + 1][i]);
            }
            if range.0 > 0 {
                result.push(self.matrix[row + 1][range.0 - 1]);
            }
            if range.1 < self.width() {
                result.push(self.matrix[row + 1][range.1]);
            }
        }
        result
    }

    fn is_valid(&self, row: usize, range: (usize, usize)) -> bool {
        self.neighbours(row, range)
            .iter()
            .any(|&c| !(c.is_ascii_digit() || c == '.'))
    }

    fn result(&mut self) -> usize {
        let mut numbers = vec![];
        for i in 0..self.height() {
            let mut start = None;
            let mut number = 0;

            for j in 0..self.width() {
                let c = self.matrix[i][j];
                if let Some(n) = c.to_digit(10) {
                    if start.is_none() {
                        start = Some(j);
                    }
                    number = number * 10 + (n as usize);
                } else if let Some(s) = start {
                    if self.is_valid(i, (s, j)) {
                        numbers.push(number);
                    }
                    number = 0;
                    start = None;
                }
            }
            if let Some(s) = start {
                if self.is_valid(i, (s, self.width())) {
                    numbers.push(number);
                }
            }
        }
        numbers.iter().sum()
    }

    fn accept(&mut self, line: &str) {
        self.matrix.push(line.chars().collect());
    }
}

impl From<EnginSchematic> for usize {
    fn from(value: EnginSchematic) -> Self {
        let mut value = value;
        value.result()
    }
}

impl Solution for Part1 {
    type Item = &'static str;

    type Acc = EnginSchematic;

    type Output = usize;

    const EXAMPLE: &'static str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    const INPUT: &'static str = include_str!("input.txt");

    fn calculate(acc: Self::Acc, item: Self::Item) -> Self::Acc {
        let mut acc = acc;
        acc.accept(item);
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sol = Part1;
        assert_eq!(sol.run_example(), 4361);
        assert_eq!(sol.run(), 4361);
    }
}
