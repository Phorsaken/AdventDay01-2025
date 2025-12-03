use std::fs;

fn has_repeating(num: u64) -> bool {
    let snum = num.to_string();
    let num_len = snum.len();

    for pat_len in 1..=(num_len / 2) {
        if num_len % pat_len != 0 {
            continue;
        }
        let pattern = &snum[0..pat_len];
        if snum.chars()
            .collect::<Vec<_>>()
            .chunks(pat_len)
            .all(|chunk| chunk.iter().collect::<String>() == pattern)
        {
            return true
        }
    }

    false
}

fn get_number_value(num: String) -> u64 {
    let parts: Vec<u64> = num.split('-')
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    let mut total: u64 = 0;
    for i in parts[0]..=parts[1] {
        if has_repeating(i) {
            total += i;
        }
    }

    total
}

fn main() {
    let test = fs::read_to_string("assets/puzzleday02.txt").expect("Unable to read file");
    let number_list: Vec<&str> = test.split(',').collect();
    let mut total: u64 = 0;

    for num in number_list {
        total += get_number_value(String::from(num));
    }

    println!("Final code is {}", total);
}