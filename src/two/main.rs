use std::fs::File;
use std::io;

struct Position {
    x: i32,
    y: i32,
}

pub fn run(lines: io::Lines<io::BufReader<File>>) {
    b(lines);
}

fn a(lines: io::Lines<io::BufReader<File>>) {
    let mut pos = Position { x: 0, y: 0 };
    for line in lines {
        if let Ok(ip) = line {
            let mut split_line = ip.split(" ");
            let command = split_line.next().unwrap();
            let times = split_line.next().unwrap().parse::<i32>().unwrap();
            match command {
                "forward" => pos.y += times,
                "up" => pos.x -= times,
                "down" => pos.x += times,
                _ => println!("Nothing"),
            }
        }
    }

    println!("{:?}", pos.x * pos.y);
}

struct Position2 {
    x: i32,
    y: i32,
    aim: i32,
}

impl Position2 {
    fn forward(&mut self, times: i32) {
        self.y += times;
        self.x += times * self.aim;
    }
}

fn b(lines: io::Lines<io::BufReader<File>>) {
    let mut pos = Position2 { x: 0, y: 0, aim: 0 };
    for line in lines {
        if let Ok(ip) = line {
            let mut split_line = ip.split(" ");
            let command = split_line.next().unwrap();
            let times = split_line.next().unwrap().parse::<i32>().unwrap();
            match command {
                "forward" => pos.forward(times),
                "up" => pos.aim -= times,
                "down" => pos.aim += times,
                _ => println!("Nothing"),
            }
        }
    }

    println!("{:?}", pos.x * pos.y);
}
