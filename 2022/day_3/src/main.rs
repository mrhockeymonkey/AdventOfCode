use std::collections::HashSet;
use color_eyre::owo_colors::OwoColorize;
use itertools::Itertools;
use item::Item;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let _a = Item::try_from(b'a')?;
    //let _exclaim = Item(b'!');

    // for thing in b'a'..=b'z' {
    //     let item = Item::try_from(thing)?;
    //     println!("{thing} is {} with priority {}", thing as char, item.priority());
    // }

    for thing in b'A'..=b'Z' {
        let item = Item::try_from(thing)?;
        let pri = item.priority();
        println!("{item:?} is {} with priority {}", thing, pri);
    }

    // part 1
    let mut total_priority = 0;
    for line in include_str!("input.txt").lines() {

        let (first, second) = line.split_at(line.len() / 2);
        let first_items = first
            .bytes()
            .map(Item::try_from)
            .collect::<Result<Vec<_>,_>>()?;
        let second_items = second
            .bytes()
            .map(Item::try_from)
            .collect::<Result<Vec<_>,_>>()?;

        let dupe_item = second
            .bytes()
            .map(Item::try_from)
            .find_map(|item| {
                item.ok().and_then(|item| {
                    first_items.iter()
                        .copied()
                        .find(|&fi| fi == item) // here &fi is a pattern that matches to a reference to an item
                })
            })
            .expect("There should be exactly one duplicate");

        println!("- {first_items:?} | {second_items:?} --- dupe item is {dupe_item:?} so has priority {}", dupe_item.priority());
        total_priority += dupe_item.priority();
    }

    dbg!(total_priority);

    // part 2 -- with itertools
    // let rucksacks = include_str!("input.txt")
    //     .lines()
    //     .map(|line| {
    //         line.bytes()
    //             .map(Item::try_from)
    //             .collect::<Result<HashSet<_>, _>>()
    //     });
    //
    // let sum = itertools::process_results(rucksacks, |rs| {
    //     rs.tuples()// groups iterable into tuples
    //         .map(|(a, b, c)| {
    //             a.iter()
    //                 .copied()
    //                 .find(|i| b.contains(i) && c.contains(i))
    //                 .map(|i| dbg!(i.priority()))
    //                 .unwrap_or_default()
    //         })
    //         .sum::<u32>()
    // });
    //
    // dbg!(sum);

    // part 2 - without itertools (could be nicer if using im::HashSet)

    let sum = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.bytes()
                //.map(Item::try_from)// this returns a result which cant be used to collect to hashSet
                .map(|b| b.try_into().unwrap()) // this is ignoring the fact conversion is fallible
                .collect::<HashSet<Item>>()
        })
        .chunks(3)
        .into_iter() // into_iter moves (iter would use references)
        .map(|chunk| {
            chunk.reduce(|a, b| a.intersection(&b)
                    .cloned()// intersection yields refs to items so we need to clone to match signiture or reduce
                    .collect())
                .expect("that there is an intersection between the groups")
                .iter()
                .next()
                .expect("there should be only one item in common")
                .priority()
        })
        .sum::<u32>();

    dbg!(sum);



    Ok(())
}

mod item {
    use std::fmt::{Debug, Formatter};

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Item(u8);

    impl Item {
        pub fn priority(self) -> u32 {
            // because this has the copy trait self here is a copy on the stack
            match self {
                Item(b'a'..=b'z') => 1 + (self.0 - b'a' ) as u32,
                Item(b'A'..=b'Z') => 27 + (self.0 - b'A' ) as u32,
                _ => unreachable!()
            }
        }
    }

    impl TryFrom<u8> for Item {
        type Error = color_eyre::Report;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
                _ => Err(color_eyre::eyre::eyre!("{} is not a valid item", value as char))
            }
        }
    }

    impl Debug for Item {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0 as char)
        }
    }
}

