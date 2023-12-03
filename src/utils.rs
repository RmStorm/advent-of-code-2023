use std::io::BufRead;
use std::{fs, io};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    Ok(io::BufReader::new(fs::File::open(filename)?).lines())
}
