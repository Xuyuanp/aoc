pub mod day1;
pub mod day2;

pub trait Solution {
    type Item;
    type Acc;
    type Output;

    const EXAMPLE: &'static str;
    const INPUT: &'static str;

    fn calculate(init: Self::Acc, input: Self::Item) -> Self::Acc;

    fn run_example(&self) -> Self::Output
    where
        Self::Item: From<&'static str>,
        Self::Acc: Default,
        Self::Output: From<Self::Acc>,
    {
        Self::EXAMPLE
            .lines()
            .map(Self::Item::from)
            .fold(Self::Acc::default(), Self::calculate)
            .into()
    }

    fn run(&self) -> Self::Output
    where
        Self::Item: From<&'static str>,
        Self::Acc: Default,
        Self::Output: From<Self::Acc>,
    {
        Self::INPUT
            .lines()
            .map(Self::Item::from)
            .fold(Self::Acc::default(), Self::calculate)
            .into()
    }
}
