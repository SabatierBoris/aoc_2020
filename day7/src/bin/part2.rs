use regex::Regex;
use std::collections::HashMap;
use std::io;

fn get_contained_numbers(bags: &HashMap<String, HashMap<String, u32>>, lookup: String) -> u32 {
    let mut score = 0;

    let contained = &bags[&lookup];

    for (bag, number) in contained.iter() {
        score = score + number + number * get_contained_numbers(bags, bag.to_string());
    }

    return score;
}

fn main() {
    let re = Regex::new(r"(?P<number>[\d]+) (?P<name>[\w]+ [\w]+) bag[s]?").unwrap();
    let mut bags = HashMap::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == "" {
            break;
        }

        let first_process: Vec<_> = input.trim().split(" bags contain ").collect();

        let source = first_process[0].to_string();
        let raw_result = first_process[1];
        let mut result = HashMap::new();

        if raw_result != "no other bags." {
            let second_process: Vec<_> = raw_result.trim().split(",").collect();
            for item in second_process.into_iter() {
                let caps = re.captures(&item.trim()).unwrap();
                let name = caps.name("name").map_or("", |m| m.as_str());
                let number = caps
                    .name("number")
                    .map_or("", |m| m.as_str())
                    .parse::<u32>()
                    .unwrap();
                result.insert(name.to_string(), number);
            }
        }

        bags.insert(source, result);
    }

    let score = get_contained_numbers(&bags, "shiny gold".to_string());

    println!("{:?}", score);
}
