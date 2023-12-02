use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct

struct Fichas {
    rojo : usize,
    verde : usize,
    azul : usize,
}

struct Juego {
    id : usize,
    fichas : Vec<Fichas>
}

impl Juego {
    fn max_rojo(&self) -> usize {
        let mut max = 0;
        for f in &self.fichas {
            if f.rojo > max { max = f.rojo; }
        }
        max
    }

    fn max_verde(&self) -> usize {
        let mut max = 0;
        for f in &self.fichas {
            if f.verde > max { max = f.verde; }
        }
        max
    }

    fn max_azul(&self) -> usize {
        let mut max = 0;
        for f in &self.fichas {
            if f.azul > max { max = f.azul; }
        }
        max
    }
}

pub struct Day2 {
    juegos : Vec<Juego>,
}

impl FromInput for Day2 {
    fn from_lines(lineas: impl Iterator<Item = String>) -> Self {
        let mut d = Day2{ juegos : Vec::new() };
        for l in lineas {
            let parte_id : Vec<&str> = l.split(":").collect();
            let parte_num : Vec<&str> = parte_id[0].split(" ").collect();
            let num = parte_num[1].replace(":", "");

            let mut j = Juego{ id: usize::from_str_radix(num.as_str(), 10).unwrap(),
                                      fichas : Vec::new() };
            
            for jugada in parte_id[1].split(";").collect::<Vec<&str>>() {
                let mut rojo = 0 as usize;
                let mut verde = 0 as usize;
                let mut azul = 0 as usize;
                for color in jugada.split(",").collect::<Vec<&str>>() {
                    let mut count = 0;
                    for (i, v) in color.trim().split(" ").enumerate() {
                        if i == 0 {
                            count = usize::from_str_radix(v, 10).unwrap();
                        } else {
                            match v {
                                "red" => { rojo = count; },
                                "green" => { verde = count; },
                                "blue" => { azul = count; },
                                &_ => panic!("Unexpected value: {}", v)
                            }
                        }
                    }
                    j.fichas.push(Fichas{rojo, verde, azul});
                }
                println!("{}", jugada);
            }

            d.juegos.push(j);
        }
        d
    }
}

fn valid_juego(j: &Juego, lim_rojo: usize, lim_verde: usize, lim_azul: usize) -> bool {
    let mut ok = true;
    for f in &j.fichas {
        if f.rojo > lim_rojo { ok = false; break; }
        if f.verde > lim_verde { ok = false; break; }
        if f.azul > lim_azul { ok = false; break; }
    }        
    ok
}

impl DaySolution for Day2 {
    fn part_one(&self) -> String {
        let lim_rojo = 12 as usize;
        let lim_verde = 13 as usize;
        let lim_azul = 14 as usize;
        let mut suma = 0;
        for juego in &self.juegos {
            if valid_juego(&juego, lim_rojo, lim_verde, lim_azul) {
                suma += juego.id;
            }
        }
        suma.to_string()
    }

    fn part_two(&self) -> String {
        let mut suma = 0;
        for j in &self.juegos {
            let power = j.max_azul() * j.max_rojo() * j.max_verde();
            suma += power;
        }
        suma.to_string()
    }
}
