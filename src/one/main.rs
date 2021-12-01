use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    println!("Solving challange one");
    b();
}

fn a() {
    let mut last_value: Option<i32> = None;
    let mut number_of_increases: u32 = 0;
    if let Ok(lines) = read_lines("./src/one/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let current_value = ip.parse::<i32>().unwrap();

                if last_value.is_none() {
                    last_value = Some(current_value);
                    continue;
                }

                if last_value.unwrap() < current_value {
                    number_of_increases += 1;
                }

                last_value = Some(current_value);
            }
        }
    }

    println!("Number of increases {}", number_of_increases);
}

fn b() {
    let mut last_value: Option<i32> = None;
    let mut number_of_increases: u32 = 0;
    let mut vec: VecDeque<i32> = VecDeque::new();
    if let Ok(lines) = read_lines("./src/one/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let current_value = ip.parse::<i32>().unwrap();

                vec.push_back(current_value);

                if vec.len() < 3 {
                    continue;
                } else if vec.len() > 3 {
                    vec.pop_front();
                }

                let vector_sum = calculate_vector(&vec);

                if last_value.is_none() {
                    last_value = Some(vector_sum);
                    continue;
                }

                if last_value.unwrap() < vector_sum {
                    number_of_increases += 1;
                }

                last_value = Some(vector_sum);
            }
        }
    }

    println!("Number of increases {}", number_of_increases);
}

fn calculate_vector(vec: &VecDeque<i32>) -> i32 {
    let mut result: i32 = 0;
    for val in vec {
        result += val;
    }
    return result;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
