use std::unreachable;

use crate::Day;

pub struct Day2 {
    games: Vec<Game>,
}

/// Type that represents the amount of cubes that are drawn
type DrawType = u8;

/// 3-Tuple of #DrawType
type CubeTriple = (DrawType, DrawType, DrawType);

struct Game {
    id: usize,
    draws: Vec<CubeTriple>,
}

enum Color {
    Blue,
    Red,
    Green,
}

impl Day for Day2 {
    fn parse(text: &str) -> Box<Self>
    where
        Self: Sized,
    {
        fn parse_draw(draw: &str) -> (DrawType, Color) {
            let (amount, color) = draw.split_once(' ').expect("Not a draw");
            let amount = amount.parse::<DrawType>().unwrap();
            let color = match color {
                "red" => Color::Red,
                "blue" => Color::Blue,
                "green" => Color::Green,
                _ => unreachable!("Color {color} doesnt exist (yet)!"),
            };
            (amount, color)
        }

        let games = text
            .lines()
            .enumerate()
            .map(|(index, line)| {
                let colon = line
                    .find(':')
                    .expect("Must have a colon, else its formatted wrong");
                let draws = line[(colon + 1)..]
                    .split(';')
                    .map(|draw| {
                        // example draw: " 3 blue, 4 red"
                        draw.split(',').map(str::trim_start).map(parse_draw).fold(
                            (0, 0, 0),
                            |(r, g, b), (amount, color)| match color {
                                Color::Red => (r + amount, g, b),
                                Color::Green => (r, g + amount, b),
                                Color::Blue => (r, g, b + amount),
                            },
                        )
                    })
                    .collect();
                Game {
                    id: index + 1,
                    draws,
                }
            })
            .collect::<Vec<_>>();

        Box::new(Day2 { games })
    }

    /// Count games that are possible if the bag only contains **12 red**, **13 green** and **14 blue** cubes
    fn solve1(&self) -> usize {
        self.games
            .iter()
            .filter(|game| game.is_possible())
            .map(|game| game.id)
            .sum()
    }

    /// Calculates the power (product of each minimal color-draw) of all possible games
    fn solve2(&self) -> usize {
        self.games
            .iter()
            .map(|game| game.max_cubes_neccessary())
            .map(|(r, g, b)| (r as usize * g as usize * b as usize))
            .sum()
    }
}

/// Checks if each element of the first tuple is smaller or equal its corresponding element in the second tuple
fn triple_lower((a, b, c): CubeTriple, (x, y, z): CubeTriple) -> bool {
    a <= x && b <= y && c <= z
}

impl Game {
    const CAPACITY: CubeTriple = (12, 13, 14);

    /// Returns the is possible of this `Game` by checking if each draw does not exceed the capacity `Game::CAPACITY`
    fn is_possible(&self) -> bool {
        self.draws
            .iter()
            .all(|&draw| triple_lower(draw, Game::CAPACITY))
    }

    /// Returns the min capacity cubes neccessary of this [`Game`].
    fn max_cubes_neccessary(&self) -> CubeTriple {
        self.draws.iter().fold(
            (DrawType::MIN, DrawType::MIN, DrawType::MIN),
            |(max_r, max_g, max_b), (r, g, b)| (max_r.max(*r), max_g.max(*g), max_b.max(*b)),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Day;

    use super::Day2;

    const EXAMPLE: &str = include_str!("../../data/example_02.txt");

    #[test]
    fn test_part1() {
        let day = Day2::parse(EXAMPLE);
        assert_eq!(8, day.solve1());
    }

    #[test]
    fn test_part2() {
        let day = Day2::parse(EXAMPLE);
        assert_eq!(2286, day.solve2());
    }
}
