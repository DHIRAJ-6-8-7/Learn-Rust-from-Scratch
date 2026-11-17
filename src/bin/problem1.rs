use std::collections::HashMap;
use std::io::{self, Write};

fn calculate_median_and_mode(list: &[i32]) -> (f64, i32) {
    if list.is_empty() {
        panic!("Empty list provided");
    }

    let mut sorted = list.to_vec();
    sorted.sort();
    let mid = sorted.len() / 2;
    let median = if sorted.len() % 2 == 0 {
        (sorted[mid - 1] as f64 + sorted[mid] as f64) / 2.0
    } else {
        sorted[mid] as f64
    };

    let mut counts = HashMap::new();
    for &num in list {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mode = counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .unwrap_or(0);

    (median, mode)
}

fn to_pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut words = Vec::new();

    for word in text.split_whitespace() {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            if vowels.contains(&first) {
                words.push(format!("{}-hay", word));
            } else {
                let rest: String = chars.collect();
                words.push(format!("{}-{}-ay", rest, first));
            }
        }
    }

    words.join(" ")
}

fn run_company_directory() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("\nCompany Directory CLI");
    println!("Commands:");
    println!("  add [Name] to [Department]");
    println!("  list [Department]");
    println!("  list all");
    println!("  quit");
    println!("-----------------------------");

    loop {
        print!("\ncompany> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            break;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        if tokens[0].eq_ignore_ascii_case("add") && tokens.len() >= 4 {
            let name = tokens[1].to_string();
            let dept = tokens[3..].join(" ");

            company.entry(dept.clone()).or_default().push(name.clone());
            println!("Added {} to {}.", name, dept);
        } else if tokens[0].eq_ignore_ascii_case("list")
            && tokens.len() == 2
            && tokens[1].eq_ignore_ascii_case("all")
        {
            if company.is_empty() {
                println!("Directory is empty.");
                continue;
            }

            println!("\nFull Roster:");
            let mut depts: Vec<&String> = company.keys().collect();
            depts.sort();

            for dept in depts {
                if let Some(employees) = company.get(dept) {
                    let mut sorted_emp = employees.clone();
                    sorted_emp.sort();
                    println!("{}: {:?}", dept, sorted_emp);
                }
            }
        } else if tokens[0].eq_ignore_ascii_case("list") && tokens.len() >= 2 {
            let dept = tokens[1..].join(" ");
            if let Some(employees) = company.get(&dept) {
                let mut sorted_emp = employees.clone();
                sorted_emp.sort();
                println!("{}: {:?}", dept, sorted_emp);
            } else {
                println!("Department '{}' not found.", dept);
            }
        } else {
            println!("Invalid command.");
        }
    }
}

fn main() {
    println!("Chapter 8 Exercises Demo\n");

    let numbers = vec![4, 2, 9, 2, 5, 6, 2, 8, 1, 9];
    let (median, mode) = calculate_median_and_mode(&numbers);
    println!("List: {:?}", numbers);
    println!("Median: {}", median);
    println!("Mode: {}\n", mode);

    let text = "first apple hello world";
    let pig = to_pig_latin(text);
    println!("Original: {}", text);
    println!("Pig Latin: {}\n", pig);

    run_company_directory();

    println!("\nGoodbye!");
}
