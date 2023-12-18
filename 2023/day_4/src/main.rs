use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    //let input = include_str!("sample.txt");
    let input = include_str!("input.txt");

    let cards = input.lines()
        .map(Card::from)
        .collect::<Vec<_>>();

    // part 1
    let score_sum: u32 = cards.iter()
        .map(|c| c.score())
        .sum();

    println!("sum of scores is {}", score_sum);

    // part 2
    let mut card_count = 0;
    let mut q = VecDeque::from(cards.clone());
    let mut cache = HashMap::new();
    while !q.is_empty() {
        let card = q.pop_front().unwrap();
        card_count += 1;
        let mc = match_count_memo(&mut cache, &card) as usize;
        for i in 0..mc {
            let copy = cards.get(card.id + i).unwrap().clone();
            q.push_back(copy);
        }
    }

    println!("total card count is {}", card_count);
}

fn match_count_memo(cache: &mut HashMap<usize, u32>, card: &Card) -> u32 {
    match cache.get(&card.id) {
        Some(&count) => count,
        None => {
            let count = card.match_count();
            cache.insert(card.id, count);
            count
        }
    }
}

#[derive(Debug, Clone)]
struct Card {
    name: String,
    id: usize,
    winning: Vec<u32>,
    revealed: Vec<u32>,
}

impl Card {
    fn match_count(&self) -> u32 {
        let win_set: HashSet<u32> = self.winning.iter().copied().collect();
        let rev_set: HashSet<u32> = self.revealed.iter().copied().collect();

        win_set.intersection(&rev_set).count() as u32
    }

    fn score(&self) -> u32 {
        match self.match_count() {
            0 => 0,
            n=> 1 << n-1
        }
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {

        let i = value.chars().position(|c| c == ':').unwrap();
        let j = value.chars().position(|c| c == '|').unwrap();

        let name = value[0..i].to_string();
        let id = name.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<usize>().unwrap();
        let winning_str = value[i+2..j-1].to_string();
        let revealed_str = value[j+2..].to_string();

        Card {
            name,
            id,
            winning: winning_str.split(' ').filter_map(|n| n.parse::<u32>().ok()).collect(),
            revealed: revealed_str.split(' ').filter_map(|n| n.parse::<u32>().ok()).collect(),
        }
    }
}
