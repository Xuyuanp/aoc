pub mod day1;
pub mod day2;

pub trait Solution {
    type Item;
    type Output;

    const EXAMPLE: &'static str;
    const INPUT: &'static str;

    fn calculate(init: Self::Output, input: Self::Item) -> Self::Output;

    fn run_example(&self) -> Self::Output
    where
        Self::Item: From<&'static str>,
        Self::Output: Default,
    {
        Self::EXAMPLE
            .lines()
            .map(Self::Item::from)
            .fold(Self::Output::default(), Self::calculate)
    }

    fn run(&self) -> Self::Output
    where
        Self::Item: From<&'static str>,
        Self::Output: Default,
    {
        Self::INPUT
            .lines()
            .map(Self::Item::from)
            .fold(Self::Output::default(), Self::calculate)
    }
}
