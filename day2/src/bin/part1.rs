use regex::Regex;
use std::io;

#[derive(Debug)]
struct PasswordRule {
    min: u32,
    max: u32,
    letter: char,
}

impl PasswordRule {
    fn validate(self, password: String) -> bool {
        let mut cpt = 0;
        for c in password.chars() {
            if c == self.letter {
                cpt += 1;
            }
        }
        return self.min <= cpt && cpt <= self.max;
    }
}

impl From<String> for PasswordRule {
    fn from(raw: String) -> Self {
        let re = Regex::new(r"^(?P<min>[0-9]+)-(?P<max>[0-9]+) (?P<char>[a-z]{1})$").unwrap();
        let caps = re.captures(&raw).unwrap();

        let min = caps
            .name("min")
            .map_or("", |m| m.as_str())
            .parse::<u32>()
            .unwrap();
        let max = caps
            .name("max")
            .map_or("", |m| m.as_str())
            .parse::<u32>()
            .unwrap();
        let letter = caps
            .name("max")
            .map_or("", |m| m.as_str())
            .parse::<u32>()
            .unwrap();

        let splited: Vec<&str> = raw.split(' ').collect();

        let letter = splited[1].trim().chars().nth(0).unwrap();

        PasswordRule {
            min: min,
            max: max,
            letter: letter,
        }
    }
}

#[derive(Debug)]
struct Line {
    rule: PasswordRule,
    password: String,
}

impl Line {
    fn is_valid(self) -> bool {
        self.rule.validate(self.password)
    }
}

impl From<String> for Line {
    fn from(raw: String) -> Self {
        let re = Regex::new(r"^(?P<rule>.+): (?P<password>[a-z]+)$").unwrap();
        let caps = re.captures(&raw).unwrap();

        let raw_rule = caps.name("rule").map_or("", |m| m.as_str()).to_string();
        let rule = raw_rule.into();

        let password = caps.name("password").map_or("", |m| m.as_str()).to_string();

        Line {
            rule: rule,
            password: password,
        }
    }
}

fn read_line() -> Result<Line, io::Error> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    Ok(Line::from(input.trim().to_string()))
}

fn main() {
    let mut valid_password = 0;
    loop {
        match read_line() {
            Ok(line) => {
                println!("{:?}", line);
                if line.is_valid() {
                    println!("Valide");
                    valid_password += 1;
                } else {
                    println!("Invalide");
                }
            }
            Err(error) => {
                println!("{:?}", error);
                break;
            }
        }
    }
    println!("{}", valid_password);
}
