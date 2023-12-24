use std::{env, println};

use util::{read_input, Day};

mod day1;
mod day2;
mod util;

fn main() {
    let mut args = env::args();
    args.next(); // Discard useless arg

    if let Ok(day) = args
        .next()
        .expect("First argument must be the day (0 < day <= 25)")
        .parse()
    {
        let (part1, part2) = solve(day);
        println!("Solution day {day:>02}; Part1={part1}, Part2={part2}");
    }
}

fn solve(day: u8) -> (usize, usize) {
    if let Ok(input) = &read_input(day) {
        let day: Box<dyn Day> = match day {
            1 => day1::Day1::parse(input),
            2 => day2::Day2::parse(input),
            _ if day > 25 => unreachable!("There are only 25 days!"),
            _ => todo!("Not done!"),
        };
        day.solve()
    } else {
        eprintln!("No input located.");
        std::process::exit(1);
    }
}
