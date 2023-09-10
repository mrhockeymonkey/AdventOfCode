use std::ops::RangeInclusive;
use iter_tools::Itertools;

fn main() {
    dbg!((2..=4).contains_range(&(6..=8)));
    dbg!((6..=8).contains_range(&(2..=4)));
    dbg!((4..=6).contains_range(&(6..=6)));

    let input = include_str!("input.txt")
        .lines()
        .map(|line| line
            .split(',')
            .map(|range| range
                .split('-')
                .map(|n| n
                    .parse()
                    .expect("range start/end should be u32 "))
                .collect_tuple::<(u32,u32)>()
                .map(|(start, end)| start..=end)
                .expect("each range has a start and an end")
            )
            .collect_tuple::<(_, _)>()
            .expect("each line has a pair of ranges")
        );

    let i = input.clone();
    let v = vec![0..10];

    // in how many pairs does one range complete include the other
    let count = input.clone()
        .filter(|(a, b)| a.contains_or_is_contained(b))
        .count();

    dbg!(count);

    // in how many pairs is there anny overlap?
    let count2 = &input
        .filter(|(a, b)| a.overlaps_or_is_overlapped(b))
        .count();

    dbg!(count2);

}

// define out own trait, one fn has a default impl
trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps_range(&self, other: &Self) -> bool;

    fn overlaps_or_is_overlapped(&self,  other: &Self) -> bool {
        self.overlaps_range(other) || other.overlaps_range(self)
    }
}

// implement the trait for an existing type from core lib
impl<T> InclusiveRangeExt for RangeInclusive<T>
where T : PartialOrd {
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps_range(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}
