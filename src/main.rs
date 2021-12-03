use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process::exit;

fn main() {

}

fn get_gamma(lines: &Vec<u32>) -> u32 {
    let mut total = 0;
    for i in 0..15 {
        let common_zero = more_zero_bits(&lines, i);
        if !common_zero {
            total = total + (1 << i)
        }
    }
    return total;
}

fn more_zero_bits(inputs: &Vec<u32>, bit: u16) -> bool {
    let mut zero = 0;
    let mut one = 0;

    for input in inputs {
        let val = (input >> (bit)) & 1;
        if val == 1 {
            one = one + 1;
        } else {
            zero = zero + 1;
        }
    }

    return zero > one;
}

fn read_lines(file: &str) -> Vec<u32> {
    let file = File::open(file).expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    let result = lines.filter_map(io::Result::ok)
        .map(|s| u32::from_str_radix(&s, 2).unwrap())
        .collect();

    return result
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_can_read_binary() {
        let result = read_lines("input/input.txt");
        assert_eq!(result[0], 533)
    }

    #[test]
    fn test_count_common_bit() {
        let val = 0x55;
        assert_eq!(val >> 0 & 1, 1);
        assert_eq!(val >> 1 & 1, 0);
        assert_eq!(val >> 2 & 1, 1);
    }

    #[test]
    fn test_common_bits() {
        let result = read_lines("input/test.txt");

        let val = more_zero_bits(&result, 4);

        assert_eq!(val, false)
    }

    #[test]
    fn test_common_value() {
        let result = read_lines("input/test.txt");

        let val = get_gamma(&result);

        assert_eq!(val, 22);
    }

    #[test]
    fn test_value() {
        let results = read_lines("input/test.txt");

        let gamma = get_gamma(&results);
        let val = gamma * (!gamma & 0x1F);
        assert_eq!(val, 198);
    }

    #[test]
    fn test_input_value() {
        let results = read_lines("input/input.txt");

        let gamma = get_gamma(&results);
        let epsilon = !gamma & 0xFFF;
        let val = gamma * epsilon;
        assert_eq!(val, 3633500);
    }
}
