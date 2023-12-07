use crate::{DaySolution, FromInput};
use itertools::Itertools;

struct MapEntry {
    source: usize,
    destination : usize,
    length : usize,
}

impl MapEntry {
    fn contains(&self, value : usize) -> bool {
        self.source <= value && value < self.source + self.length
    }
}

struct CustomMap {
    entries : Vec<MapEntry>,
    _name : String,
}

// TODO: Model the problem into this struct
pub struct Day5 {
    seed2soil : CustomMap,
    soil2fert : CustomMap,
    fert2water : CustomMap,
    water2light : CustomMap,
    light2temp : CustomMap,
    temp2humid : CustomMap,
    humid2location : CustomMap,
    seeds : Vec<usize>,
}

impl CustomMap {
    fn new(n : &str) -> CustomMap {
        CustomMap{ entries : vec![], _name : String::from(n) }
    }

    fn add_entry(&mut self, input: &String) {
        let mut iter = input.split_whitespace();
        self.entries.push(MapEntry{
            destination : usize::from_str_radix(iter.next().unwrap(), 10).unwrap(),
            source : usize::from_str_radix(iter.next().unwrap(), 10).unwrap(),
            length : usize::from_str_radix(iter.next().unwrap(), 10).unwrap(),
        });
    }

    fn lookup(&self, value: usize) -> usize {
        for me in &self.entries {
            if me.contains(value) {
                let off = value - me.source;
                let val = me.destination + off;

                return val;
            }
        }

        // if no MapEntry, this is an identity mapping
        value
    }
}

impl FromInput for Day5 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut d = Day5{
            seed2soil : CustomMap::new("seed-to-soil"),
            soil2fert : CustomMap::new("soil-to-fertilizer"),
            fert2water : CustomMap::new("fertilizer-to-water"),
            water2light: CustomMap::new("water-to-light"),
            light2temp : CustomMap::new("light-to-temperatur"),
            temp2humid : CustomMap::new("temperature-to-humidity"),
            humid2location : CustomMap::new("humidity-to-location"),
            seeds : vec![]
        };

        let mut next_map = &mut d.seed2soil;

        for l in lines {
            if l == "" { continue; }

            if l.starts_with("seeds:") {
                for (i, v) in l.split(" ").enumerate() {
                    // skip first column
                    if i > 0 {
                        d.seeds.push(usize::from_str_radix(v, 10).unwrap());
                    }
                }
                continue;
            }

            match l.as_str() {
                "seed-to-soil map:" => { next_map = &mut d.seed2soil; },
                "soil-to-fertilizer map:" => { next_map = &mut d.soil2fert; },
                "fertilizer-to-water map:" => { next_map = &mut d.fert2water; },
                "water-to-light map:" => { next_map = &mut d.water2light; },
                "light-to-temperature map:" => { next_map = &mut d.light2temp; },
                "temperature-to-humidity map:" => { next_map = &mut d.temp2humid; },
                "humidity-to-location map:" => { next_map = &mut d.humid2location; },
                &_ => { /* everything else is a mapping entry */
                    next_map.add_entry(&l);
                }
            }
        }

        d
    }
}

impl DaySolution for Day5 {
    fn part_one(&self) -> String {
        let mut locations = vec![];
        for seed in &self.seeds {
            let soil = self.seed2soil.lookup(*seed);
            let fert = self.soil2fert.lookup(soil);
            let water = self.fert2water.lookup(fert);
            let light = self.water2light.lookup(water);
            let temp = self.light2temp.lookup(light);
            let hum = self.temp2humid.lookup(temp);
            let location = self.humid2location.lookup(hum);
            locations.push(location);
        }

        locations.iter().min().unwrap().to_string()
    }

    fn part_two(&self) -> String {
        let mut iter = self.seeds.iter();
        let mut min_loc = usize::MAX;

        while let Some((start, length)) = iter.next_tuple() {
            println!("{}..{}", start, length);

            for x in *start..(start+length) {
                let soil = self.seed2soil.lookup(x);
                let fert = self.soil2fert.lookup(soil);
                let water = self.fert2water.lookup(fert);
                let light = self.water2light.lookup(water);
                let temp = self.light2temp.lookup(light);
                let hum = self.temp2humid.lookup(temp);
                let location = self.humid2location.lookup(hum);

                if min_loc > location { min_loc = location; }
            }
        }

        min_loc.to_string()
    }
}
