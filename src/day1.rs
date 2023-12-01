use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day1 {
    lineas : Vec<String>
}

impl FromInput for Day1 {
    fn from_lines(lineas: impl Iterator<Item = String>) -> Self {
        let mut x = Day1 { lineas : Vec::new() };
        for l in lineas {
            x.lineas.push(l);
        }
        x
    }
}

impl DaySolution for Day1 {
    fn part_one(&self) -> String {
        let mut suma = 0;
        for l in &self.lineas {
            let mut primera = -1;
            let mut carga = -1;

            //println!("{}", l);
            for c in l.chars() {       
                if c.is_numeric() {
                    if primera < 0 {
                        primera = c as i32 - '0' as i32;
                        carga = primera;
                    } else {
                        carga = c as i32 - '0' as i32;
                    }
                }
            }
            //println!("{} {} {} {}", primera, carga, primera*10+carga, suma);
            suma += primera * 10 + carga;
        }
        suma.to_string()
    }

    fn part_two(&self) -> String {
        let mut suma = 0;

        let cifras = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let numeros = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        for l in &self.lineas {
            let mut primera: i32 = -1;
            let mut carga = -1;

            let mut posiciones_primeros: Vec<i32> = vec![-1; 10];
            let mut posiciones_cargos: Vec<i32> = vec![-1; 10];

            // println!("{}", l);
            for c in cifras.iter() {
                // println!("  {}", c);
                let ic = *c as usize - '0' as usize;
                if let Some(pos) = l.find(*c) {
                    // println!("    1st {}", pos);
                    posiciones_primeros[ic] = pos as i32;
                }
                if let Some(pos) = l.rfind(*c) {
                    // println!("    last {}", pos);
                    posiciones_cargos[ic] = pos as i32;
                }
            }

            for n in 1..10 {
                // println!("  {}", numeros[n]);
                if let Some(pos) = l.find(numeros[n]) {
                    // println!("      pos {} prev {}", pos, posiciones_primeros[n]);
                    if posiciones_primeros[n] >= 0 && posiciones_primeros[n] > pos as i32 {
                        // println!("    1st {}", pos);
                        posiciones_primeros[n] = pos as i32;
                    } else if posiciones_primeros[n] == -1 {
                        // println!("    1st {}", pos);
                        posiciones_primeros[n] = pos as i32;
                    }
                }
                if let Some(pos) = l.rfind(numeros[n]) {
                    // println!("      pos {} prev {}", pos, posiciones_cargos[n]);
                    if posiciones_cargos[n] >= 0 && posiciones_cargos[n] < pos as i32 {
                        // println!("    last {}", pos);
                        posiciones_cargos[n] = pos as i32;
                    } else if posiciones_cargos[n] == -1 {
                        // println!("    last {}", pos);
                        posiciones_cargos[n] = pos as i32;
                    }
                }
            }

            // println!("  prim {:?}", posiciones_primeros);
            // println!("  carg {:?}", posiciones_cargos);

            let mut pos_prim = -1;
            let mut pos_carg = -1;
            for n in 1..10 {
                if primera == -1 && posiciones_primeros[n] >= 0 {
                    primera = n as i32;
                    pos_prim = posiciones_primeros[n];
                } else if posiciones_primeros[n] >= 0 {
                    if posiciones_primeros[n] < pos_prim {
                        primera = n as i32;
                        pos_prim = posiciones_primeros[n];
                    }
                }

                if primera == -1 && posiciones_cargos[n] >= 0 {
                    carga = n as i32;
                    pos_carg = posiciones_cargos[n];
                } else if posiciones_cargos[n] >= 0 {
                    if posiciones_cargos[n] >= pos_carg {
                        carga = n as i32;
                        pos_carg = posiciones_cargos[n];
                    }
                }
                //println!("    p {} pp {} c {} pc {}", primera, pos_prim, carga, pos_carg);
            }
            // println!("  {} {} {} {}", primera, carga, primera*10+carga, suma);
            suma += primera * 10 + carga;
        }
        suma.to_string()
    }
}
