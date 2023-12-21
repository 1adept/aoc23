use core::num;

use crate::util::{Day, Solution};

pub struct Day1;

impl Day for Day1 {
    fn solve(input: &str) -> Solution {
        let (part1, part2) = input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| (part1_line(&line), part2_line(line)))
            .reduce(|(p1, p2), (part1, part2)| (p1 + part1, p2 + part2))
            .expect("Failed to solve");

        (part1, part2)
    }
}

fn part1_line(line: &str) -> usize {
    let numbers: Vec<usize> = line
        .chars()
        .filter(|char| char.is_ascii_digit())
        .map(|char| char.to_string().parse().expect("Expected a number"))
        .collect();
    let first = numbers.first().unwrap_or(&0);
    let last = numbers.last().unwrap_or(&0);
    first * 10 + last
}

fn part2_line(line: &str) -> usize {
    const WORD_NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn of_two_option<F>(a: Option<usize>, b: Option<usize>, f: F) -> Option<usize>
    where
        F: Fn(usize, usize) -> usize,
    {
        match (a, b) {
            (None, None) => None,
            (None, Some(i)) => Some(i),
            (Some(i), None) => Some(i),
            (Some(l), Some(r)) => Some(f(l, r)),
        }
    }

    let res = WORD_NUMBERS
        .iter()
        .enumerate()
        .flat_map(|(index, &word)| {
            let number = index + 1;
            let (number_first, number_last) = {
                let number_string = &number.to_string();
                (line.find(number_string), line.rfind(number_string))
            };
            let word_first = line.find(word);
            let word_last = line.rfind(word);

            let min = of_two_option(number_first, word_first, usize::min).map(|idx| (idx, number));
            let max = of_two_option(number_last, word_last, usize::max).map(|idx| (idx, number));
            vec![min, max]
        })
        .flatten()
        .collect::<Vec<_>>();

    let tens = res
        .iter()
        .min_by_key(|(idx, _num)| idx)
        .map(|(_, number)| number)
        .expect("Expected to find at least one number");
    let ones = res
        .iter()
        .max_by_key(|(idx, _num)| idx)
        .map(|(_, number)| number)
        .expect("Expected to find at least one number");

    println!("In {line} we have ({tens}, {ones})");

    // 55336 too HIGH
    10 * tens + ones
}

#[cfg(test)]
mod tests {
    use crate::util::Day;

    use super::Day1;

    const EXAMPLE1: &str = "\
    1abc2\n\
    pqr3stu8vwx\n\
    a1b2c3d4e5f\n\
    treb7uchet";

    const EXAMPLE2: &str = "\
    two1nine\n\
    eightwothree\n\
    abcone2threexyz\n\
    xtwone3four\n\
    4nineeightseven2\n\
    zoneight234\n\
    7pqrstsixteen";

    #[test]
    fn test_part1() {
        assert_eq!(142, Day1::solve(EXAMPLE1).0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(281, Day1::solve(EXAMPLE2).1);
    }
}
