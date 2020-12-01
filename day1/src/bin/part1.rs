extern crate day1;

use std::collections::HashSet;

fn main() {
    let mut numbers = HashSet::<i32>::new();
    loop {
        match day1::read_number() {
            Ok(num) => {
                let diff = 2020 - num;
                if numbers.contains(&diff) {
                    println!("result = {}", num * diff);
                    break;
                } else {
                    numbers.insert(num);
                };
            }
            Err(error) => {
                println!("{:?}", error);
                break;
            }
        }
    }
}
