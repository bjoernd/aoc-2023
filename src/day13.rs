

use crate::{DaySolution, FromInput};

struct Map {
    map: Vec<Vec<char>>,
    map_rotated: Vec<Vec<char>>,
}

// TODO: Model the problem into this struct
pub struct Day13 {
    maps: Vec<Map>,
}

#[derive(PartialEq, Debug)]
enum ReflectionDirection {
    Horizontal,
    Vertical,
}

fn distance(one: &Vec<char>, two: &Vec<char>) -> usize {
    let mut res = 0_usize;
    for (idx, _c) in one.iter().enumerate() {
        if one[idx] != two[idx] {
            res += 1;
        }
    }
    res
}

fn find_horizontal_reflection(map: &Vec<Vec<char>>, imperfections: usize) -> Option<usize> {
    // println!("find_horizontal_reflection {}..{} imperf {}", 1, map.len()-1, imperfections);
    let mut imperf_seen = 0;
    for l in 1..map.len() {
        let mut idx_up = l as i32 - 1;
        let mut idx_down = l as i32;
        loop {
            // println!("   {} {:?} {} {:?}", idx_up, map[idx_up as usize], idx_down, map[idx_down as usize]);
            let dist = distance(&map[idx_down as usize], &map[idx_up as usize]);
            //if imperf_seen + dist >= imperfections { break; }
            imperf_seen += dist;

            idx_up -= 1;
            idx_down += 1;

            // println!("    -> {} {} {}", idx_up, idx_down, imperf_seen);
            if idx_up < 0 || idx_down > map.len() as i32 - 1 {
                break;
            }
        }
        // println!("idx: {} up {} down {} imperf {}", l, idx_up, idx_down, imperf_seen);
        if (idx_up < 0 || idx_down > map.len() as i32 - 1) && imperf_seen == imperfections {
            return Some(l);
        }
        imperf_seen = 0;
    }
    None
}

impl Map {
    #[allow(dead_code)]
    fn print(&self) {
        println!("===== MAP");
        for (line, l) in self.map.iter().enumerate() {
            print!("{:2} ", line);
            for c in l {
                print!("{} ", c);
            }
            println!();
        }
        println!("===== MAP_ROT");
        for (line, l) in self.map_rotated.iter().enumerate() {
            print!("{:2} ", line);
            for c in l {
                print!("{} ", c);
            }
            println!();
        }
    }
}

impl FromInput for Day13 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut map: Vec<Vec<char>> = vec![];
        let mut d = Day13 { maps: vec![] };

        for (_lineno, l) in _lines.enumerate() {
            // [
            //     "#.##..##.",
            //     "..#.##.#.",
            //     "##......#",
            //     "##......#",
            //     "..#.##.#.",
            //     "..##..##.",
            //     "#.#.##.#.",
            //     "",
            //     "#...##..#",
            //     "#....#..#",
            //     "..##..###",
            //     "#####.##.",
            //     "#####.##.",
            //     "..##..###",
            //     "#....#..#",
            //     "",
            // "##..#.#..##",
            // "###...#....",
            // "##.##.##...",
            // "....###..##",
            // "####..###..",
            // "##.####....",
            // "...####.###",
            // "###...##...",
            // "##...#..###",
            // "##...#.....",
            // ".##..##.###",
            // "",
            // ].iter().enumerate() {
            if l.trim() != "" {
                map.push(l.chars().collect());
            } else {
                let mut rot_map = vec![];

                for c in 0..map[0].len() {
                    let chars: Vec<char> = map.iter().map(|x| *x.iter().nth(c).unwrap()).collect();
                    rot_map.push(chars);
                }

                d.maps.push(Map {
                    map: map.clone(),
                    map_rotated: rot_map,
                });
                map.clear();
            }
        }

        d
    }
}

impl DaySolution for Day13 {
    fn part_one(&self) -> String {
        let mut sum = 0;

        for m in &self.maps {
            //m.print();

            if let Some(x) = find_horizontal_reflection(&m.map, 0) {
                // println!("Horizontal reflection at {}", x);
                sum += 100 * x;
            } else if let Some(x) = find_horizontal_reflection(&m.map_rotated, 0) {
                // println!("Vertical reflection at {}", x);
                sum += x;
            } else {
                m.print();
                panic!("No reflection!");
            }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0;

        for (idx, m) in self.maps.iter().enumerate() {
            //m.print();

            let old_reflection: (ReflectionDirection, usize);

            if let Some(x) = find_horizontal_reflection(&m.map, 0) {
                // println!("Horizontal reflection at {}", x);
                old_reflection = (ReflectionDirection::Horizontal, x);
            } else if let Some(x) = find_horizontal_reflection(&m.map_rotated, 0) {
                // println!("Vertical reflection at {}", x);
                old_reflection = (ReflectionDirection::Vertical, x);
            } else {
                panic!("No reflection!");
            }

            if let Some(x) = find_horizontal_reflection(&m.map, 1) {
                // println!("Horizontal reflection at {}", x);
                if old_reflection != (ReflectionDirection::Horizontal, x) {
                    sum += 100 * x;
                }
            } else if let Some(x) = find_horizontal_reflection(&m.map_rotated, 1) {
                // println!("Vertical reflection at {}", x);
                if old_reflection != (ReflectionDirection::Vertical, x) {
                    sum += x;
                }
            } else {
                println!("idx {}", idx);
                m.print();
                panic!("No reflection!");
            }
        }

        sum.to_string()
    }
}
