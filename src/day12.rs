use std::usize;

use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct

struct GroupDescription {
    items: Vec<char>,
    numeric : Vec<usize>,
}

pub struct Day12 {
    groups: Vec<GroupDescription>,
}

impl FromInput for Day12 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut groups = vec![];

        for l in lines
            // [
            //     "???.### 1,1,3",
            //     ".??..??...?##. 1,1,3",
            //     "?#?#?#?#?#?#?#? 1,3,1,6",
            //     "????.#...#... 4,1,1",
            //     "????.######..#####. 1,6,5",
            //     "?###???????? 3,2,1"
            // ]
        {
            let mut items = vec![];
            let mut numbers = vec![];

            for (i, item) in l.split_whitespace().enumerate() {
                match i {
                    0 => {
                        items = item.chars().collect();
                    },
                    1 => {
                        numbers = item.split(',').map(|x| str::parse(x).unwrap()).collect();
                    },
                    _ => panic!("Too many items to unpack.")
                }
            }
            groups.push(GroupDescription {
                items,
                numeric: numbers,
            });
        }

        Day12 { groups }
    }
}

fn is_valid(items: &[char], numbers: &[usize]) -> bool {
    let s: String = items.iter().collect();
    let counts: Vec<usize> = s.replace('.', " ").split_whitespace().map(|x| x.len()).collect();

    //println!("{:?} == {:?} {}", counts, numbers, &counts==numbers);

    counts == numbers
}

fn solve_one(items: &[char], numeric: &[usize]) -> usize {
    //println!("{:?} | {:?}", items, numeric);
    if !items.contains(&'?') {
        if is_valid(items, numeric) { return 1; }
        else { return 0; }
    }

    let next_quest = items.iter().position(|x| *x == '?').unwrap();
    solve_one(&[ &items[0..next_quest], vec!['.'].as_slice(), &items[next_quest+1..] ].concat(), numeric)
     + solve_one(&[ &items[0..next_quest], vec!['#'].as_slice(), &items[next_quest+1..] ].concat(), numeric)
}

impl DaySolution for Day12 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;
        for (i, g) in self.groups.iter().enumerate() {
            sum += solve_one(&g.items, &g.numeric);
            if i % 50 == 0 {
                println!("{}/{}", i, self.groups.len());
            }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        todo!("Solve part two of day 12 using your parsed input")
    }
}
