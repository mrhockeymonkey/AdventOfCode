
fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    // using our own fn
    // let input = read_input().unwrap();

    // using built in macro
    let input = include_str!("../input.txt");

    // let lines = input.lines();
    // loop {
    //     // let maybe_line = lines.next();
    //     // match maybe_line {
    //     //     Some(s) => {
    //     //         println!("Got line: {s}")
    //     //     },
    //     //     None => break
    //     // }
    //
    //     if let Some(s) = lines.next() {
    //         println!("Got line: {s}")
    //     } else {
    //         break
    //     }
    // }

    for group in include_str!("../input.txt").split("\n\n") {
        println!("Group:");
        for line in group.lines() {
            let value = line.parse::<u64>()?;
            println!("  - {value}")
        }
    }

    let lines = include_str!("../input.txt")
        .lines()
        .map(|l| l.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let groups = lines
        .split(|v| v.is_none())
        .map(|g| g
            .iter()
            .map(|v| v.unwrap())
            .sum::<u64>())
        .collect::<Vec<_>>();

    let top = groups.iter().max();

    println!("{groups:?}");
    println!("Max is {top:?}");

    let mut copy = groups.clone();
    copy.sort_by(|a,b| b.cmp(a));
    let top3_combined = copy.iter().take(3).sum::<u64>();
    println!("Top 3 combined is {top3_combined:?}");

    Ok(())
}

// fn read_input() -> color_eyre::Result<String> {
//     let path = "input.txtt";
//     // ? operator returns early and propagates Err
//     let input = std::fs::read_to_string(path)?;
//     Ok(input)
// }

// without color_eyre we can create our own error types

// #[derive(Debug)]
// struct InputError {
//     path: String,
//     error: std::io::Error
// }

// fn read_input() -> Result<String, InputError> {
//     let path = "input.txtt";
//     match std::fs::read_to_string(path) {
//         Ok(s) => Ok(s),
//         Err(e) => Err(InputError{path: path.into(), error: e})
//     }
// }

