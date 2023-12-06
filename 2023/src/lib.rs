pub mod day1;
pub mod day2;

pub trait Solution {
    type Item;
    type Acc;
    type Output;

    const EXAMPLE: &'static str;
    const INPUT: &'static str;

    fn calculate(acc: Self::Acc, item: Self::Item) -> Self::Acc;

    fn run_example(&self) -> Self::Output
    where
        Self::Item: From<&'static str>,
        Self::Acc: Default,
        Self::Output: From<Self::Acc>,
    {
        self._run(Self::EXAMPLE)
    }

    fn run(&self) -> Self::Output
    where
        Self::Item: From<&'static str>,
        Self::Acc: Default,
        Self::Output: From<Self::Acc>,
    {
        self._run(Self::INPUT)
    }

    fn _run(&self, input: &'static str) -> Self::Output
    where
        Self::Item: From<&'static str>,
        Self::Acc: Default,
        Self::Output: From<Self::Acc>,
    {
        input
            .lines()
            .map(Self::Item::from)
            .fold(Self::Acc::default(), Self::calculate)
            .into()
    }
}
