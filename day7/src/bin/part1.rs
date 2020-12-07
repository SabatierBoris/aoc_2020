use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

fn get_containors_of(bags: &HashMap<String, HashSet<String>>, lookup: String) -> HashSet<String> {
    let mut containors = HashSet::new();
    let mut sub_containors = HashSet::new();

    for (containor, sub_bags) in bags.iter() {
        if sub_bags.contains(&lookup) {
            containors.insert(containor.to_string());
        }
    }

    for containor in &containors {
        for sub_containor in get_containors_of(&bags, containor.to_string()) {
            sub_containors.insert(sub_containor.to_string());
        }
    }
    containors.extend(sub_containors);

    return containors;
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
        let mut result = HashSet::new();

        if raw_result != "no other bags." {
            let second_process: Vec<_> = raw_result.trim().split(",").collect();
            for item in second_process.into_iter() {
                let caps = re.captures(&item.trim()).unwrap();
                let name = caps.name("name").map_or("", |m| m.as_str());
                result.insert(name.to_string());
            }
        }

        bags.insert(source, result);
    }

    let containors = get_containors_of(&bags, "shiny gold".to_string());

    println!("{:?}", containors);
    println!("{:?}", containors.len());
}
