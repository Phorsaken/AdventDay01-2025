// This program is my solve for part one. It will not work for part two as the only math way of doing it will cause me to just loop through the number ranges anyway.
use std::collections::HashSet;
use std::fs;

fn get_length(n: u64) -> u64 {
    (n.checked_ilog10().unwrap_or(0) + 1) as u64
}

fn get_multi(length: u64) -> u64 {
    10_u64.pow(length as u32) + 1
}

fn make_list() -> HashSet<u64> {
    let mut has_num: HashSet<u64> = HashSet::new();

    for i in 1_u64..=1000000_u64 {
        has_num.insert(i * get_multi(get_length(i)));
    }

    has_num
}

fn get_number_list(num: String) -> HashSet<u64> {
    let parts: Vec<u64> = num.split('-')
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    let mut number_list: HashSet<u64> = HashSet::new();
    for i in parts[0]..=parts[1] {
        number_list.insert(i);
    }

    number_list
}

fn main() {
    let num_list = make_list();
    let test = fs::read_to_string("assets/puzzleday02.txt").expect("Unable to read file");
    let number_list: Vec<&str> = test.split(',').collect();
    let mut total = 0;

    for num in number_list {
        let numbers = get_number_list(String::from(num));
        let count: u64 = numbers.intersection(&num_list).copied().sum();
        total += count;
    }

    println!("Final total: {}", total);
}
