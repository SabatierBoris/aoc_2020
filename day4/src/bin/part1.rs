use std::io;

#[derive(Debug)]
struct Passport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: false,
            iyr: false,
            eyr: false,
            hgt: false,
            hcl: false,
            ecl: false,
            pid: false,
            cid: false,
        }
    }

    fn is_valide(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}

fn main() {
    let mut current = Passport::new();
    let mut nb_valid = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == "" {
            break;
        }

        if input.trim() == "" {
            if current.is_valide() {
                nb_valid += 1
            }
            println!("{:?} - {}", current, current.is_valide());
            current = Passport::new();
            continue;
        }

        for raw_infos in input.trim().split(" ") {
            let infos = raw_infos.split(":").collect::<Vec<&str>>();
            let info_type = infos[0];
            match &info_type[..] {
                "byr" => current.byr = true,
                "iyr" => current.iyr = true,
                "eyr" => current.eyr = true,
                "hgt" => current.hgt = true,
                "hcl" => current.hcl = true,
                "ecl" => current.ecl = true,
                "pid" => current.pid = true,
                "cid" => current.cid = true,
                _ => println!("??"),
            };
        }
    }

    if current.is_valide() {
        nb_valid += 1
    }
    println!("{:?} - {}", current, current.is_valide());

    println!("{} ", nb_valid);
}
