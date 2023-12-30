fn main() {
    //let input = include_str!("sample.txt");
    let input = include_str!("input.txt");

    let histories: Vec<History> = input.lines().map(History::from).collect();

    dbg!(&histories);

    let part_1: i32 = histories.iter()
        .map(History::predict)
        .sum();

    println!("part 1: {}", part_1);

    let part_2: i32 = histories.iter()
        .map(History::postdict)
        .inspect(|v| {println!("{}", v);})
        .sum();

    println!("part 2: {}", part_2);

}

#[derive(Debug)]
struct History {
    series: Vec<Vec<i32>>,
}

impl History {
    fn extrapolate_all(&mut self) {
        loop {
            let new = self.extrapolate_once();
            let fin = new.iter().all(|&value| value == 0);
            self.series.push(new);
            if fin {break;}
        }
    }

    fn extrapolate_once(&self) -> Vec<i32> {
        let target = &self.series[self.series.len() - 1];

        target.iter()
            .fold((None::<i32>, Vec::new()), |(p, mut acc), &v| {
                if let Some(prev) = p {
                    acc.push(v - prev)
                }

                (Some(v), acc)
        }).1
    }

    fn predict(&self) -> i32 {
        self.series.iter()
            .rev()
            .fold(0, |mut acc, values| {
                let last = values[values.len() - 1];
                acc = last + acc;
                acc
            })
    }

    fn postdict(&self) -> i32 {
        self.series.iter()
            .rev()
            .fold(0, |mut acc, values| {
                let first = values[0];
                acc = first - acc;
                acc
            })
    }
}

impl From<&str> for History {
    fn from(value: &str) -> Self {
        let readings: Vec<i32> = value
            .split_ascii_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        let mut history = Self {
            series: vec![readings],
        };

        history.extrapolate_all(); // bit weird, would have been better as a pure function...
        history
    }
}
