use itertools::Itertools;

use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day10 {
    map: Vec<Vec<char>>,
}

impl FromInput for Day10 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut v: Vec<Vec<char>> = vec![];

        /*
        let lines2 = vec![
            /*
            "FF7FSF7F7F7F7F7F---7",
            "L|LJ||||||||||||F--J",
            "FL-7LJLJ||||||LJL-77",
            "F--JF--7||LJLJ7F7FJ-",
            "L---JF-JLJ.||-FJLJJ7",
            "|F|F-JF---7F7-L7L|7|",
            "|FFJF7L7F-JF7|JL---7",
            "7-L-JL7||F7|L7F-7F7|",
            "L.L7LFJ|||||FJL7||LJ",
            "L7JLJL-JLJLJL--JLJ.L",

            "..........",
            ".S------7.",
            ".|F----7|.",
            ".||OOOO||.",
            ".||OOOO||.",
            ".|L-7F-J|.",
            ".|II||II|.",
            ".L--JL--J.",
            "..........",
            */
            ".F----7F7F7F7F-7....",
            ".|F--7||||||||FJ....",
            ".||.FJ||||||||L7....",
            "FJL7L7LJLJ||LJ.L-7..",
            "L--J.L7...LJS7F-7L7.",
            "....F-J..F7FJ|L7L7L7",
            "....L7.F7||L7|.L7L7|",
            ".....|FJLJ|FJ|F7|.LJ",
            "....FJL-7.||.||||...",
            "....L---J.LJ.LJLJ...",
        ];
        */

        // Read data and pad it with a boundary of '.' chars so that later we never
        // need to worry about going out of bounds
        for (i, l) in lines.enumerate() {
            let mut lv = vec![];

            if i == 0 {
                let mut extra_line = vec![];
                for _ in 0..l.len() + 2 {
                    extra_line.push('.');
                }
                v.push(extra_line);
            }

            lv.push('.');

            for c in l.chars() {
                lv.push(c);
            }

            lv.push('.');

            v.push(lv);
        }

        let mut extra_line = vec![];
        for _ in 0..v[0].len() {
            extra_line.push('.');
        }
        v.push(extra_line);

        Day10 { map: v }
    }
}

fn build_dist_matrix(lines: usize, cols: usize) -> Vec<Vec<usize>> {
    let mut matrix = vec![];
    for _ in 0..lines {
        matrix.push(vec![0_usize; cols]);
    }
    matrix
}

fn find_start_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
    let start_line = map
        .iter()
        .map(|x| x.iter().contains(&'S'))
        .position(|z| z)
        .unwrap();
    let start_col = map[start_line].iter().position(|c| *c == 'S').unwrap();

    (start_line, start_col)
}

#[allow(dead_code)]
fn print_surroundings<T: std::fmt::Display>(map: &Vec<Vec<T>>, line: usize, col: usize) {
    let lstart = line - 1;
    let cstart = col - 1;
    for l in lstart..lstart + 3 {
        for c in cstart..cstart + 3 {
            print!("{} ", map[l][c]);
        }
        println!();
    }
}

fn find_connecting(map: &Vec<Vec<char>>, line: usize, col: usize) -> Vec<(usize, usize)> {
    let mut connections = vec![];

    match map[line][col] {
        '|' => {
            connections.push((line - 1, col));
            connections.push((line + 1, col));
        }
        '-' => {
            connections.push((line, col - 1));
            connections.push((line, col + 1));
        }
        'L' => {
            connections.push((line - 1, col));
            connections.push((line, col + 1));
        }
        'J' => {
            connections.push((line, col - 1));
            connections.push((line - 1, col));
        }
        '7' => {
            connections.push((line, col - 1));
            connections.push((line + 1, col));
        }
        'F' => {
            connections.push((line + 1, col));
            connections.push((line, col + 1));
        }
        'S' => {
            let up = map[line - 1][col];
            let down = map[line + 1][col];
            let left = map[line][col - 1];
            let right = map[line][col + 1];

            if up == '|' || up == '7' || up == 'F' {
                connections.push((line - 1, col));
            }
            if down == '|' || down == 'L' || down == 'J' {
                connections.push((line + 1, col));
            }
            if left == '-' || left == 'L' || left == 'F' {
                connections.push((line, col - 1));
            }
            if right == '-' || right == '7' || right == 'J' {
                connections.push((line, col + 1));
            }
        }
        _ => todo!("!"),
    }

    assert!(connections.len() == 2);

    connections
}

fn replace_s_tile(map: &Vec<Vec<char>>, line: usize, col: usize) -> char {
    let connectors = find_connecting(map, line, col);

    let up = (line - 1, col);
    let down = (line + 1, col);
    let left = (line, col - 1);
    let right = (line, col + 1);

    if connectors.contains(&up) && connectors.contains(&down) {
        return '|';
    }
    if connectors.contains(&up) && connectors.contains(&right) {
        return 'L';
    }
    if connectors.contains(&up) && connectors.contains(&left) {
        return 'J';
    }
    if connectors.contains(&down) && connectors.contains(&right) {
        return 'F';
    }
    if connectors.contains(&down) && connectors.contains(&left) {
        return '7';
    }
    if connectors.contains(&right) && connectors.contains(&left) {
        return '-';
    }

    assert!(false, "Should never get here.");

    '?'
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn is_dot(c: char) -> bool {
    c == '.'
}

fn access(map: &Vec<Vec<char>>, line: i32, col: i32) -> char {
    map[line as usize][col as usize]
}

fn turn_o(map: &mut Vec<Vec<char>>, line: i32, col: i32) {
    if access(map, line, col) == '.' {
        map[line as usize][col as usize] = 'O';
    }
}

fn turn_i(map: &mut Vec<Vec<char>>, line: i32, col: i32) {
    if access(map, line, col) == '.' {
        map[line as usize][col as usize] = 'I';
    }
}

fn count_i(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .map(|x| x.iter().filter(|x| *x == &'I').count())
        .sum()
}

fn count_o(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .map(|x| x.iter().filter(|x| *x == &'O').count())
        .sum()
}

fn find_outer_bound(map: &Vec<Vec<char>>) -> (i32, i32) {
    let mut rand_line: usize = rand::random::<usize>() % map.len();
    let mut col = 0;
    loop {
        loop {
            if col >= map.len() {
                break;
            }

            if !is_dot(map[rand_line][col]) {
                // found the pipe!
                break;
            }
            col += 1;
        }

        if col < map.len() {
            break;
        }

        rand_line = rand::random::<usize>() % map.len();
        col = 0;
    }

    (rand_line as i32, col as i32)
}

impl DaySolution for Day10 {
    fn part_one(&self) -> String {
        //println!("lines: {}", self.map.len());
        //println!("columns: {}", self.map[0].len());

        let mut dist_matrix = build_dist_matrix(self.map.len(), self.map[0].len());
        let (start_line, start_col) = find_start_pos(&self.map);

        //println!("start_line: {}", start_line);
        //println!("start_col: {}", start_col);

        //print_surroundings(&self.map, start_line, start_col);

        // positions stores the two pointers we are now following
        let mut positions = find_connecting(&self.map, start_line, start_col);

        let mut visited_positions = vec![];
        visited_positions.push((start_line, start_col));
        let mut distance = 1;

        loop {
            let (l1, c1) = positions.pop().unwrap();
            let (l2, c2) = positions.pop().unwrap();

            dist_matrix[l1][c1] = distance;
            dist_matrix[l2][c2] = distance;

            visited_positions.push((l1, c1));
            visited_positions.push((l2, c2));

            match find_connecting(&self.map, l1, c1)
                .iter()
                .filter(|x| !visited_positions.contains(*x))
                .next()
            {
                Some(x) => positions.push(*x),
                None => {
                    break;
                }
            };
            match find_connecting(&self.map, l2, c2)
                .iter()
                .filter(|x| !visited_positions.contains(*x))
                .next()
            {
                Some(x) => positions.push(*x),
                None => {
                    break;
                }
            };

            distance += 1;
        }

        distance.to_string()
    }

    fn part_two(&self) -> String {
        /* First, we redo part 1. This time we don't need a distance, we just want to know which tiles are part of the loop. */
        let mut dist_matrix = build_dist_matrix(self.map.len(), self.map[0].len());
        let (start_line, start_col) = find_start_pos(&self.map);
        let mut positions = find_connecting(&self.map, start_line, start_col);

        let mut visited_positions = vec![];
        visited_positions.push((start_line, start_col));

        loop {
            let (l1, c1) = positions.pop().unwrap();
            let (l2, c2) = positions.pop().unwrap();

            dist_matrix[l1][c1] = 1;
            dist_matrix[l2][c2] = 1;

            visited_positions.push((l1, c1));
            visited_positions.push((l2, c2));

            match find_connecting(&self.map, l1, c1)
                .iter()
                .filter(|x| !visited_positions.contains(*x))
                .next()
            {
                Some(x) => positions.push(*x),
                None => {
                    break;
                }
            };
            match find_connecting(&self.map, l2, c2)
                .iter()
                .filter(|x| !visited_positions.contains(*x))
                .next()
            {
                Some(x) => positions.push(*x),
                None => {
                    break;
                }
            };
        }

        /* Now we build a new map that only contains the loop */
        let mut new_map = vec![];

        for l in 0..self.map.len() {
            let mut new_line = vec![];
            for c in 0..self.map[0].len() {
                match self.map[l][c] {
                    '.' => {
                        new_line.push('.');
                    }
                    'S' => {
                        new_line.push(replace_s_tile(&self.map, l, c));
                    }
                    _ => {
                        if dist_matrix[l][c] != 0 {
                            new_line.push(self.map[l][c]);
                        } else {
                            new_line.push('.');
                        }
                    }
                }
            }
            new_map.push(new_line);
        }

        print_map(&new_map);

        /* Algorithm:

           1) Pick a random position in the left border and move right until we hit the pipe.
              We now know where 'outside' and 'inside' of the loop are because we come from the
              outside.
           2) Walk the loop so that 'inside' is on our right and 'outside' is on our left. Mark
              all '.' tiles we see as neighbours with either O or I.
           3) Walk the whole map again and turn remaining dots into O or I based on neighbouring
              relationship. This may take a couple of passes but eventually we will be done.
        */

        let (mut lpos, mut cpos) = find_outer_bound(&new_map);
        println!(
            "Pipe hit at {} {} ({})",
            lpos,
            cpos,
            access(&new_map, lpos, cpos)
        );

        // We came from the left. If we now go UP, outside is always left and inside is always right.
        // But we have to set the initial vector based on our position in order to 'go up'.

        let mut lvec = 0_i32;
        let mut cvec = 0_i32;

        match access(&new_map, lpos, cpos) {
            '|' => {
                lvec = -1;
                cvec = 0;
            }
            'L' => {
                lvec = 0;
                cvec = -1;
            }
            'F' => {
                lvec = -1;
                cvec = 0;
            }
            '-' | '7' | 'J' => {
                assert!(false, "Could never hit a -/7/J pipe initially!");
            }
            _ => {
                assert!(false, "Unexpected char: {}", access(&new_map, lpos, cpos));
            }
        }

        let start_l = lpos;
        let start_c = cpos;
        let mut steps = 0;

        loop {
            steps += 1;

            //println!("----- {}", access(&new_map, lpos, cpos));

            // adjust neighbouring dots based on left/right; adjust movement vector
            match access(&new_map, lpos, cpos) {
                '|' => {
                    // if we are moving UP, out is left, in is right
                    if lvec == -1 {
                        turn_o(&mut new_map, lpos - 1, cpos - 1);
                        turn_o(&mut new_map, lpos, cpos - 1);
                        turn_o(&mut new_map, lpos + 1, cpos - 1);

                        turn_i(&mut new_map, lpos - 1, cpos - 1);
                        turn_i(&mut new_map, lpos, cpos);
                        turn_i(&mut new_map, lpos + 1, cpos + 1);
                    } else {
                        assert!(lvec == 1);
                        turn_i(&mut new_map, lpos - 1, cpos - 1);
                        turn_i(&mut new_map, lpos, cpos - 1);
                        turn_i(&mut new_map, lpos + 1, cpos - 1);

                        turn_o(&mut new_map, lpos - 1, cpos - 1);
                        turn_o(&mut new_map, lpos, cpos);
                        turn_o(&mut new_map, lpos + 1, cpos + 1);
                    }
                    // no change to movement vector
                }
                '-' => {
                    // if we were moving LEFT, out is down and in is up
                    if cvec == -1 {
                        turn_i(&mut new_map, lpos - 1, cpos - 1);
                        turn_i(&mut new_map, lpos - 1, cpos);
                        turn_i(&mut new_map, lpos - 1, cpos + 1);

                        turn_o(&mut new_map, lpos + 1, cpos - 1);
                        turn_o(&mut new_map, lpos + 1, cpos);
                        turn_o(&mut new_map, lpos + 1, cpos + 1);
                    } else {
                        assert!(cvec == 1);
                        turn_o(&mut new_map, lpos - 1, cpos - 1);
                        turn_o(&mut new_map, lpos - 1, cpos);
                        turn_o(&mut new_map, lpos - 1, cpos + 1);

                        turn_i(&mut new_map, lpos + 1, cpos - 1);
                        turn_i(&mut new_map, lpos + 1, cpos);
                        turn_i(&mut new_map, lpos + 1, cpos + 1);
                    }
                    // no change to movement vector
                }
                '7' => {
                    // were we moving up? -> in is right/up, out is left/down
                    if lvec == -1 {
                        turn_o(&mut new_map, lpos + 1, cpos - 1);

                        turn_i(&mut new_map, lpos + 1, cpos + 1);
                        turn_i(&mut new_map, lpos, cpos + 1);
                        turn_i(&mut new_map, lpos - 1, cpos + 1);
                        turn_i(&mut new_map, lpos - 1, cpos);
                        turn_i(&mut new_map, lpos - 1, cpos - 1);

                        lvec = 0;
                        cvec = -1;
                    } else {
                        // we were moving right and turn down
                        assert!(cvec == 1);
                        turn_i(&mut new_map, lpos + 1, cpos - 1);

                        turn_o(&mut new_map, lpos + 1, cpos + 1);
                        turn_o(&mut new_map, lpos, cpos + 1);
                        turn_o(&mut new_map, lpos - 1, cpos + 1);
                        turn_o(&mut new_map, lpos - 1, cpos);
                        turn_o(&mut new_map, lpos - 1, cpos - 1);

                        lvec = 1;
                        cvec = 0;
                    }
                }
                'L' => {
                    // were we moving down? -> in is right/down
                    if lvec == 1 {
                        turn_o(&mut new_map, lpos - 1, cpos + 1);

                        turn_i(&mut new_map, lpos - 1, cpos - 1);
                        turn_i(&mut new_map, lpos, cpos - 1);
                        turn_i(&mut new_map, lpos + 1, cpos - 1);
                        turn_i(&mut new_map, lpos + 1, cpos);
                        turn_i(&mut new_map, lpos + 1, cpos + 1);

                        lvec = 0;
                        cvec = 1;
                    } else {
                        // we were moving left and turn up
                        assert!(cvec == -1);
                        turn_i(&mut new_map, lpos - 1, cpos + 1);

                        turn_o(&mut new_map, lpos - 1, cpos - 1);
                        turn_o(&mut new_map, lpos, cpos - 1);
                        turn_o(&mut new_map, lpos + 1, cpos - 1);
                        turn_o(&mut new_map, lpos + 1, cpos);
                        turn_o(&mut new_map, lpos + 1, cpos + 1);

                        lvec = -1;
                        cvec = 0;
                    }
                }
                'F' => {
                    // were we moving up? -> in is right/down
                    if lvec == -1 {
                        turn_i(&mut new_map, lpos + 1, cpos + 1);

                        turn_o(&mut new_map, lpos + 1, cpos - 1);
                        turn_o(&mut new_map, lpos, cpos - 1);
                        turn_o(&mut new_map, lpos - 1, cpos - 1);
                        turn_o(&mut new_map, lpos - 1, cpos);
                        turn_o(&mut new_map, lpos - 1, cpos + 1);

                        lvec = 0;
                        cvec = 1;
                    } else {
                        // we were moving right and turn down
                        assert!(cvec == -1);
                        turn_o(&mut new_map, lpos + 1, cpos + 1);

                        turn_i(&mut new_map, lpos + 1, cpos - 1);
                        turn_i(&mut new_map, lpos, cpos - 1);
                        turn_i(&mut new_map, lpos - 1, cpos - 1);
                        turn_i(&mut new_map, lpos - 1, cpos);
                        turn_i(&mut new_map, lpos - 1, cpos + 1);

                        lvec = 1;
                        cvec = 0;
                    }
                }
                'J' => {
                    // were we moving down?
                    if lvec == 1 {
                        turn_i(&mut new_map, lpos - 1, cpos - 1);

                        turn_o(&mut new_map, lpos - 1, cpos + 1);
                        turn_o(&mut new_map, lpos, cpos + 1);
                        turn_o(&mut new_map, lpos + 1, cpos - 1);
                        turn_o(&mut new_map, lpos + 1, cpos);
                        turn_o(&mut new_map, lpos + 1, cpos + 1);

                        lvec = 0;
                        cvec = -1;
                    } else {
                        // we were moving left and turn up
                        assert!(cvec == 1);
                        turn_o(&mut new_map, lpos - 1, cpos - 1);

                        turn_i(&mut new_map, lpos - 1, cpos + 1);
                        turn_i(&mut new_map, lpos, cpos + 1);
                        turn_i(&mut new_map, lpos + 1, cpos - 1);
                        turn_i(&mut new_map, lpos + 1, cpos);
                        turn_i(&mut new_map, lpos + 1, cpos + 1);

                        lvec = -1;
                        cvec = 0;
                    }
                }
                _ => {
                    println!("Unexpected char: {}", access(&new_map, lpos, cpos));
                }
            }

            //print_map(&new_map); println!();

            // move to next position
            lpos += lvec;
            cpos += cvec;

            // hit start? -> done
            if lpos == start_l && cpos == start_c {
                break;
            }
        }

        println!("Walked loop in {} steps", steps);
        print_map(&new_map);

        let mut old_o = count_o(&new_map);
        let mut old_i = count_i(&new_map);

        // now go and turn all dots neighbouring a I/O into their respective
        // counterparts
        loop {
            for l in 1..new_map.len() - 1 {
                for c in 1..new_map[0].len() - 1 {
                    if !is_dot(new_map[l][c]) {
                        continue;
                    }

                    for x in ['O', 'I'] {
                        if new_map[l - 1][c - 1] == x {
                            new_map[l][c] = x;
                        }
                        if new_map[l - 1][c] == x {
                            new_map[l][c] = x;
                        }
                        if new_map[l - 1][c + 1] == x {
                            new_map[l][c] = x;
                        }
                        if new_map[l][c - 1] == x {
                            new_map[l][c] = x;
                        }
                        if new_map[l][c] == x {
                            new_map[l][c] = x;
                        }
                        if new_map[l][c + 1] == x {
                            new_map[l][c] = x;
                        }
                        if new_map[l + 1][c - 1] == x {
                            new_map[l][c] = x;
                        }
                        if new_map[l + 1][c] == x {
                            new_map[l][c] = x;
                        }
                        if new_map[l + 1][c + 1] == x {
                            new_map[l][c] = x;
                        }
                    }
                }
            }

            let new_o = count_o(&new_map);
            let new_i = count_i(&new_map);

            if old_o == new_o && old_i == new_i {
                break;
            }

            old_o = new_o;
            old_i = new_i;
        }

        count_i(&new_map).to_string()
    }
}
