use std::fs::File;
use std::io;

pub fn run(lines: io::Lines<io::BufReader<File>>) {
    b(lines);
}

fn a(lines: io::Lines<io::BufReader<File>>) {
    let mut number_of_entries = 0;
    let mut vec = Vec::new();

    for line in lines {
        if let Ok(ip) = line {
            for (i, c) in ip.chars().enumerate() {
                let val = c.to_digit(2).unwrap();
                if vec.len() < (i + 1) {
                    vec.push(val);
                } else {
                    vec[i] += val;
                }
            }
            number_of_entries += 1;
        }
    }

    let mut gamma_string = String::new();
    let mut epsilon_string = String::new();
    let threshold = number_of_entries / 2;
    for sum in vec {
        if sum > threshold {
            gamma_string.push_str("1");
            epsilon_string.push_str("0");
        } else {
            gamma_string.push_str("0");
            epsilon_string.push_str("1");
        }
    }

    let gamma = isize::from_str_radix(gamma_string.as_str(), 2).unwrap();
    let epsilon = isize::from_str_radix(epsilon_string.as_str(), 2).unwrap();
    let sum = gamma * epsilon;
    println!("{:?}", sum);
}

enum COMPARATOR {
    OXYGEN,
    C02,
}

fn b(lines: io::Lines<io::BufReader<File>>) {
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in lines {
        if let Ok(ip) = line {
            data.push(ip.chars().collect());
        }
    }

    let last_number_larger = divider_ref(&data, 0, COMPARATOR::OXYGEN);
    let last_number_smaller = divider(data, 0, COMPARATOR::C02);

    let ocxygen_string: String = last_number_larger.into_iter().collect();
    let oxygen = isize::from_str_radix(ocxygen_string.as_str(), 2).unwrap();
    let c02_string: String = last_number_smaller.into_iter().collect();
    let c02 = isize::from_str_radix(c02_string.as_str(), 2).unwrap();

    let sum = oxygen * c02;
    println!("{:?}", sum);
}
fn divider_ref(data: &Vec<Vec<char>>, index: usize, comparator: COMPARATOR) -> Vec<char> {
    let data_copy = data.to_vec();
    divider(data_copy, index, comparator)
}

fn divider(data: Vec<Vec<char>>, index: usize, comparator: COMPARATOR) -> Vec<char> {
    if data.len() < 2 {
        return data[0].clone();
    }

    let mut zero_vector = Vec::new();
    let mut one_vector = Vec::new();
    for num in data {
        if num[index] == '0' {
            zero_vector.push(num.clone());
        } else {
            one_vector.push(num.clone());
        }
    }

    let new_index = index + 1;
    if matches!(comparator, COMPARATOR::OXYGEN) {
        if zero_vector.len() <= one_vector.len() {
            return divider(one_vector, new_index, comparator);
        } else {
            return divider(zero_vector, new_index, comparator);
        }
    } else {
        if zero_vector.len() <= one_vector.len() {
            return divider(zero_vector, new_index, comparator);
        } else {
            return divider(one_vector, new_index, comparator);
        }
    }
}
