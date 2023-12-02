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

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(Game::from)
        .filter(Game::is_possible)
        .map(|g| g.id)
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(Game::from)
        .map(|g| g.min_set())
        .map(|s| s.power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("input.txt");
        assert_eq!(part1(input), 2156);
    }

    #[test]
    fn test_part2_example() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(part2(input), 2286);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("input.txt");
        assert_eq!(part2(input), 66909);
    }
}
