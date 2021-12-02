use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process::exit;

fn main() {
    let lines = read_lines();

    let tokens = split_lines(lines);

    let position = calculate_aim(&tokens);

    println!("{}", position.0 * position.1)
}

fn calculate_position(coords : &Vec<(String, i32)>) -> (i32, i32) {
    let mut x_pos = 0;
    let mut y_pos = 0;

    for coord in coords {
        match (coord.0.as_str(), coord.1) {
            ("up", step) => {y_pos = y_pos - step}
            ("down", step) => {y_pos = y_pos + step}
            ("forward", step) => {x_pos = x_pos + step}
            ("backward", step) => {x_pos = x_pos - step}
            _ => {
                println!("Error");
                exit(1)
            }
        }
    }

    return (x_pos, y_pos)
}

fn calculate_aim(coords : &Vec<(String, i32)>) -> (i32, i32) {
    let mut aim = 0;
    let mut x_pos = 0;
    let mut y_pos = 0;

    for coord in coords {
        match (coord.0.as_str(), coord.1) {
            ("up", step) => {aim = aim - step}
            ("down", step) => {aim = aim + step}
            ("forward", step) => {
                x_pos = x_pos + step;
                y_pos = y_pos + (aim * step)
            }
            ("backward", step) => {x_pos = x_pos - step}
            _ => {
                println!("Error");
                exit(1)
            }
        }
    }

    return (x_pos, y_pos)
}

fn split_lines(lines: Vec<String>) -> Vec<(String, i32)> {
    let mut input: Vec<(String, i32)> = vec![];
    for line in lines {
        let split = line.split_ascii_whitespace().collect::<Vec<&str>>();
        input.push((split[0].to_ascii_lowercase(), split[1].parse().unwrap()))
    }

    return input;
}

fn read_lines() -> Vec<String> {
    let file = File::open("input.txt").expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    lines.filter_map(io::Result::ok)
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_calculate_position() {
        let coords = vec![
            ("forward".to_ascii_lowercase(), 5),
            ("down".to_ascii_lowercase(), 5),
            ("forward".to_ascii_lowercase(), 8),
            ("up".to_ascii_lowercase(), 3),
            ("down".to_ascii_lowercase(), 8),
            ("forward".to_ascii_lowercase(), 2),
        ];

        let result = calculate_position(&coords);
        let mult = result.0 * result.1;
        assert_eq!(result.0, 15);
        assert_eq!(result.1, 10);
        assert_eq!(mult, 150);
    }

    #[test]
    fn test_calculate_aim() {
        let coords = vec![
            ("forward".to_ascii_lowercase(), 5),
            ("down".to_ascii_lowercase(), 5),
            ("forward".to_ascii_lowercase(), 8),
            ("up".to_ascii_lowercase(), 3),
            ("down".to_ascii_lowercase(), 8),
            ("forward".to_ascii_lowercase(), 2),
        ];

        let result = calculate_aim(&coords);
        let mult = result.0 * result.1;
        assert_eq!(result.0, 15);
        assert_eq!(result.1, 60);
        assert_eq!(mult, 900);
    }
}
