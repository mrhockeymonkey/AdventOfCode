use std::collections::HashMap;

// Notes
// ???.### 1,1,3 could be represented as a regex .*#.+#.+###.*$
// #.#.### matches
// .##.### does not

// this regex can be represented as a NFA (Non-deterministic Finite Automata)
// whereby as we process each character and keep track of the state machine we can determine if it is valid if we reach the end state

// instead of complicated regex control structures such as + & * we can model this state machine as if a '?' matches either working or broken (~= '\.|#')
// so when we reach a ? we split the head of the state machine into two and explore both paths
// the end result will be many states, some "complete" and some not.

// Links
// https://swtch.com/~rsc/regexp/regexp1.html
// https://www.reddit.com/r/adventofcode/comments/18ge41g/comment/kd3rclt/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
// https://github.com/ConcurrentCrab/AoC/blob/main/solutions/12-1.go
// https://alexoxorn.github.io/posts/aoc-day12-regular_languages/#part-1-the-simple-way


fn main() {

    assert_eq!(count_valid(ConditionRecord::from("???.### 1,1,3")), 1);  
    assert_eq!(count_valid(ConditionRecord::from(".??..??...?##. 1,1,3")), 4);  
    assert_eq!(count_valid(ConditionRecord::from("?#?#?#?#?#?#?#? 1,3,1,6")), 1);  
    assert_eq!(count_valid(ConditionRecord::from("????.#...#... 4,1,1")), 1);  
    assert_eq!(count_valid(ConditionRecord::from("????.######..#####. 1,6,5")), 4
);  
    assert_eq!(count_valid(ConditionRecord::from("?###???????? 3,2,1")), 10);  

    let input = include_str!("input.txt");

    let part_1: u64 = input.lines()
        .map(ConditionRecord::from)
        .map(count_valid)
        .sum();

    println!("part 1: {}", part_1);


    let part_2: u64 = input.lines()
        .map(ConditionRecord::from_unfolded)
        .map(count_valid)
        .sum();

    println!("part 2: {}", part_2);

}

fn count_valid(record: ConditionRecord) -> u64 {
    simulate_nfa2(&record).iter()
        // states that have consumed all damaged groups are considered complete
        .filter_map(|(k, v)| if k.gi == record.damaged_grouping.len() {Some(v)} else {None} )
        .sum::<u64>()
}

#[derive(Debug)]
struct ConditionRecord {
    springs: String,
    damaged_grouping: Vec<u32>
}

impl ConditionRecord {
    fn from_unfolded(value: &str) -> Self {
        let (l, r) = value.split_once(' ').unwrap();
        let ul = [l; 5].join("?");
        let ur = [r; 5].join(",");
        ConditionRecord::from(format!("{} {}", ul, ur).as_ref())
    }
}

impl From<&str> for ConditionRecord {
    fn from(value: &str) -> Self {
        let (l, r) = value.split_once(' ').unwrap();

        Self {
            springs: l.to_string(),
            damaged_grouping: r.split(',').map(|c| c.parse().unwrap()).collect()
        }
    }
}


#[derive(Clone, PartialEq, Debug, Eq, Hash)]
enum Condition {
    Working,
    Broken,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    gi: usize, // grouping index
    dc: u32, // damaged counter 
    expect: Option<Condition>, 
}

impl State {
    fn progress(&self, condition: Condition, record: &ConditionRecord) -> Option<Self> {
        // in some cases we must have a particular char next in order for the state to be valid
        if let Some(expected) = &self.expect {
            if &condition != expected {
                return None
            }
        }

        match condition {
            Condition::Working => {
                Some(Self{ gi: self.gi, dc: self.dc, expect: None})
            },
            Condition::Broken => {
                if self.gi == record.damaged_grouping.len() {
                    // we dont expect there to be any more broken at this point
                    return None
                }

                let dc = self.dc + 1;
                if dc == record.damaged_grouping[self.gi] {
                    // we have come to the end of a group and so are guaranteed a '.' next
                    Some(Self{gi: self.gi + 1, dc: 0, expect: Some(Condition::Working)})
                }
                else {
                    // we are in the middle of a group and so are guaranteed a '#' next
                    Some(Self{ gi: self.gi, dc: dc, expect: Some(Condition::Broken)})
                }
            }
        }
    }
}

fn simulate_nfa2(record: &ConditionRecord) -> HashMap<State, u64> {

    let mut cstates: HashMap<State, u64> = HashMap::new();
    let start = State{
        gi: 0,
        dc: 0,
        expect: None
    };
    cstates.insert(start, 1);

    fn track_next_state(nstates: &mut HashMap<State, u64>, next: Option<State>, count: u64) {
        if let Some(s) = next {
            *nstates.entry(s).or_insert(0) += count;
        };
    }

    for c in record.springs.chars() {
        let mut nstates: HashMap<State, u64> = HashMap::new();
        // process the each head/state
        for (state, &count) in cstates.iter() {
            match c {
                '#' => track_next_state(&mut nstates, state.progress(Condition::Broken, &record), count),
                '.' => track_next_state(&mut nstates, state.progress(Condition::Working, &record), count),
                '?' => {
                    track_next_state(&mut nstates, state.progress(Condition::Working, &record), count);
                    track_next_state(&mut nstates, state.progress(Condition::Broken, &record), count);
                }
                _ => unreachable!()
            }
        }
        //println!("After processing {} the next states are:", c);
        //dbg!(&nstates);
        cstates = nstates;
    }

    cstates
}