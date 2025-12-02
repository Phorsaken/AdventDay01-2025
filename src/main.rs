use std::fs;

const STARTING_POS: i32 = 50;
const MAX_POS: i32 = 99;
const MIN_POS: i32 = 0;

fn main() {
    let turns = fs::read_to_string("assets/puzzle.txt").expect("Unable to read file");

    let mut new_pos = STARTING_POS;
    let mut count = 0;
    for turn in turns.lines() {
        let mut line = String::from(turn);
        let td = line.remove(0);
        let add_count;

        match td {
            'R' => {
                (add_count, new_pos) = turn_r(line.parse::<i32>().unwrap(), new_pos);
                count += add_count;
            },
            'L' => {
                (add_count, new_pos) = turn_l(line.parse::<i32>().unwrap(), new_pos);
                count += add_count;
            },
            _ => println!("Unknown move {}", td),
        }
    }

    println!("{}", count);
}

fn turn_l(num: i32, cur_pos: i32) -> (i32, i32) {
    let mut new_pos = cur_pos - num;
    let mut pass_zero = 0;

    if new_pos < 0 {
        let new_num = -new_pos - 1;
        new_pos = MAX_POS;
        (pass_zero, new_pos) = turn_l(new_num, new_pos);
        if cur_pos != 0 { pass_zero += 1; }
        return (pass_zero, new_pos);
    }

    if new_pos == 0 { pass_zero += 1; }

    (pass_zero, new_pos)
}

fn turn_r(num: i32, cur_pos: i32) -> (i32, i32) {
    let mut new_pos = cur_pos + num;
    let mut pass_zero = 0;

    if new_pos > MAX_POS {
        let new_num = new_pos - (MAX_POS + 1);
        new_pos = MIN_POS;
        (pass_zero, new_pos) = turn_r(new_num, new_pos);
        pass_zero += 1;
        return (pass_zero, new_pos);
    }

    (pass_zero, new_pos)
}