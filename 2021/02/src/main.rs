use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

// Solution for the first task
#[derive(Debug)]
struct Position {
    x: i32,
    depth: i32,
}

impl Position {
    pub fn parse_string(&mut self, input: String) {
        let segments: Vec<&str> = input.split(" ").collect();
        let control = segments[0];
        let value: i32 = segments[1].parse().unwrap();

        match control {
            "forward" => self.x += value,
            "down" => self.depth += value,
            "up" => self.depth -= value,
            _ => {}
        }
    }

    pub fn multiply(&self) -> i32 {
        self.x * self.depth
    }
}

// Solution for the secon task
#[derive(Debug)]
struct Position2 {
    x: i32,
    depth: i32,
    aim: i32,
}

impl Position2 {
    pub fn parse_string(&mut self, input: String) {
        let segments: Vec<&str> = input.split(" ").collect();
        let control = segments[0];
        let value: i32 = segments[1].parse().unwrap();

        match control {
            "forward" => {
                self.x += value;
                self.depth += self.aim * value;
            }
            "down" => self.aim += value,
            "up" => self.aim -= value,
            _ => {}
        }
    }

    pub fn multiply(&self) -> i32 {
        self.x * self.depth
    }
}

fn main() {
    let mut position = Position2 {
        x: 0,
        depth: 0,
        aim: 0,
    };
    let lines = read_lines(("input.txt").to_string());
    for line in lines {
        let line = line.unwrap();
        position.parse_string(line);
    }

    println!("{:?}", position.multiply())
}

