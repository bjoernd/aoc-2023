use crate::{DaySolution, FromInput};
use cached::proc_macro::cached;
use itertools::{Itertools, PeekingNext};

// TODO: Model the problem into this struct

struct GroupDescription {
    items: Vec<char>,
    numeric : Vec<usize>,
}

pub struct Day12 {
    groups: Vec<GroupDescription>,
}

impl FromInput for Day12 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut groups = vec![];

        for l in &_lines.collect_vec()
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
                        let mut lastc = 'X';
                        for ch in item.chars().collect::<Vec<char>>() {
                            if ch == '.' && lastc != '.' {
                                items.push(ch);
                            } else if ch != '.' {
                                items.push(ch);
                            }
                            lastc = ch;
                        }
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

fn compute_schema(items: &Vec<char>) -> Vec<usize>
{
    items.into_iter().collect::<String>().replace(".", " ").split_whitespace().map(|x| x.len()).collect::<Vec<usize>>()
}

fn validate(items: &Vec<char>, schema: &Vec<usize>) -> bool {
    if items.contains(&'?') { return false; }
    compute_schema(items) == *schema
}


#[cached]
fn solve_one(items: Vec<char>, schema: Vec<usize>, depth: usize) -> usize {
    // for i in 0..depth*3 { print!(" "); }
    // println!("==== {:?} {:?}", items, schema);
    
    if !items.contains(&'?') {
        if validate(&items, &schema) {
            // for i in 0..depth*3 { print!(" "); }
            // println!("{:?} {:?} 1", items, schema);
            return 1;
        }
        else {
            // for i in 0..depth*3 { print!(" "); }
            // println!("{:?} {:?} 1", items, schema);
            return 0;
        }
    }

    /* Empty schema - we must not have any hashes left */
    if schema.len() == 0 {
        if items.contains(&'#') { return 0; }
        return 1;
    }

    if items.len() == 0 { return 0; }

    let mut idx  = 0_usize;

    match items[idx] {
        '.' => {
            return solve_one(items[1..].to_vec(), schema, depth+1);
        },
        '#' => {
            let mut needed_hashes = schema[0];

            // walk the items list. consume # and ? as hashes until we
            // have found our next group
            while idx < items.len() && needed_hashes > 0 {
                if items[idx] == '#' || items[idx] == '?' {
                    idx += 1;
                    needed_hashes -= 1;
                } else if items[idx] == '.' {
                    break;
                }
            }

            // for i in 0..depth*3 { print!(" "); }
            // println!("idx {} needed_hashes {} len {}", idx, needed_hashes, items.len());

            if needed_hashes > 0 { return 0; }

            if idx == items.len() {
                if schema.len() == 1 { return 1; } // we matched the last group
                return 0;
            }

            // We found a group! Now if the next char is still a hash, we mismatched. If it's a . or a ?, treat it as a .
            if items[idx] == '#' { return 0; }

            let new_schema = schema[1..].to_vec();
            return solve_one(items[idx+1..].to_vec(), new_schema, depth+1);
        },
        '?' => {
            let new_items = [vec!['#'], items[1..].to_vec()].concat();
            return solve_one(new_items, schema.clone(), depth+1) + solve_one(items[1..].to_vec(), schema, depth+1);
        }
        _ => { panic!("Invalid item"); }
    }
}

impl DaySolution for Day12 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;
        for (_i, g) in self.groups.iter().enumerate() {
            // println!("------------------------------------------------------------");
            let res = solve_one(g.items.clone(), g.numeric.clone(), 0);
            // println!("{:?} {:?} {}", g.items, g.numeric, res);
            sum += res;
            // if _i % 50 == 49 {
            //     println!("{}/{}", _i+1, self.groups.len());
            // }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0_usize;
        for (_i, g) in self.groups.iter().enumerate() {
            let mut expanded_items = g.items.clone();
            expanded_items.push('?');
            expanded_items.append(&mut g.items.clone());
            expanded_items.push('?');
            expanded_items.append(&mut g.items.clone());
            expanded_items.push('?');
            expanded_items.append(&mut g.items.clone());
            expanded_items.push('?');
            expanded_items.append(&mut g.items.clone());

            let expanded_schema = g.numeric.repeat(5);

            let res = solve_one(expanded_items, expanded_schema, 0);
            // println!("{:?} {:?} {}", g.items, g.numeric, res);
            sum += res;
            if _i % 50 == 49 {
                println!("{}/{}", _i+1, self.groups.len());
            }
        }
        sum.to_string()
    }
}
