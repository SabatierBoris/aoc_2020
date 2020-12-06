use std::collections::HashSet;
use std::io;

fn main() {
    let mut answers: HashSet<char> = HashSet::new();
    let mut total = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == "" {
            break;
        }

        if input.trim() == "" {
            println!("size : {} ({:?})", answers.len(), answers);
            println!("===New group===");
            total += answers.len();
            answers = HashSet::new();
            continue;
        }

        let chars: HashSet<char> = input.trim().chars().collect();
        answers.extend(&chars);
    }
    println!("size : {} ({:?})", answers.len(), answers);
    total += answers.len();

    println!("Total : {}", total);
}
