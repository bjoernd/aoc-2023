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
    map: Vec<Vec<char>>
}

impl Day16 {
    fn in_bounds(&self, beam: &Beam) -> bool {
        self.map[beam.line][beam.column] != 'O'
    }

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

impl DaySolution for Day16 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;

        let mut beams_todo = vec![];
        beams_todo.push( Beam{line:1, column: 1, vec_l:0, vec_c:1 } );

        let col_len = self.map.len();
        let line_len = self.map[0].len();
        // Use a tracking array to find locations we already visited
        let mut visited_map = vec![vec![0;line_len];col_len];

        while !beams_todo.is_empty() {
            let mut beam: Beam = beams_todo.pop().unwrap();
            println!("Tracking BEAM @ ({},{}) direction ({:2},{:2}): '{}'", beam.line, beam.column, beam.vec_l, beam.vec_c,
                    self.map[beam.line][beam.column]);
            visited_map[beam.line][beam.column] += 1;
            /* Follow the single beam until it goes out of bounds.
            
                - Move around the map where necessary.
                - Add new TODO beams whenever we hit a split that we haven't seen before
                - We're done when we reach the out-of-bound situation.
             */
            while self.in_bounds(&beam) {
                match self.map[beam.line][beam.column] {
                    '.' => {
                        // just move on, no adjustment
                    },

                    '|' => {
                        // adjust if we come from left/right. We stop the current beam and add two new ones
                        // to the todo list. Only do this when we hit this split for the FIRST time.
                        if beam.vec_c != 0 && visited_map[beam.line][beam.column] == 1 {
                            beams_todo.push(Beam{ line: beam.line - 1, column: beam.column, vec_l: -1, vec_c: 0});
                            beams_todo.push(Beam{ line: beam.line + 1, column: beam.column, vec_l: 1, vec_c: 0});
                            break; // out of bounds / done
                        }
                        // no adjustment otherwise, just move on
                    },

                    '-' => {
                        // same as the other split, just from above
                        if beam.vec_l != 0 && visited_map[beam.line][beam.column] == 1 {
                            beams_todo.push(Beam{ line: beam.line, column: beam.column+1, vec_l: 0, vec_c: 1});
                            beams_todo.push(Beam{ line: beam.line, column: beam.column-1, vec_l: 0, vec_c: -1});
                            break;
                        }
                    },

                    '/' => {
                        if beam.vec_l != 0 { // moving up/down
                            beam.vec_c = -beam.vec_l;
                            beam.vec_l = 0;
                        } else if beam.vec_c != 0 { // moving left/right
                            beam.vec_l = -beam.vec_c;
                            beam.vec_c = 0;
                        }
                    },

                    '\\' => {
                        if beam.vec_l != 0 { // moving up/down
                            beam.vec_c = beam.vec_l;
                            beam.vec_l = 0;
                        } else if beam.vec_c != 0 { // moving left/right
                            beam.vec_l = beam.vec_c;
                            beam.vec_c = 0;
                        }
                    },

                    _ => { panic!("Unexpected map symbol!"); }
                }

                beam.column = (beam.column as i32 + beam.vec_c) as usize;
                beam.line = (beam.line as i32 + beam.vec_l) as usize;
                visited_map[beam.line][beam.column] += 1;
                // println!("   --> BEAM @ ({},{}) direction ({:2},{:2}): '{}'", beam.line, beam.column, beam.vec_l, beam.vec_c,
                //         self.map[beam.line][beam.column]);
            }
        }

        let max_l = self.map.len();
        let max_c = self.map[0].len();

        for l in 0..max_l {
            visited_map[l][0] = 0;
            visited_map[l][max_c - 1] = 0;
        }

        for c in 0..max_c {
            visited_map[0][c] = 0;
            visited_map[max_l - 1][c] = 0;
        }

        self.print();

        for l in 1..visited_map.len()-1 {
            print!("  ");
            for c in 1..visited_map[0].len()-1 {
                // print!("{} ", visited_map[l][c]);
                if visited_map[l][c] > 0 {
                    sum += 1;
                    print!("# ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
        sum.to_string()
    }

    fn part_two(&self) -> String {
        let sum = 0_usize;
        todo!("Solve part two of day 16 using your parsed input");
        sum.to_string()
    }
}
