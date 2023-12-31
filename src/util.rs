use std::{fs, path::Path};

pub type Solution = (usize, usize);

pub trait Day {
    fn parse(text: &str) -> Box<Self>
    where
        Self: Sized;
    fn solve1(&self) -> usize;
    fn solve2(&self) -> usize;
    fn solve(&self) -> (usize, usize) {
        (self.solve1(), self.solve2())
    }
}

///
/// Read data for a specific day
/// ---
/// It is assumed that the data file is formatted as input_dd.txt
/// # Errors
///
/// This function will return an error if .
pub fn read_input(day: u8) -> Result<String, std::io::Error> {
    const BASE_PATH: &str = "../data";
    let path = format!("{BASE_PATH}/input_{day:02}.txt");
    let path = Path::new(&path);
    println!("{path:?}");
    fs::read_to_string(path)
}
