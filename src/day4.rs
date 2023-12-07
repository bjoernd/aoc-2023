use crate::{DaySolution, FromInput};
use std::collections::HashSet;

// TODO: Model the problem into this struct
struct Card {
    winning : HashSet<usize>,
    drawn : HashSet<usize>
}

pub struct Day4 {
    cards : Vec<Card>,
}

impl FromInput for Day4 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut d = Day4 { cards : vec![] };
        for l in lines {
            let mut card = Card{ winning : HashSet::new(), drawn : HashSet::new() };
            for (idx, val) in l.split(":").enumerate() {
                // ignore the Card ID
                if idx == 1 {
                    for (idx2, val2) in val.split("|").enumerate() {
                        // 0 == winning
                        if idx2 == 0 {
                            for v in val2.split(' ') {
                                if v == "" { continue; }
                                card.winning.insert(usize::from_str_radix(v, 10).unwrap());
                            }
                        } else { // 1 == drawn
                            for v in val2.split(" ") {
                                if v == "" { continue; }
                                card.drawn.insert(usize::from_str_radix(v, 10).unwrap());
                            }
                        }
                    }
                }
            }
            d.cards.push(card);
        }
        d
    }
}

impl DaySolution for Day4 {
    fn part_one(&self) -> String {
        let mut sum = 0;

        for c in &self.cards {
            let intersection : HashSet<_> = c.winning.intersection(&c.drawn).collect();
            
            // no match -> 0 points
            if intersection.is_empty() { continue; }
            
            let power: u32 = intersection.len() as u32 - 1;
            sum += 2_usize.pow(power);
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut counts = vec![ 1_usize; self.cards.len() ];

        for (i, c) in (&self.cards).into_iter().enumerate() {
            let intersection : HashSet<_> = c.winning.intersection(&c.drawn).collect();
            // no match -> no new cards
            if intersection.is_empty() { continue; }

            let count_won = intersection.len();
            let count_current = counts[i];

            for j in 1..count_won+1 {
                counts[i+j] += count_current;
            }

        }

        counts.iter().sum::<usize>().to_string()
    }
}
