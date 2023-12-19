use std::{collections::HashMap};

use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day15 {
    input: String,
}

impl FromInput for Day15 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Day15 {
            // input : String::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            input: lines.collect(),
        }
    }
}

fn hashfn(input: &str) -> u32 {
    let mut current = 0_u32;
    for c in input.chars() {
        current += c as u32;
        current *= 17;
        current %= 256;
    }
    current
}

fn find<'a>(
    label: &'a str,
    hashmap: &'a HashMap<u32, Vec<(&'a str, u32)>>,
) -> Option<(&'a str, u32)> {
    let the_box = hashfn(label);
    let me = hashmap.get(&the_box);
    match me {
        Some(v) => {
            for item in v {
                if item.0 == label {
                    return Some(*item);
                }
            }
            None
        }
        None => None,
    }
}

#[allow(dead_code)]
fn print_map(hm: &HashMap<u32, Vec<(&str, u32)>>) {
    for bx in 0_u32..256 {
        let me = hm.get(&bx).unwrap();
        if !me.is_empty() {
            print!("Box {}: ", bx);
            for (id, lens) in me {
                print!("[{} {}] ", id, lens);
            }
            println!();
        }
    }
}

impl DaySolution for Day15 {
    fn part_one(&self) -> String {
        let mut sum = 0;
        for item in self.input.split(',') {
            let x = hashfn(item);
            // println!("{:5} {} {}", item, x, y);
            sum += x;
        }
        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0_usize;
        let mut hashmap = HashMap::<u32, Vec<(&str, u32)>>::new();

        for i in 0..256 {
            hashmap.insert(i, Vec::new());
        }

        for item in self.input.split(',') {
            let lbl = item.split('=').next().unwrap().split('-').next().unwrap();
            let the_box = hashfn(lbl);

            if item.contains('-') {
                match find(lbl, &hashmap) {
                    Some(_v) => {
                        let me = hashmap.get_mut(&the_box).unwrap();
                        let idx = me.iter().position(|x| x.0 == lbl).unwrap();
                        me.remove(idx);
                    }
                    None => {}
                }
            } else if item.contains('=') {
                let new_lens = str::parse::<u32>(item.split('=').nth(1).unwrap()).unwrap();
                match find(lbl, &hashmap) {
                    Some(_v) => {
                        let me = hashmap.get_mut(&the_box).unwrap();
                        let idx = me.iter().position(|x| x.0 == lbl).unwrap();
                        me.remove(idx);
                        me.insert(idx, (lbl, new_lens));
                    }
                    None => {
                        let me = hashmap.get_mut(&the_box).unwrap();
                        (*me).push((lbl, new_lens));
                    }
                }
            } else {
                panic!("Neither - nor =: {}", item);
            }
            // print_map(&hashmap);
            // println!("");
        }

        for i in 0_usize..256 {
            let me = hashmap.get(&(i as u32)).unwrap();
            if !me.is_empty() {
                for (idx, (_, lens)) in me.iter().enumerate() {
                    sum += (i + 1) * (idx + 1) * (*lens as usize);
                }
            }
        }

        sum.to_string()
    }
}
