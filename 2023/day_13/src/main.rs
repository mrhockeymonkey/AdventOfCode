fn main() {
    //let input = include_str!("sample.txt");
    let input = include_str!("input.txt");

    let patterns = input.split("\n\n")
        .map(|s| Pattern::from_str(s))
        .collect::<Vec<_>>();

    let sum = patterns.iter()
        .inspect(|&pattern| {dbg!(&pattern.rows()[0]);})
        .map(|pattern| (
            reflected_count(pattern.rows().iter().map(|r| r.value()).collect()),
            reflected_count(pattern.cols().iter().map(|c| c.value()).collect())
        ))
        .inspect(|result| {dbg!(result);})
        .map(|result | match result {
            (Some(row_count), None) => row_count as u32 * 100,
            (None, Some(col_count)) => col_count as u32,
            _ => unreachable!(),
        })
        .sum::<u32>();

    println!("Summary is {}", sum);


}

struct Pattern {
    grid: Vec<Vec<char>>
}

trait Axis {
    fn chars(&self) -> &[char];
    fn value(&self) -> u32 {
        let binary = self.chars().iter()
            .map(|c| match c {
                '.' => '0',
                '#' => '1',
                _ => unreachable!()
            })
            .collect::<String>();

        u32::from_str_radix(binary.as_str(), 2).unwrap()
    }
}

#[derive(Debug)]
struct Row {
    index: usize,
    chars: Vec<char>,
}

impl Axis for Row {
    fn chars(&self) -> &[char] {
        &self.chars
    }
}

#[derive(Debug)]
struct Col {
    index: usize,
    chars: Vec<char>,
}

impl Axis for Col {
    fn chars(&self) -> &[char] {
        &self.chars
    }
}

impl Pattern {
    fn from_str(s: &str) -> Self {
        let grid = s.lines()
            .map(|line| line.chars()
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Pattern{ grid }
    }

    fn rows(&self) -> Vec<Row> {
        self.grid.iter()
            .enumerate()
            .map(|(i, row)| Row{ index: i + 1, chars: row.clone() })
            .collect()
    }

    fn cols(&self) -> Vec<Col> {
        let transposed: Vec<Vec<char>> = transpose(self.grid.clone());
        transposed.iter()
            .enumerate()
            .map(|(i, col)| Col{ index: i + 1, chars: col.clone()}) // todo
            .collect()
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}


fn reflected_count(values: Vec<u32>) -> Option<usize> {

    let possible = (0..values.len() - 1)
        .filter_map(|(i)| if values[i] == values[i + 1] {Some(i)} else {None})
        .collect::<Vec<_>>();

    let perfect = possible.into_iter()
        .filter_map(|i| {
            let mut j = i as isize;
            let mut k= i + 1;

            while j >= 0 && k < values.len() {

                if values[j as usize] != values[k] {
                    // symmetry broken
                    return None
                }

                j -= 1;
                k += 1;
            }
            Some(i)
        })
        .collect::<Vec<_>>();

    match perfect.as_slice() {
        [] => None,
        [only] => Some(only + 1),
        _ => panic!("")
    }
}

// fn has_reflection(values: &[u32]) -> bool {
//     let len = values.len();
//     assert_eq!(len % 2, 0, "Checking reflection requires even number of elements");
//     let mid = len / 2;
//     let left = &values[0..mid];
//     let mut right = values[mid..len].to_vec();
//     right.reverse();
//
//     left == right
// }
