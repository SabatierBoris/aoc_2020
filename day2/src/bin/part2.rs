use std::io;

#[derive(Debug)]
struct PasswordRule {
    pos1: usize,
    pos2: usize,
    letter: char,
}

impl PasswordRule {
    fn validate(self, password: String) -> bool {
        let letter1 = password.chars().nth(self.pos1).unwrap();
        let letter2 = password.chars().nth(self.pos2).unwrap();

        println!(
            "{} - {} - {}",
            letter1,
            letter2,
            letter1 == self.letter || letter2 == self.letter
        );

        return (letter1 == self.letter && letter2 != self.letter)
            || (letter1 != self.letter && letter2 == self.letter);
    }
}

impl From<String> for PasswordRule {
    fn from(raw: String) -> Self {
        let splited: Vec<&str> = raw.split(' ').collect();

        let raw_numbers = splited[0].trim().to_string();
        let letter = splited[1].trim().chars().nth(0).unwrap();

        let numbers: Vec<&str> = raw_numbers.split('-').collect();

        let pos1 = numbers[0].trim().parse::<usize>().unwrap() - 1;
        let pos2 = numbers[1].trim().parse::<usize>().unwrap() - 1;

        PasswordRule {
            pos1: pos1,
            pos2: pos2,
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
        let splited: Vec<&str> = raw.split(':').collect();

        let rule = PasswordRule::from(splited[0].trim().to_string());
        let password = splited[1].trim().to_string();

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
