fn main() {
    //let input = vec![(7_u64, 9_u64), (15, 40), (30, 200)];
    //let input = vec![(40_u64, 215_u64), (70, 1051), (98, 2147), (79, 1005)];

    //let input = vec![(71530_u64, 940200_u64)];
    let input = vec![(40709879_u64, 215105121471005_u64)];

    let races: Vec<Race> = input.into_iter()
        .map(|(time_ms, distance_ms)| Race{time_ms, distance_ms})
        .collect();

    let part_1: usize = races.iter()
        .map(|race| (race, Race::possible_outcomes(race.time_ms)))
        .map(|(race, possible)| {
            let wins = possible.iter()
                .filter(|p| p.distance_ms > race.distance_ms)
                .collect::<Vec<_>>();

            wins.len()
        })
        .fold(1, |acc, n| acc * n);

    println!("part 1 answer: {}", part_1);
}

#[derive(Debug)]
struct Race {
    time_ms: u64,
    distance_ms: u64,
}

impl Race {
    fn possible_outcomes(time: u64) -> Vec<Race>  {
        (0..=time).into_iter()
            .map(|t| {
                let race_time = time - t;
                Race {
                    time_ms: time,
                    distance_ms: t * race_time
                }
            })
            .collect()
    }
}

