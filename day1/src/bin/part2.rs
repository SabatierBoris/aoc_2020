extern crate day1;

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut numbers = HashSet::<i32>::new();
    let mut tmp_sum = HashMap::<i32, i32>::new();
    loop {
        match day1::read_number() {
            Ok(num) => {
                let diff = 2020 - num;
                if tmp_sum.contains_key(&diff) {
                    let val = tmp_sum.get(&diff).unwrap();
                    println!("result = {}", num * val);
                    break;
                } else {
                    for tmp in &numbers {
                        tmp_sum.insert(num + tmp, tmp * num);
                    }
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
