use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day14 {
    map: Vec<Vec<char>>,
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    let max_weight = map.len();
    for l in 0..map.len() {
        print!("{:3} | ", max_weight - l);
        for c in &map[l] {
            print!("{} ", c);
        }
        println!();
    }
}

fn tilt_map_north(map: &mut Vec<Vec<char>>) {
    for l in 1..map.len() {
        for c in 0..map[l].len() {
            if map[l][c] == 'O' {
                let mut new_line = l - 1;
                loop {
                    if map[new_line][c] == '.' {
                        map[new_line][c] = 'O';
                        map[new_line + 1][c] = '.';
                        // println!("------>");
                        // print_map(map);
                    } else {
                        break;
                    }

                    if new_line == 0 {
                        break;
                    }

                    new_line -= 1;
                }
            }
        }
    }
}

fn tilt_map_south(map: &mut Vec<Vec<char>>) {
    for l in (0..map.len() - 1).rev() {
        for c in 0..map[l].len() {
            if map[l][c] == 'O' {
                let mut new_line = l + 1;
                loop {
                    if map[new_line][c] == '.' {
                        map[new_line][c] = 'O';
                        map[new_line - 1][c] = '.';
                        // println!("------>");
                        // print_map(map);
                    } else {
                        break;
                    }

                    if new_line == map.len() - 1 {
                        break;
                    }

                    new_line += 1;
                }
            }
        }
    }
}

fn tilt_map_west(map: &mut Vec<Vec<char>>) {
    for l in 0..map.len() {
        for c in 1..map[l].len() {
            if map[l][c] == 'O' {
                let mut new_col = c - 1;
                loop {
                    if map[l][new_col] == '.' {
                        map[l][new_col] = 'O';
                        map[l][new_col + 1] = '.';
                    } else {
                        break;
                    }
                    if new_col == 0 {
                        break;
                    }

                    new_col -= 1;
                }
            }
        }
    }
}

fn tilt_map_east(map: &mut Vec<Vec<char>>) {
    for l in 0..map.len() {
        for c in (0..map[l].len() - 1).rev() {
            if map[l][c] == 'O' {
                let mut new_col = c + 1;
                loop {
                    if map[l][new_col] == '.' {
                        map[l][new_col] = 'O';
                        map[l][new_col - 1] = '.';
                    } else {
                        break;
                    }
                    if new_col == map[0].len() - 1 {
                        break;
                    }

                    new_col += 1;
                }
            }
        }
    }
}

fn cycle_map(map: &mut Vec<Vec<char>>) {
    tilt_map_north(map);
    // println!("After north tilt:"); print_map(map);
    tilt_map_west(map);
    // println!("After west tilt:"); print_map(map);
    tilt_map_south(map);
    // println!("After south tilt:"); print_map(map);
    tilt_map_east(map);
    // println!("After east tilt:"); print_map(map);
}

fn compute_north_weight(map: &Vec<Vec<char>>) -> usize {
    let max_weight = map.len();
    let mut res = 0;
    for l in 0..map.len() {
        for c in &map[l] {
            if *c == 'O' {
                res += max_weight - l;
            }
        }
    }
    res
}

impl FromInput for Day14 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut d = Day14 { map: vec![] };
        for l in _lines {
            // [
            //     "O....#....",
            //     "O.OO#....#",
            //     ".....##...",
            //     "OO.#O....O",
            //     ".O.....O#.",
            //     "O.#..O.#.#",
            //     "..O..#O..O",
            //     ".......O..",
            //     "#....###..",
            //     "#OO..#....",
            // ]
            // .iter() {
            d.map.push(l.chars().collect());
        }

        d
    }
}

impl DaySolution for Day14 {
    fn part_one(&self) -> String {
        // print_map(&self.map);

        let mut new_map = self.map.clone();
        tilt_map_north(&mut new_map);
        println!();

        // print_map(&new_map);

        compute_north_weight(&new_map).to_string()
    }

    fn part_two(&self) -> String {
        //print_map(&self.map);

        let mut new_map = self.map.clone();
        let mut results = vec![];

        let mut idx = 0;
        loop {
            cycle_map(&mut new_map);
            // print_map(&new_map);
            if !results.is_empty() && results.contains(&new_map) {
                break;
            }
            results.push(new_map.clone());
            idx += 1;
        }

        // idx is the first position of the NEXT cycle
        // i2 is the first position of the FIRST cycle
        let i2 = results.iter().position(|map| **map == new_map).unwrap();

        println!("Cycle length:    {}", idx - i2);
        println!("Cycle start pos: {}", i2);

        // for x in i2 .. idx {
        //     println!("{} {}", x, compute_north_weight(&results[x]));
        // }

        // Of course, one could now do a clever modulo computation. But i'm too stupid for this
        // right now, so let's just hard-count until 1 billion cycles...
        let mut c = 1_usize;
        let mut mod_c = 0_usize;
        while c < 1_000_000_000 {
            c += 1;
            mod_c += 1;
            if mod_c >= idx {
                mod_c = i2;
            }
        }

        compute_north_weight(&results[mod_c]).to_string()
    }
}
