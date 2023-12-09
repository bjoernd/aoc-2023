use crate::{DaySolution, FromInput};
use std::collections::HashMap;

#[derive(Clone)]
struct Hand {
    cards : Vec<char>,
    bid : usize,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn card_val_generic(c: char, card_values: &str) -> usize {
    match card_values.find(c) {
        Some(v) => {v+1}
        None => { 0}
    }
}

fn card_val(c: char) -> usize {
    card_val_generic(c, "23456789TJQKA")
}

fn card_val2(c: char) -> usize {
    card_val_generic(c, "J23456789TQKA")
}

impl Hand {

    fn hand_type_common(&self, card_counts: &HashMap<char, usize>) -> HandType {
        if card_counts.values().max().unwrap() == &5 {
            return HandType::FiveOfAKind;
        }

        if card_counts.values().max().unwrap() == &4 {
            return HandType::FourOfAKind;
        }

        let mut threes = 0;
        let mut pairs = 0;

        for v in card_counts.values() {
            if v == &3 { threes += 1; }
            if v == &2 { pairs += 1; }
        }

        if threes == 1 && pairs == 1 {
            return HandType::FullHouse;
        } else if threes == 1 {
            return HandType::ThreeOfAKind;
        } else if pairs == 2 {
            return HandType::TwoPair;
        } else if pairs == 1 {
            return HandType::OnePair;
        }

        HandType::HighCard
    }

    fn count_cards(&self) -> HashMap<char, usize> {
        let mut card_counts : HashMap<char, usize> = HashMap::new();
        for c in &self.cards {
            match card_counts.get(c) {
                Some(i) => { card_counts.insert(*c, i+1); },
                None => { card_counts.insert(*c, 1); },
            }
        }
        card_counts
    }

    fn hand_type(&self) -> HandType {
        let counts = self.count_cards();
        self.hand_type_common(&counts)
    }

    fn hand_type2(&self) -> HandType {
        let mut card_counts = self.count_cards();

        // apply jokers by adding to the highest card count
        let count_j = card_counts.get(&'J').unwrap_or(&0);
        if count_j > &0 {
            let mut max_k = ' ';
            let mut max_v = 0_usize;
            for (k,v) in card_counts.iter() {
                if *k != 'J' && v > &max_v {
                    max_v = *v;
                    max_k = *k;
                }
            }

            card_counts.insert(max_k, max_v + count_j);
            card_counts.insert('J', 0);
        }

        self.hand_type_common(&card_counts)
    }

    fn compare(&self, other: &Hand) -> core::cmp::Ordering {
        /* If type is the same, we need to go by cards */
        if self.hand_type() == other.hand_type() {
            let mut idx = 0;
            while idx < 5 {
                if self.cards[idx] != other.cards[idx] {
                    return card_val(self.cards[idx]).cmp(&card_val(other.cards[idx]))
                }
                idx += 1;
            }
            return core::cmp::Ordering::Equal;
        }
        /* Otherwise, type rules the comparison */
        self.hand_type().cmp(&other.hand_type())
    }

    fn compare2(&self, other: &Hand) -> core::cmp::Ordering {
        /* If type is the same, we need to go by cards */
        if self.hand_type2() == other.hand_type2() {
            let mut idx = 0;
            while idx < 5 {
                if self.cards[idx] != other.cards[idx] {
                    return card_val2(self.cards[idx]).cmp(&card_val2(other.cards[idx]))
                }
                idx += 1;
            }
            return core::cmp::Ordering::Equal;
        }
        /* Otherwise, type rules the comparison */
        self.hand_type2().cmp(&other.hand_type2())
    }
}

// TODO: Model the problem into this struct
pub struct Day7 {
    hands : Vec<Hand>,
}

impl FromInput for Day7 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut d = Day7{ hands: vec![] };
        for l in lines {
            let mut h = Hand{ cards: vec![], bid: 0 };
            for (i, v) in l.split_whitespace().enumerate() {
                match i {
                    0 => {
                        for c in v.chars() {
                            h.cards.push(c);
                        }
                    },
                    1 => {
                        h.bid = usize::from_str_radix(v, 10).unwrap();
                    },
                    _ => {}
                }
            }
            d.hands.push(h);
        }
        d
    }
}

impl DaySolution for Day7 {
    fn part_one(&self) -> String {

        let mut hands = self.hands.to_vec();

        hands.sort_by(|a, b| a.compare(b));
        
        let mut rank = 1;
        let mut sum = 0;
        for h in hands {
            //let s: String = h.cards.iter().collect();
            //println!("{} {} {:?}", s, h.bid, h.hand_type());
            sum += rank * h.bid;
            rank += 1;
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut hands = self.hands.to_vec();

        hands.sort_by(|a, b| a.compare2(b));
        
        let mut rank = 1;
        let mut sum = 0;
        for h in hands {
            //let s: String = h.cards.iter().collect();
            //println!("{} {} {:?}", s, h.bid, h.hand_type2());
            sum += rank * h.bid;
            rank += 1;
        }

        sum.to_string()
    }
}
