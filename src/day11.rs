use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day11 {
    map: Vec<Vec<char>>,
}

impl FromInput for Day11 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut map: Vec<Vec<char>> = vec![];

        for l in lines {
            map.push(l.chars().collect());
            // line full of dots? -> duplicate
            if l.chars().filter(|x| *x == '.').count() == l.len() {
                map.push(l.chars().collect());
            }
        }

        // now expand columns with only dots. Find the columns first and only then expand
        // in order to avoid counting too many
        let mut dot_columns = vec![];
        for c in 0..map[0].len() {
            let column = map.iter().map(|x| x[c]).collect::<Vec<char>>();
            if column.iter().filter(|x| **x == '.').count() == column.len() {
                dot_columns.push(c)
            }
        }

        for (i, c) in dot_columns.iter().enumerate() {
            for line in &mut map {
                line.insert(*c + i, '.');
            }
        }

        Day11 { map }
    }
}

fn manhattan_dist( p1: &(usize, usize), p2: &(usize, usize) ) -> usize {
    let dx = (p1.0 as i32 - p2.0 as i32).abs() as usize;
    let dy = (p1.1 as i32 - p2.1 as i32).abs() as usize;
    dx + dy
}

impl DaySolution for Day11 {
    fn part_one(&self) -> String {
        let mut sum = 0;

        println!("{}x{}", self.map.len(), self.map[0].len());

        // collect points
        let mut points = vec![];
        for l in 0..self.map.len() {
            for c in 0..self.map[0].len() {
                if self.map[l][c] == '#' {
                    points.push((l, c));
                }
            }
        }

        for (idx, p1) in points.iter().enumerate() {
            for p2 in points[idx+1..].iter() {
                sum += manhattan_dist(p1, p2);
            }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        todo!("Solve part two of day 11 using your parsed input")
    }
}
