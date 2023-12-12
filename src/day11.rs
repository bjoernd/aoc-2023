use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day11 {
    map: Vec<Vec<char>>,
    special_cols: Vec<usize>,
    special_lines: Vec<usize>,
}

impl FromInput for Day11 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut map: Vec<Vec<char>> = vec![];
        let mut extra_lines = vec![];
        let mut extra_cols = vec![];

        for (idx, l) in lines.enumerate() {
            map.push(l.chars().collect());
            // line full of dots? -> duplicate
            if l.chars().filter(|x| *x == '.').count() == l.len() {
                extra_lines.push(idx);
            }
        }

        for c in 0..map[0].len() {
            let column = map.iter().map(|x| x[c]).collect::<Vec<char>>();
            if column.iter().filter(|x| **x == '.').count() == column.len() {
                extra_cols.push(c)
            }
        }

        Day11 {
            map,
            special_cols: extra_cols,
            special_lines: extra_lines,
        }
    }
}

impl Day11 {
    fn special_manhattan_dist(
        &self,
        p1: &(usize, usize),
        p2: &(usize, usize),
        add_extra: usize,
    ) -> usize {
        let dx = (p1.0 as i32 - p2.0 as i32).unsigned_abs() as usize;
        let dy = (p1.1 as i32 - p2.1 as i32).unsigned_abs() as usize;

        let lmax = p1.0.max(p2.0);
        let lmin = p1.0.min(p2.0);

        let cmax = p1.1.max(p2.1);
        let cmin = p1.1.min(p2.1);

        let mut extra_space = 0;
        for col in &self.special_cols {
            if cmin < *col && *col < cmax {
                extra_space += add_extra;
            }
        }

        for line in &self.special_lines {
            if lmin < *line && *line < lmax {
                extra_space += add_extra;
            }
        }

        dx + dy + extra_space
    }
}

impl DaySolution for Day11 {
    fn part_one(&self) -> String {
        let mut sum = 0;

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
            for p2 in points[idx + 1..].iter() {
                sum += self.special_manhattan_dist(p1, p2, 1);
            }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0;

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
            for p2 in points[idx + 1..].iter() {
                sum += self.special_manhattan_dist(p1, p2, 999999);
            }
        }

        sum.to_string()
    }
}
