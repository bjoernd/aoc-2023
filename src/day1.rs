use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day1 {
    lines : Vec<String>
}

impl FromInput for Day1 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut x = Day1 { lines : Vec::new() };
        for l in lines {
            x.lines.push(l);
        }
        x
    }
}

impl DaySolution for Day1 {
    fn part_one(&self) -> String {
        let mut sum = 0;
        for l in &self.lines {
            let mut first = -1;
            let mut last = -1;

            //println!("{}", l);
            for c in l.chars() {       
                if c.is_numeric() {
                    if first < 0 {
                        first = c as i32 - '0' as i32;
                        last = first;
                    } else {
                        last = c as i32 - '0' as i32;
                    }
                }
            }
            //println!("{} {} {} {}", first, last, first*10+last, sum);
            sum += first * 10 + last;
        }
        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0;
        for l in &self.lines {
        }
        sum.to_string()
    }
}
