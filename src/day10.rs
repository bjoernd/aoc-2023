use itertools::Itertools;
use num::traits::Pow;

use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day10 {
    map: Vec<Vec<char>>,
}

impl FromInput for Day10 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut v: Vec<Vec<char>> = vec![];

        // Read data and pad it with a boundary of '.' chars so that later we never
        // need to worry about going out of bounds
        for (i, l) in lines.enumerate() {
            let mut lv = vec![];

            if i == 0 {
                let mut extra_line = vec![];
                for j in 0..l.len()+2 { extra_line.push('.'); }
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
        for j in 0..v[0].len() { extra_line.push('.'); }
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
            .position(|z| z )
            .unwrap();
        let start_col = map[start_line].iter().position(|c| *c == 'S').unwrap();

    (start_line, start_col)
}

#[allow(dead_code)]
fn print_surroundings<T : std::fmt::Display>(map: &Vec<Vec<T>>, line: usize, col: usize) {
    let lstart = line - 1;
    let cstart = col - 1;
    for l in lstart..lstart+3 {
        for c in cstart..cstart+3 {
            print!("{} ", map[l][c]);
        }
        println!();
    }
}

fn find_connecting(map: &Vec<Vec<char>>, line: usize, col: usize) -> Vec<(usize, usize)> {    

    let mut connections = vec![];

    match map[line][col] {
        '|' => { connections.push( (line-1, col) ); connections.push( (line+1, col) ); },
        '-' => { connections.push( (line, col-1) ); connections.push( (line, col+1) ); },
        'L' => { connections.push( (line-1, col) ); connections.push( (line, col+1) ); },
        'J' => { connections.push( (line, col-1) ); connections.push( (line-1, col) ); },
        '7' => { connections.push( (line, col-1) ); connections.push( (line+1, col) ); },
        'F' => { connections.push( (line+1, col) ); connections.push( (line, col+1) ); },
        'S' => {
            let up = map[line-1][col];
            let down = map[line+1][col];
            let left = map[line][col-1];
            let right = map[line][col+1];

            if up == '|' || up == '7' || up == 'F' { connections.push( (line-1, col) ); }
            if down == '|' || down == 'L' || down == 'J' { connections.push( (line+1, col) ); }
            if left == '-' || left == 'L' || left == 'F' { connections.push( (line, col-1) ); }
            if right == '-' || right == '7' || right == 'J' { connections.push( (line, col+1) ); }

        },
        _ => todo!("!")
    }

    assert!(connections.len() == 2);

    connections
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
        visited_positions.push( (start_line, start_col) );
        let mut distance = 1;

        while positions[0] != positions[1] {
            let (l1, c1) = positions.pop().unwrap();
            let (l2, c2) = positions.pop().unwrap();

            dist_matrix[l1][c1] = distance;
            dist_matrix[l2][c2] = distance;

            visited_positions.push((l1, c1));
            visited_positions.push((l2, c2));

            match find_connecting(&self.map, l1, c1).iter().filter(|x| !visited_positions.contains(*x)).next() {
                Some(x) => {positions.push(*x)},
                None => {break; }
            };
            match find_connecting(&self.map, l2, c2).iter().filter(|x| !visited_positions.contains(*x)).next() {
                Some(x) => {positions.push(*x)},
                None => {break; }
            };

            distance += 1;
        }

        distance.to_string()
    }

    fn part_two(&self) -> String {
        todo!("Solve part two of day 10 using your parsed input")
    }
}
