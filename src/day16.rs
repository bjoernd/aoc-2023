use std::collections::HashSet;

use crate::{DaySolution, FromInput};

struct Beam {
    // Beam position
    line: usize,
    column: usize,
    // Beam direction
    vec_l: i32,
    vec_c: i32,
}

// TODO: Model the problem into this struct
pub struct Day16 {
    map: Vec<Vec<char>>,
}

impl Day16 {
    fn in_bounds(&self, beam: &Beam) -> bool {
        self.map[beam.line][beam.column] != 'O'
    }

    #[allow(dead_code)]
    fn print(&self) {
        for l in &self.map {
            for c in l {
                print!("{} ", c);
            }
            println!();
        }
    }
}

impl FromInput for Day16 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut map: Vec<Vec<char>> = vec![];
        for l in _lines {
            // [
            //     r".|...\....",
            //     r"|.-.\.....",
            //     r".....|-...",
            //     r"........|.",
            //     r"..........",
            //     r".........\",
            //     r"..../.\\..",
            //     r".-.-/..|..",
            //     r".|....-|.\",
            //     r"..//.|....",
            // ].iter() {
            let mut map_line = vec![];
            map_line.push('O');
            for c in l.chars() {
                map_line.push(c);
            }
            map_line.push('O');

            map.push(map_line);
        }

        let line_len = map[0].len();
        let mut pad_line = vec![];
        for _ in 0..line_len {
            pad_line.push('O');
        }

        let mut final_map = vec![];
        final_map.push(pad_line.clone());
        final_map.append(&mut map);
        final_map.push(pad_line.clone());

        Day16 { map: final_map }
    }
}

impl Day16 {
    fn solve_one(&self, init_beam: Beam) -> usize {
        let mut sum = 0;
        let mut beams_todo = vec![];
        beams_todo.push(init_beam);

        let col_len = self.map.len();
        let line_len = self.map[0].len();
        // Use a tracking array to find locations we already visited
        let mut visited_map = vec![vec![false; line_len]; col_len];
        let mut seen_paths = HashSet::<String>::new();

        while let Some(mut beam) = beams_todo.pop() {
            
            // println!("Tracking BEAM @ ({},{}) direction ({:2},{:2}): '{}'", beam.line, beam.column, beam.vec_l, beam.vec_c,
            //         self.map[beam.line][beam.column]);
            visited_map[beam.line][beam.column] = true;
            /* Follow the single beam until it goes out of bounds.

               - Move around the map where necessary.
               - Add new TODO beams whenever we hit a split that we haven't seen before
               - We're done when we reach the out-of-bound situation.
            */
            while self.in_bounds(&beam) {
                let seen_key = format!(
                    "{}-{}-{}-{}",
                    beam.line, beam.column, beam.vec_l, beam.vec_c
                );
                if seen_paths.contains(&seen_key) {
                    // We've already seen this beam path, so we can stop here
                    break;
                }
                seen_paths.insert(seen_key);

                match self.map[beam.line][beam.column] {
                    '.' => {
                        // just move on, no adjustment
                    }

                    '|' => {
                        // adjust if we come from left/right. We stop the current beam and add two new ones
                        // to the todo list. Only do this when we hit this split for the FIRST time.
                        if beam.vec_c != 0 {
                            beams_todo.push(Beam {
                                line: beam.line - 1,
                                column: beam.column,
                                vec_l: -1,
                                vec_c: 0,
                            });
                            beams_todo.push(Beam {
                                line: beam.line + 1,
                                column: beam.column,
                                vec_l: 1,
                                vec_c: 0,
                            });
                            // self.print(); print_visited(&visited_map);
                            break; // out of bounds / done
                        }
                        // no adjustment otherwise, just move on
                    }

                    '-' => {
                        // same as the other split, just from above
                        if beam.vec_l != 0 {
                            beams_todo.push(Beam {
                                line: beam.line,
                                column: beam.column + 1,
                                vec_l: 0,
                                vec_c: 1,
                            });
                            beams_todo.push(Beam {
                                line: beam.line,
                                column: beam.column - 1,
                                vec_l: 0,
                                vec_c: -1,
                            });
                            // self.print(); print_visited(&visited_map);
                            break;
                        }
                    }

                    '/' => {
                        if beam.vec_l != 0 {
                            // moving up/down
                            beam.vec_c = -beam.vec_l;
                            beam.vec_l = 0;
                        } else if beam.vec_c != 0 {
                            // moving left/right
                            beam.vec_l = -beam.vec_c;
                            beam.vec_c = 0;
                        }
                        // self.print(); print_visited(&visited_map);
                    }

                    '\\' => {
                        if beam.vec_l != 0 {
                            // moving up/down
                            beam.vec_c = beam.vec_l;
                            beam.vec_l = 0;
                        } else if beam.vec_c != 0 {
                            // moving left/right
                            beam.vec_l = beam.vec_c;
                            beam.vec_c = 0;
                        }
                        // self.print(); print_visited(&visited_map);
                    }

                    _ => {
                        panic!("Unexpected map symbol!");
                    }
                }

                beam.column = (beam.column as i32 + beam.vec_c) as usize;
                beam.line = (beam.line as i32 + beam.vec_l) as usize;
                visited_map[beam.line][beam.column] = true;
                // println!("   --> BEAM @ ({},{}) direction ({:2},{:2}): '{}'", beam.line, beam.column, beam.vec_l, beam.vec_c,
                //         self.map[beam.line][beam.column]);
            }
        }

        for l in 1..visited_map.len() - 1 {
            for c in 1..visited_map[0].len() - 1 {
                if visited_map[l][c] {
                    sum += 1;
                }
            }
        }

        sum
    }
}

impl DaySolution for Day16 {
    fn part_one(&self) -> String {
        self.solve_one(Beam {
            line: 1,
            column: 1,
            vec_l: 0,
            vec_c: 1,
        })
        .to_string()
    }

    fn part_two(&self) -> String {
        let mut maximum = 0_usize;
        for c in 1..self.map[0].len() - 2 {
            maximum = usize::max(
                maximum,
                self.solve_one(Beam {
                    line: 1,
                    column: c,
                    vec_l: 1,
                    vec_c: 0,
                }),
            );
            // println!("{} {} {}", 1, c, self.solve_one(Beam{line: 1, column: c, vec_l: 1, vec_c: 0}));

            maximum = usize::max(
                maximum,
                self.solve_one(Beam {
                    line: self.map.len() - 2,
                    column: c,
                    vec_l: -1,
                    vec_c: 0,
                }),
            );
            // println!("{} {} {}", self.map.len()-2, c, self.solve_one(Beam{line: self.map.len()-2, column: c, vec_l: -1, vec_c: 0}));
        }

        for l in 1..self.map.len() - 1 {
            maximum = usize::max(
                maximum,
                self.solve_one(Beam {
                    line: l,
                    column: 1,
                    vec_l: 0,
                    vec_c: 1,
                }),
            );
            // println!("{} {} {}", l, 1, self.solve_one(Beam{line: l, column: 1, vec_l: 0, vec_c: 1}));

            maximum = usize::max(
                maximum,
                self.solve_one(Beam {
                    line: l,
                    column: self.map[0].len() - 2,
                    vec_l: 0,
                    vec_c: -1,
                }),
            );
            // println!("{} {} {}", l, self.map[0].len()-2, self.solve_one(Beam{line: l, column: self.map[0].len()-2, vec_l: 0, vec_c: -1}));
        }

        maximum.to_string()
    }
}
