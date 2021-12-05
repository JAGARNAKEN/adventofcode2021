use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
mod three {
    pub mod main;
}
fn main() {
    if let Ok(lines) = read_lines("./src/three/input.txt") {
        three::main::run(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
