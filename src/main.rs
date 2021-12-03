use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process::exit;

fn main() {

}

fn get_oxygen_generator_rating(lines: &Vec<u32>, bits_to_check: u32) -> u32 {
    let bits = bits_to_check-1;
    let mut cloned_lines = lines.clone();
    for i in 0..bits+1 as u32 {
        let most_common_bit = one_is_most_common_bit(&cloned_lines, bits-i);

        let mut v = vec![];
        for cloned_line in cloned_lines {
            let temp = (cloned_line >> (bits-i)) & 0x01;
            if most_common_bit == true && temp == 1{
                v.push(cloned_line)
            } else if most_common_bit == false && temp == 0 {
                v.push(cloned_line)
            }
        }
        cloned_lines = v;
        if cloned_lines.len() == 1 {
            break;
        }
    }
    return cloned_lines[0];
}

fn get_co2_scrubber_rating(lines: &Vec<u32>, bits_to_check: u32) -> u32 {
    let bits = bits_to_check-1;
    let mut cloned_lines = lines.clone();
    for i in 0..bits+1 as u32 {
        let mut most_common_bit = !one_is_most_common_bit(&cloned_lines, bits-i);

        let mut v = vec![];
        for cloned_line in cloned_lines {
            let temp = (cloned_line >> (bits-i)) & 0x01; // Checks if the relevant bit position is set in current value
            if most_common_bit == true && temp == 1{
                v.push(cloned_line)
            } else if most_common_bit == false && temp == 0 {
                v.push(cloned_line)
            }
        }

        cloned_lines = v;
        if cloned_lines.len() == 1 {
            break;
        }
    }
    return cloned_lines[0];
}

fn one_is_most_common_bit(inputs: &Vec<u32>, bit: u32) -> bool {
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

    if zero > one {
        return false;
    }
    return true;
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

        let val = one_is_most_common_bit(&result, 4);

        assert_eq!(val, false)
    }

    #[test]
    fn test_common_value() {
        let result = read_lines("input/test.txt");

        let oxy = get_oxygen_generator_rating(&result, 5);
        assert_eq!(oxy, 23);

        let co2 = get_co2_scrubber_rating(&result, 5);
        assert_eq!(co2, 10)
    }

    #[test]
    fn test_real_value() {
        let result = read_lines("input/input.txt");

        let oxy = get_oxygen_generator_rating(&result, 12);
        let co2 = get_co2_scrubber_rating(&result, 12);

        assert_eq!(oxy, 1327);
        assert_eq!(co2, 3429);
        assert_eq!(oxy * co2, 4550283);
    }
}
