use std::fs;

fn main() {
    let nums = fs::read_to_string("assets/puzzleday03.txt").expect("Unable to read file.");
    let mut total: u32 = 0;

    for num in nums.lines() {
        let mut int_array: Vec<u32> = num.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let last_digit: u32 = int_array.pop().unwrap();
        let max_val: u32 = int_array.iter().max().unwrap().clone();

        let mut second_digit: u32 = 0;
        let mut found: bool = false;
        let mut final_num: String = String::new();
        for x in int_array {
            if x == max_val && !found {
                final_num = x.to_string();
                found = true;
                continue;
            }

            if !found {
                continue;
            }

            if x == 9 {
                second_digit = 9;
                break;
            }

            if x > second_digit {
                second_digit = x;
            }
        }

        if last_digit > second_digit {
            final_num.push_str(&last_digit.to_string());
        }
        else {
            final_num.push_str(&second_digit.to_string());
        }

        total += final_num.parse::<u32>().unwrap();
    }

    println!("{}", total);
}