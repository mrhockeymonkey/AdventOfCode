use std::cmp::Ordering;
use std::collections::HashMap;
use crate::Card::{Ace, Eight, Five, Four, Jack, King, Nine, Queen, Seven, Six, Ten, Three, Two};
use crate::HandType::{FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair};

fn main() {
    //let input = include_str!("sample.txt");
    let input = include_str!("input.txt");

    let mut hands: Vec<_> = input.lines()
        .map(Hand::from)
        // .inspect(|h| {
        //     dbg!(h);
        // })
        .collect();

    hands.sort();
    hands.iter().for_each(|h|{
       println!("{:?} - {:?}{:?}{:?}{:?}{:?} {}", h.hand_type, h.cards[0], h.cards[1],h.cards[2],h.cards[3],h.cards[4], h.bid);
    });
    //dbg!(&hands);

    let part_1: u32 = hands.iter().enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .sum();
    println!("part 1 answer: {}", part_1);
}

// part 1 order
// #[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash)]
// enum Card {
//     Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
// }

// part 2 order
#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum Card {
    Jack, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Queen, King, Ace,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    hand_type: HandType
}

impl Hand {
    // part 2 hand type
    fn hand_type_2(cards: &Vec<Card>) -> HandType {
        let mut distributions: HashMap<&Card, u32> = cards.iter()
            .fold(HashMap::new(), |mut acc, card| {
                acc.entry(card)
                    .and_modify(|count| { *count += 1; })
                    .or_insert(1_u32);
                acc
            });

        let wildcards = distributions.remove(&Card::Jack).or(Some(0)).unwrap();
        let mut others: Vec<u32> = distributions.into_values().collect();

        others.sort_by(|a, b| b.cmp(a));
        if let Some(i) = others.get_mut(0) {
            *i += wildcards;
        }
        else {
            others.push(wildcards)
        }
        // others[0] += wildcards;
        Hand::count(others.as_slice())
    }

    // part 1 hand type
    fn hand_type(cards: &Vec<Card>) -> HandType {
        let mut distributions: Vec<u32> = cards.iter()
            .fold(HashMap::new(), |mut acc, card| {
                acc.entry(card)
                    .and_modify(|count| {*count += 1;} )
                    .or_insert(1_u32);
                acc
            })
            .into_values()
            .collect();

        distributions.sort_by(|a, b | b.cmp(a));

        Hand::count(distributions.as_slice())
    }

    fn count(dist: &[u32]) -> HandType {
        match dist {
            [5] => FiveOfKind,
            [4, _] => FourOfKind,
            [3, 2] => FullHouse,
            [3, ..] => ThreeOfKind,
            [2, 2, ..] => TwoPair,
            [2, ..] => OnePair,
            [1, ..] => HighCard,
            _ => unreachable!()
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if other.hand_type == self.hand_type {
            for i in 0..5 {
                let c = self.cards[i].cmp(&other.cards[i]);
                if c != Ordering::Equal {
                    return c
                }
            }
            Ordering::Equal
        }
        else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut split = value.split(' ');
        let mut cards: Vec<Card> = split.next().unwrap().chars().map(|c| match c {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Jack,
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            _ => unreachable!()
        }).collect();
        //cards.sort();
        let bid = split.next().unwrap().parse::<u32>().unwrap();

        Hand {
            bid,
            hand_type: Hand::hand_type_2(&cards),
            cards: cards.try_into().unwrap(),
        }
    }
}
