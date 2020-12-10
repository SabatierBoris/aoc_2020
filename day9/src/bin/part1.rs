use std::io;

fn main() {
    let mut numbers = Vec::new();
    const LIMIT: usize = 25;
    let mut offset = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == "" {
            break;
        }

        let number = input.trim().parse::<i32>().unwrap();

        if numbers.len() < LIMIT {
            numbers.push(number);
            continue;
        }

        let mut founded = false;

        for (i, tmp) in numbers[offset..(offset + LIMIT)].iter().enumerate() {
            let target = number - tmp;
            if numbers[(offset + i + 1)..].contains(&target) {
                println!("{}+{} = {} ({})", tmp, target, number, i);
                founded = true;
                break;
            }
        }

        if !founded {
            println!("Result => {}", number);
            break;
        }

        offset += 1;
        numbers.push(number);
    }
}
