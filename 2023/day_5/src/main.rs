use std::collections::HashMap;
use std::ops::RangeInclusive;

fn main() {
    //let mut sections = include_str!("sample.txt").split("\n\n");
    let mut sections = include_str!("input.txt").split("\n\n");

    // part 1 treats each as a seed number
    let seeds = sections.next().unwrap()[7..]
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // part 2 treats pairs as a range of seeds
    let seeds_ranges: &Vec<RangeInclusive<u64>> = &seeds
        .chunks_exact(2)
        .map(|chunk| {
            if let [start, len] = chunk {
                let end = start + len - 1;
                (start.clone()..=end)
            }
            else {
                unreachable!()
            }
        })
        .collect();

    dbg!(&seeds_ranges);


    let seed_to_soil= get_mapping_ranges(sections.next().unwrap());
    let soil_to_fertilizer= get_mapping_ranges(sections.next().unwrap());
    let fertilizer_to_water= get_mapping_ranges(sections.next().unwrap());
    let water_to_light= get_mapping_ranges(sections.next().unwrap());
    let light_to_temp= get_mapping_ranges(sections.next().unwrap());
    let temp_to_humidity= get_mapping_ranges(sections.next().unwrap());
    let humidity_to_location= get_mapping_ranges(sections.next().unwrap());


    // assert_eq!(map_source_to_destination(&79u32, &seed_to_soil), 81u32);
    // assert_eq!(map_source_to_destination(&14u32, &seed_to_soil), 14u32);
    // assert_eq!(map_source_to_destination(&55u32, &seed_to_soil), 57u32);
    // assert_eq!(map_source_to_destination(&13u32, &seed_to_soil), 13u32);

    // part 1
    let resolved = seeds.iter()
        .map(| src | map_source_to_destination(src, &seed_to_soil))
        .map(| src | map_source_to_destination(&src, &soil_to_fertilizer))
        .map(| src | map_source_to_destination(&src, &fertilizer_to_water))
        .map(| src | map_source_to_destination(&src, &water_to_light))
        .map(| src | map_source_to_destination(&src, &light_to_temp))
        .map(| src | map_source_to_destination(&src, &temp_to_humidity))
        .map(| src | map_source_to_destination(&src, &humidity_to_location))
        .collect::<Vec<_>>();
    dbg!(&resolved);

    let lowest = resolved.iter().min().unwrap();
    println!("part 1 lowest is {}", lowest);

    // part 2
    let lowest2 = seeds_ranges.iter()
        .inspect(|r| {dbg!(r);})
        .map(|range| range.clone() // todo
            .map(| src | map_source_to_destination(&src, &seed_to_soil))
            .map(| src | map_source_to_destination(&src, &soil_to_fertilizer))
            .map(| src | map_source_to_destination(&src, &fertilizer_to_water))
            .map(| src | map_source_to_destination(&src, &water_to_light))
            .map(| src | map_source_to_destination(&src, &light_to_temp))
            .map(| src | map_source_to_destination(&src, &temp_to_humidity))
            .map(| src | map_source_to_destination(&src, &humidity_to_location))
            .reduce(|acc, x| match x {
                x if x < acc => x,
                _ => acc
            })
        )
        .inspect(|ans| {dbg!(ans);})
        .min();

    println!("part 2 lowest is {}", lowest2.unwrap().unwrap());


}

fn get_mapping_ranges(s: &str) -> Vec<MappingRange> {
    println!("{}", s);
    s.lines()
        .skip(1)
        .map(|l| l.split(" ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>())
        .map(|vec| {
            assert!(vec.len() == 3);
            MappingRange{
                source_start: vec[1],
                destination_start: vec[0],
                length: vec[2]
            }
        })
        .collect::<Vec<_>>()
}

fn map_source_to_destination(source: &u64, ranges: &Vec<MappingRange>) -> u64 {
    let mapped = ranges.iter()
        .filter_map(|range| range.map_source(source))
        .collect::<Vec<_>>();

    if mapped.is_empty() {
        source.clone()
    }
    else {
        assert_eq!(mapped.len(), 1);
        mapped.first().unwrap().clone()
    }
}

#[derive(Debug)]
struct MappingRange {
    source_start: u64,
    destination_start: u64,
    length: u64
}

impl MappingRange {
    fn contains_source(&self, source: &u64) -> bool {
        source >= &self.source_start &&
            source <= &(self.source_start + self.length - 1)
    }

    fn map_source(&self,  source: &u64) -> Option<u64> {
        if self.contains_source(source) {
            let offset = source - self.source_start;
            let dest = self.destination_start + offset;
            Some(dest)
        }
        else {
            None
        }
    }
}



// It wasn't a surprise that this takes prohibitively too long for the real input because of the looping...
// fn make_almanac_map(s: &str) -> HashMap<u32, u32> {
//     println!("{}", s);
//     s.lines()
//         .skip(1)
//         .map(|l| l.split(" ")
//             .map(|s| s.parse::<u32>().unwrap())
//             .collect::<Vec<_>>())
//         .fold(HashMap::new(), |mut acc: HashMap<u32, u32>, i| {
//             // bad perf?
//             dbg!(&i);
//             for n in 0..i[2] {
//                 acc.insert(i[1] + n, i[0] + n);
//             }
//             acc
//         })
// }

// trait HashMapExt {
//     fn get_or_key(&self, k: u32) -> u32;
// }
//
// impl HashMapExt for HashMap<u32, u32> {
//     fn get_or_key(&self, k: u32) -> u32 {
//         if self.contains_key(&k) { self[&k] } else { k }
//     }
// }



