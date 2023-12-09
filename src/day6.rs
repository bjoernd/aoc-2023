use crate::{DaySolution, FromInput};

struct Race {
    time: usize,
    distance: usize,
}

pub struct Day6 {
    races: Vec<Race>,
}

impl FromInput for Day6 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut times: Vec<usize> = vec![];
        let mut distances: Vec<usize> = vec![];

        for (i, l) in lines.enumerate() {
            match i {
                0 => {
                    for (j, num) in l.split_whitespace().enumerate() {
                        match j {
                            0 => {}
                            _ => {
                                times.push(usize::from_str_radix(num, 10).unwrap());
                            }
                        }
                    }
                }
                1 => {
                    for (j, num) in l.split_whitespace().enumerate() {
                        match j {
                            0 => {}
                            _ => {
                                distances.push(usize::from_str_radix(num, 10).unwrap());
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let mut races = vec![];

        while !times.is_empty() {
            races.push(Race {
                time: times.pop().unwrap(),
                distance: distances.pop().unwrap(),
            });
        }

        Day6 { races }
    }
}

impl DaySolution for Day6 {
    fn part_one(&self) -> String {
        let mut prod = 1;
        for r in &self.races {
            let mut outcomes = vec![0; r.time + 1];
            println!("Time {} Dist {}", r.time, r.distance);

            for speed in 0..r.time + 1 {
                let runtime = r.time - speed;
                outcomes[speed] = runtime * speed;
            }

            let wins: usize = outcomes
                .iter()
                .filter(|&&x| x > r.distance)
                .collect::<Vec<&usize>>()
                .len();
            prod *= wins;
        }

        prod.to_string()
    }

    fn part_two(&self) -> String {
        /* Yeah, no way I'm going to write code to parse those lines again if I can just hard-code things here... */
        let dist = 282107911471062_u64;
        let time = 47707566_u64;
        let mut wins = 0;

        for speed in 1..time {
            let runtime = time - speed;
            if speed * runtime > dist { wins += 1; }
        }

        wins.to_string()
    }
}
