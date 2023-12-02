use crate::Solution;

#[derive(Debug, Default)]
pub struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

pub struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}

impl CubeSet {
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    pub fn is_possible(&self) -> bool {
        self.red <= Self::MAX_RED && self.green <= Self::MAX_GREEN && self.blue <= Self::MAX_BLUE
    }

    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl Game {
    pub fn is_possible(&self) -> bool {
        self.sets.iter().all(|set| set.is_possible())
    }

    pub fn min_set(&self) -> CubeSet {
        let mut set = CubeSet::default();
        for s in &self.sets {
            set.red = set.red.max(s.red);
            set.green = set.green.max(s.green);
            set.blue = set.blue.max(s.blue);
        }
        set
    }
}

impl<S> From<S> for CubeSet
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        let mut set = Self::default();
        for color in s.as_ref().split(", ") {
            let mut iter = color.split_whitespace();
            let count = iter.next().unwrap().parse::<usize>().unwrap();
            let color = iter.next().unwrap();
            match color {
                "red" => set.red = count,
                "green" => set.green = count,
                "blue" => set.blue = count,
                _ => unreachable!(),
            }
        }
        set
    }
}

impl<S> From<S> for Game
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        let mut iter = s.as_ref().split(": ");
        let id = iter
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let sets = iter
            .next()
            .unwrap()
            .split("; ")
            .map(CubeSet::from)
            .collect();
        Self { id, sets }
    }
}

struct Part1;

impl Solution for Part1 {
    type Item = Game;
    type Output = usize;

    const EXAMPLE: &'static str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    const INPUT: &'static str = include_str!("input.txt");

    fn calculate(init: Self::Output, game: Self::Item) -> Self::Output {
        init + game.is_possible().then_some(game.id).unwrap_or(0)
    }
}

struct Part2;

impl Solution for Part2 {
    type Item = Game;
    type Output = usize;

    const EXAMPLE: &'static str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    const INPUT: &'static str = include_str!("input.txt");

    fn calculate(init: Self::Output, game: Self::Item) -> Self::Output {
        init + game.min_set().power()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sol = Part1;
        assert_eq!(sol.run_example(), 8);
        assert_eq!(sol.run(), 2156);
    }

    #[test]
    fn test_part2() {
        let sol = Part2;
        assert_eq!(sol.run_example(), 2286);
        assert_eq!(sol.run(), 66909);
    }
}
