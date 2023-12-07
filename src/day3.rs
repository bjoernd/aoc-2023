use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct

struct Linea {
    cars : Vec<char>
}

pub struct Day3 {
    lineas : Vec<Linea>
}

impl Day3 {
    // get character at coordinates
    fn get(&self, line: usize, column: usize) -> Result<char, &str> {
        if line >= self.lineas.len() ||
           column >= self.lineas[line].cars.len() {
            return Err("out of bounds")
           }
        Ok(self.lineas[line].cars[column])
    }

    fn next_number_offset(&self, line: usize, column: usize) -> Result<usize, &str> {
        let mut i = column;
        // move until first number or end
        while i < self.lineas[line].cars.len() &&
            !self.get(line, i).unwrap().is_numeric() {
                i += 1;
        }
        
        if i == self.lineas[line].cars.len() {
            return Err("no more number");
        }

        Ok(i)
    }

    // parse number at given coordinates
    fn parse_number(&self, line: usize, column: usize) -> Result<usize, &str> {
        let mut numero = 0 as usize;
        let mut num_offset = column;

        while let Ok(num) = self.get(line, num_offset) {
            if !num.is_numeric() { break; }
            numero *= 10;
            numero += num as usize - '0' as usize;
            num_offset += 1;
        }

        if num_offset == column {
            return Err("No number found");
        }

        Ok(numero)
    }

    fn num_len(&self, i: usize) -> usize {
        if i >= 1000000 {
            return 7;
        } else if i >= 100000 {
            return 6;
        } else if i >= 10000 {
            return 5;
        } else if i >= 1000 {
            return 4;
        } else if i >= 100 {
            return 3;
        } else if i >= 10 {
            return 2;
        } else if i >= 1 {
            return 1;
        } else {
            0
        }
    }   

    fn first_col(&self, column: usize) -> bool { column == 0 }
    fn first_line(&self, line: usize) -> bool { line == 0 }
    fn last_col(&self, line: usize, column: usize) -> bool { column >= self.lineas[line].cars.len() - 1 }
    fn last_line(&self, line: usize) -> bool { line == self.lineas.len() - 1 }

    fn is_part_no(&self, line: usize, column: usize) -> bool {
        if let Ok(num) = self.parse_number(line, column) {
            let next_free = column + self.num_len(num);
            
            if !self.first_col(column) && self.lineas[line].cars[column-1] != '.' { return true; }
            if !self.last_col(line, next_free) && self.lineas[line].cars[next_free] != '.' {
                 return true;
            }

            let mut start_col = column;
            if start_col > 0 { start_col -= 1; }
            let mut end_col = next_free;
            if next_free == self.lineas[line].cars.len() { end_col -= 1; }

            for n in start_col .. end_col+1 {
                if line > 0 {
                    if self.lineas[line-1].cars[n] != '.' { return true; }
                }
                if line < self.lineas.len() -1 {
                    if self.lineas[line+1].cars[n] != '.' { return true; }
                }
            }
        }
        false
    }

    fn gear_product(&self, line: usize, column: usize) -> usize {
        if self.get(line, column).unwrap() != '*' { return 0; }

        let mut neighbouring_numbers: Vec<Vec<usize>> = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];

        /*
        println!("-------------------------------------------------------------------------");
        if line > 0 {
            println!("{} {} {} {} {}",
                self.get(line-1, column-2).unwrap(),
                self.get(line-1, column-1).unwrap(),
                self.get(line-1, column).unwrap(),
                self.get(line-1, column+1).unwrap(),
                self.get(line-1, column+2).unwrap(),
            );
            println!("{} {} {} {} {}",
                self.get(line, column-2).unwrap(),
                self.get(line, column-1).unwrap(),
                self.get(line, column).unwrap(),
                self.get(line, column+1).unwrap(),
                self.get(line, column+2).unwrap(),
            );
            println!("{} {} {} {} {}",
                self.get(line+1, column-2).unwrap(),
                self.get(line+1, column-1).unwrap(),
                self.get(line+1, column).unwrap(),
                self.get(line+1, column+1).unwrap(),
                self.get(line+1, column+2).unwrap(),
            );
        }
        */

        /* left */
        if column > 0 {
            let mut c = column - 1;
            let mut num = 0;
            
            while let Ok(v) = self.get(line, c) {
                if v.is_numeric() && c > 0 {
                    c -= 1;
                } else {
                    break;
                }
            }

            if c == 0 && self.get(line, c).unwrap().is_numeric() {
                num = self.parse_number(line, c).unwrap();
            } else if c < column - 1 {
                num = self.parse_number(line, c+1).unwrap();
            }

            neighbouring_numbers[1][0] = num;
        }

        /* right */
        if column < self.lineas[line].cars.len() - 1 {
            let mut c = column + 1;
            let mut num = 0;

            while let Ok(v) = self.get(line, c) {
                if v.is_numeric() {
                    c += 1;
                } else {
                    break;
                }
            }

            if c > column + 1 {
                num = self.parse_number(line, column + 1).unwrap();
            }

            neighbouring_numbers[1][2] = num;
        }

        /* top */
        if !self.first_line(line) {
            let l = line - 1;
            let mut num = 0;
            //println!("checking top: {:?}", self.get(l, column).unwrap().is_numeric());
            if self.get(l, column).unwrap().is_numeric() {
                let mut c = column;
                while let Ok(v) = self.get(l, c) {
                    if v.is_numeric() && c > 0 {
                        c -= 1;
                    } else {
                        break;
                    }
                }
                if c == 0 && self.get(l, c).unwrap().is_numeric() {
                    num = self.parse_number(l, c).unwrap();
                } else if c < column {
                    num = self.parse_number(l, c+1).unwrap();
                }
                neighbouring_numbers[0][1] = num;
            } else {
                /* top left + right */
                /* We know that top is NOT a number, so for top right, we just need to parse */
                if !self.last_col(l, column) && self.get(l, column+1).unwrap().is_numeric() {
                    num = self.parse_number(l, column+1).unwrap();
                    neighbouring_numbers[0][2] = num;
                }

                if column > 0 && !self.last_col(l, column) && self.get(l, column-1).unwrap().is_numeric() {
                    let mut c = column - 1;
                    while let Ok(v) = self.get(l, c) {
                        if v.is_numeric() && c > 0 {
                            c -= 1;
                        } else {
                            break;
                        }
                    }
                    if c == 0 && self.get(l, c).unwrap().is_numeric() {
                        num = self.parse_number(l, c).unwrap();
                    } else if c < column - 1 {
                        num = self.parse_number(l, c+1).unwrap();
                    }
                    
                    neighbouring_numbers[0][0] = num;
                }
            }
        }

        /* bottom */
        if ! self.last_line(line) {
            let l = line + 1;
            let mut num = 0;
            if self.get(l, column).unwrap().is_numeric() {
                let mut c = column;
                while let Ok(v) = self.get(l, c) {
                    if v.is_numeric() && c > 0 {
                        c -= 1;
                    } else {
                        break;
                    }
                }
                if c == 0 && self.get(l, c).unwrap().is_numeric() {
                    num = self.parse_number(l, c).unwrap();
                } else if c < column {
                    num = self.parse_number(l, c+1).unwrap();
                }
                neighbouring_numbers[2][1] = num;
            } else {
                /* bottom left + right */
                /* We know that below is NOT a number, so for bottom right, we just need to parse */
                if !self.last_col(l, column) && self.get(l, column+1).unwrap().is_numeric() {
                    num = self.parse_number(l, column+1).unwrap();
                    neighbouring_numbers[2][2] = num;
                }

                if !self.last_col(l, column) && self.get(l, column-1).unwrap().is_numeric() {
                    let mut c = column - 1;
                    while let Ok(v) = self.get(l, c) {
                        if v.is_numeric() && c > 0 {
                            c -= 1;
                        } else {
                            break;
                        }
                    }
                    if c == 0 && self.get(l, c).unwrap().is_numeric() {
                        num = self.parse_number(l, c).unwrap();
                    } else if c < column - 1 {
                        num = self.parse_number(l, c+1).unwrap();
                    }
                    
                    neighbouring_numbers[2][0] = num;
                }
            }
        }

        /*
        println!("");
        println!("{:?}", neighbouring_numbers[0]);
        println!("{:?}", neighbouring_numbers[1]);
        println!("{:?}", neighbouring_numbers[2]);
        */

        let mut ncount = 0;
        let mut nprod = 1;
        for i in 0..3 {
            for j in 0..3 {
                if neighbouring_numbers[i][j] != 0 {
                    ncount += 1;
                    nprod *= neighbouring_numbers[i][j];
                }
            }
        }

        //println!("CNT {} PROD {}", ncount, nprod);
        if ncount == 2 { return nprod; }

        0
    }
}

impl FromInput for Day3 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut d = Day3 { lineas : vec![] };
        for l in lines {
            let mut lin = Linea { cars: vec![] };
            for c in l.as_bytes() {
                lin.cars.push(*c as char);
            }
            d.lineas.push(lin);
        }
        d
    }
}

impl DaySolution for Day3 {
    fn part_one(&self) -> String {
        let mut sum = 0;
        for line in 0..self.lineas.len() {
            //println!("{:?}", self.lineas[line].cars);
            let mut column = 0;
            while let Ok(offset) = self.next_number_offset(line, column) {
                if let Ok(num) = self.parse_number(line, offset) {
                    if self.is_part_no(line, offset) {
                        //println!("    off {} ==> {} PART", offset, num);
                        sum += num;
                    } else {
                        //println!("    off {} ==> {}", offset, num);
                    }
                    column = offset + self.num_len(num);
                    //println!("  {}", column);
                }
            }
        }
        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum: usize = 0;

        for line in 0..self.lineas.len() {
            for column in 0..self.lineas[line].cars.len() {
                if self.get(line, column).unwrap() == '*' {
                    sum += self.gear_product(line, column);
                }
            }
        }

        sum.to_string()
    }
}
