use crate::{DaySolution, FromInput};
use num::integer::lcm;
use std::collections::HashMap;

struct Target {
    left: String,
    right: String,
}

// TODO: Model the problem into this struct
pub struct Day8 {
    targets: HashMap<String, Target>,
    directions: Vec<char>,
}

impl FromInput for Day8 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut directions: Vec<char> = vec![];
        let mut connections = HashMap::new();

        for (i, l) in lines.enumerate() {
            if i == 0 {
                directions = l.chars().collect::<Vec<_>>();
            } else {
                if l.is_empty() {
                    continue;
                }
                let raw = l.replace([',', ')', '(', '='], "");

                let mut source: String = String::new();
                let mut dest_l: String = String::new();
                let mut dest_r: String = String::new();

                for (j, v) in raw.split_whitespace().enumerate() {
                    match j {
                        0 => {
                            source = String::from(v);
                        }
                        1 => {
                            dest_l = String::from(v);
                        }
                        2 => {
                            dest_r = String::from(v);
                        }
                        _ => {}
                    }
                }

                connections.insert(
                    source,
                    Target {
                        left: dest_l,
                        right: dest_r,
                    },
                );
            }
        }
        Day8 {
            targets: connections,
            directions,
        }
    }
}

fn nodes_that_end_with_a(data: &Day8) -> Vec<String> {
    let mut nodes = Vec::new();
    for node in data.targets.keys() {
        if node.chars().nth(2).unwrap() == 'A' {
            nodes.push(node.to_string());
        }
    }
    nodes
}

impl DaySolution for Day8 {
    fn part_one(&self) -> String {
        let mut current_node = String::from("AAA");
        let mut steps = 0;

        while current_node.as_str() != "ZZZ" {
            let next_direction = self.directions[steps % self.directions.len()];
            match next_direction {
                'L' => {
                    current_node = self.targets[&current_node].left.clone();
                }
                'R' => {
                    current_node = self.targets[&current_node].right.clone();
                }
                _ => {}
            }
            steps += 1;
        }

        steps.to_string()
    }

    fn part_two(&self) -> String {
        let current_nodes = nodes_that_end_with_a(self);
        let mut total_steps = vec![];

        //println!("Nodes to follow: {}", current_nodes.len());

        for cn in current_nodes {
            let mut current_node = cn.clone();
            let mut steps = 0;
            while current_node.chars().nth(2).unwrap() != 'Z' {
                let next_direction = self.directions[steps % self.directions.len()];
                match next_direction {
                    'L' => {
                        current_node = self.targets[&current_node].left.clone();
                    }
                    'R' => {
                        current_node = self.targets[&current_node].right.clone();
                    }
                    _ => {}
                }
                steps += 1;
            }
            total_steps.push(steps);
        }

        //println!("{:?}", total_steps);

        let mut least_common_multiple = total_steps.pop().unwrap();

        while let Some(next) = total_steps.pop() {
            least_common_multiple = lcm(least_common_multiple, next);
        }

        least_common_multiple.to_string()
    }
}
