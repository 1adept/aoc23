use day1::Day1;
use util::{read_input, Day};

mod day1;
mod util;

fn main() {
    let day1 = solve(1);
    println!("Day1 = {day1:?}");
}

fn solve(day: u8) -> (usize, usize) {
    let input = &read_input(day).unwrap();
    match day {
        1 => Day1::solve(input),
        _ if day > 25 => unreachable!("There are only 25 days!"),
        _ => todo!("Not done!"),
    }
}
