use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
mod four {
    pub mod main;
}
fn main() {
    let filename = "./src/four/input.txt";
    four::main::run(string_vector_lines(filename));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn string_vector_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                vec.push(ip);
            }
        }
    }

    return vec;
}
