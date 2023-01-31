use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn task_a() {
    let lines = read_lines("./input.txt".to_string());
    let mut prev: i32 = -1;
    let mut increases = 0;
    // Iterate over the lines of the file, and in this case print them.
    for line in lines {
        let current: i32 = line.unwrap().parse().unwrap();

        if prev > -1 && current > prev {
            increases += 1;
        }

        prev = current;
    }
    println!("Number of increases: {}", increases);
}

fn task_b() {
    let lines = read_lines("./input.txt".to_string());

    let mut values = vec![];
    let mut increases = 0;
    let mut prev = -1.;
    for line in lines {
        let value: f32 = line.unwrap().parse().unwrap();
        let mut current: f32 = -1.;
        values.insert(0, value);

        if values.len() > 2 {
            current = (values[0] + values[1] + values[2]) / 3.0;
        }

        if current > prev && prev > -1. {
            increases += 1;
        }

        println!("[{}], {}, {}", value, prev, current);

        prev = current;
    }
    println!("Number of increases: {}", increases);
}

fn main() {
    task_b();
}
