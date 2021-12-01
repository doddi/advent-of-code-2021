use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let lines = read_lines();

    let mut count = 0;
    for i in 0..lines.len() - 2 {
        let sum = lines.get(i).unwrap() + lines.get(i+1).unwrap() + lines.get(i+2).unwrap();
        let next_sum = lines.get(i+1).unwrap() + lines.get(i+2).unwrap() + lines.get(i+3).unwrap();

        if sum < next_sum {
            count = count + 1;
        }
    }
    println!("{}", count)
}

fn read_lines() -> Vec<i32> {
    let file = File::open("input.txt").expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    lines.filter_map(io::Result::ok)
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
