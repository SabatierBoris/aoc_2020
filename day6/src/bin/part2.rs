use std::collections::HashSet;
use std::io;

fn main() {
    let mut answers: Vec<HashSet<char>> = Vec::new();
    let mut total = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == "" {
            break;
        }

        if input.trim() == "" {
            let intersection = answers.iter().skip(1).fold(answers[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
            });
            println!("size : {} ({:?})", intersection.len(), intersection);
            println!("===New group===");
            total += intersection.len();
            answers = Vec::new();
            continue;
        }

        let chars: HashSet<char> = input.trim().chars().collect();
        answers.push(chars);
    }

    let intersection = answers.iter().skip(1).fold(answers[0].clone(), |acc, hs| {
        acc.intersection(hs).cloned().collect()
    });
    println!("size : {} ({:?})", intersection.len(), intersection);
    total += intersection.len();

    println!("Total : {}", total);
}
