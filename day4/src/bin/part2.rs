use regex::Regex;
use std::io;

#[derive(Debug)]
struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: bool,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: 0,
            iyr: 0,
            eyr: 0,
            hgt: "".to_string(),
            hcl: "".to_string(),
            ecl: "".to_string(),
            pid: "".to_string(),
            cid: false,
        }
    }

    fn byr_valide(&self) -> bool {
        self.byr >= 1920 && self.byr <= 2002
    }

    fn iyr_valide(&self) -> bool {
        self.iyr >= 2010 && self.iyr <= 2020
    }

    fn eyr_valide(&self) -> bool {
        self.eyr >= 2020 && self.eyr <= 2030
    }

    fn hgt_valide(&self) -> bool {
        let re = Regex::new(r"^(?P<value>[0-9]+)(?P<unit>cm|in)$").unwrap();
        match re.captures(&self.hgt) {
            None => false,
            Some(caps) => {
                let val = caps
                    .name("value")
                    .map_or("", |m| m.as_str())
                    .parse::<u32>()
                    .unwrap();
                let unit = caps.name("unit").map_or("", |m| m.as_str());

                (unit == "cm" && val >= 150 && val <= 193)
                    || (unit == "in" && val >= 59 && val <= 76)
            }
        }
    }

    fn hcl_valide(&self) -> bool {
        let re = Regex::new(r"^#([a-fA-F0-9]{6})$").unwrap();
        match re.captures(&self.hcl) {
            None => false,
            Some(_) => true,
        }
    }

    fn ecl_valide(&self) -> bool {
        let v: &str = &self.ecl;
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v)
    }

    fn pid_valide(&self) -> bool {
        let re = Regex::new(r"^([0-9]{9})$").unwrap();
        match re.captures(&self.pid) {
            None => false,
            Some(_) => true,
        }
    }

    fn is_valide(&self) -> bool {
        if !self.byr_valide() {
            println!("BYR invalid");
            return false;
        }
        if !self.iyr_valide() {
            println!("iyr invalid");
            return false;
        }
        if !self.eyr_valide() {
            println!("eyr invalid");
            return false;
        }
        if !self.hgt_valide() {
            println!("hgt invalid");
            return false;
        }
        if !self.hcl_valide() {
            println!("hcl invalid");
            return false;
        }
        if !self.ecl_valide() {
            println!("ecl invalid");
            return false;
        }
        if !self.pid_valide() {
            println!("pid invalid");
            return false;
        }

        return true;
    }

    fn set_byr(&mut self, value: String) {
        self.byr = value.parse().unwrap();
    }

    fn set_iyr(&mut self, value: String) {
        self.iyr = value.parse().unwrap();
    }

    fn set_eyr(&mut self, value: String) {
        self.eyr = value.parse().unwrap();
    }

    fn set_hgt(&mut self, value: String) {
        self.hgt = value;
    }

    fn set_hcl(&mut self, value: String) {
        self.hcl = value;
    }

    fn set_ecl(&mut self, value: String) {
        self.ecl = value;
    }

    fn set_pid(&mut self, value: String) {
        self.pid = value;
    }

    fn set_cid(&mut self, _value: String) {
        self.cid = true;
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
            //println!("{:?} - {}", current, current.is_valide());
            current = Passport::new();
            continue;
        }

        for raw_infos in input.trim().split(" ") {
            let infos = raw_infos.split(":").collect::<Vec<&str>>();
            let info_type = infos[0];
            match &info_type[..] {
                "byr" => current.set_byr(infos[1].to_string()),
                "iyr" => current.set_iyr(infos[1].to_string()),
                "eyr" => current.set_eyr(infos[1].to_string()),
                "hgt" => current.set_hgt(infos[1].to_string()),
                "hcl" => current.set_hcl(infos[1].to_string()),
                "ecl" => current.set_ecl(infos[1].to_string()),
                "pid" => current.set_pid(infos[1].to_string()),
                "cid" => current.set_cid(infos[1].to_string()),
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
