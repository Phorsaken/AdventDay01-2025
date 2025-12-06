use std::fs;
use num_bigint::BigUint;

fn get_max_number(mut num: BigUint, shrink: u64) -> BigUint {
    for _ in 0..shrink {
        let mut ans = BigUint::from(0u32);
        let mut i = BigUint::from(1u32);
        let ten = BigUint::from(10u32);

        while &num / &i > BigUint::from(0u32) {
            let temp = (&num / (&i * &ten)) * &i + (&num % &i);
            i *= &ten;

            ans = ans.max(temp);
        }

        num = ans;
    }

    num
}

fn main() {
    let nums = fs::read_to_string("assets/puzzleday03.txt").expect("Unable to read file.");
    let mut total = BigUint::from(0u32);

    for num in nums.lines() {
        let shrink = num.len() as u64 - 12;
        let parsed_num: BigUint = num.parse().unwrap();

        total += get_max_number(parsed_num, shrink)
    }

    println!("{}", total)
}