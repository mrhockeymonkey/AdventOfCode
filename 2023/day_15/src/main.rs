use std::collections::HashMap;

fn main() {
    //let input = include_str!("sample.txt");
    let input = include_str!("input.txt");

    let sum_of_hash: u32 = input.split(',')
        .map(|step| hash(step.as_bytes()))
        .sum();

    println!("sum of hashed steps is {}", sum_of_hash);

    // part 2
    // make boxes, parse steps,
    let mut box_map = (0..256) // could we use internal mutability?
        .map(|id| (id, Box{id, lenses: Vec::new()}))
        .collect::<HashMap<_, _>>();

    input.split(',')
        .map(|step| Step::from(step.as_bytes()))
        .for_each(|step| {
            let h = dbg!(hash(step.label));
            let target_box = box_map.get_mut(&h);
            target_box.unwrap().process(&step);
        });

    let sum_focus_power: u32 = box_map.iter()
        .map(|(i, b)| b.focusing_powers())
        .flatten()
        .sum();

    println!("sum of focusing power is {}", sum_focus_power);


}

// Why I did this to myself I dont know but playing with lifetimes is useful for learning.
#[derive(Debug)]
struct Step<'a> {
    label: &'a[u8],
    op: u8,
    focal_len: u8,
}

impl<'a> Step<'a> {
    fn is_remove(&self) -> bool {
        self.op == 45
    }
}

impl<'a> From<&'a[u8]> for Step<'a> {
    fn from(value: &'a[u8]) -> Self {
        let l = value.len();
         if value[l -1] == 45 { // '-' == 45
             Step {
                 label: value[0..=value.len() -2].as_ref(),
                 op: 45,
                 focal_len: 0,
             }
         }
         else {
             Step {
                 label: value[0..=value.len() -3].as_ref(),
                 op: value[value.len() - 2],
                 focal_len: value[value.len() - 1],
             }
         }
    }
}

#[derive(Debug)]
struct Lens {
    label: String,
    len: u32,
}

struct Box {
    id: u32,
    lenses: Vec<Lens>
}

impl Box {
    fn process(&mut self, step: &Step) {
        let label = String::from_utf8(step.label.to_vec()).unwrap();
        let present = self.lenses.iter().position(|l| l.label == label);

        match (present, step.is_remove()) {
            (Some(i), true) => {
                self.lenses.remove(i);
            },
            (Some(i), false) => self.lenses[i] = Lens { label, len: (step.focal_len -48) as u32 },
            (None, true) => {},
            (None, false) => self.lenses.push(Lens { label, len: (step.focal_len -48) as u32 }),
        }

        dbg!(&self.lenses);
    }

    fn focusing_powers(&self) -> Vec<u32> {
        self.lenses.iter()
            .enumerate()
            .map(|(i, l)| (self.id + 1) * (i as u32 + 1) * l.len)
            .collect()
    }
}

fn hash(data: &[u8]) -> u32 {
    data.iter()
        .fold(0u32, |mut acc, val| {
            acc = acc + *val as u32;
            acc = acc * 17;
            acc = acc % 256;
            acc
        })
}