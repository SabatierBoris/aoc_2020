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
            println!("Look for => {}", number);
            let mut start = 0;
            let mut end = 0;
            loop {
                let slice = &numbers[start..end];
                let sum: i32 = slice.iter().sum();
                if sum == number {
                    let min = slice.iter().min().unwrap();
                    let max = slice.iter().max().unwrap();
                    println!("Result {}", min + max);
                    break;
                } else if sum > number {
                    start += 1;
                } else {
                    end += 1;
                }
            }
            break;
        }

        offset += 1;
        numbers.push(number);
    }
}
