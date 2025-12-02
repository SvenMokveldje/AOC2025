use core::num;
use std::fs::read_to_string;

pub fn day() {
    let input = read_to_string("src/inputs/day2.txt").unwrap();

    let total_result = second(input);

    print!("{:?}\n", total_result);
}

fn first(input: String) -> i64 {
    let mut total = 0;
    let split = input.split(',');

    for part in split {
        let number_strings: Vec<&str> = part.split('-').collect();
        let left = number_strings[0].parse::<i64>().unwrap();
        let right = number_strings[1].parse::<i64>().unwrap();

        for number in left..=right {
            let num_str = number.to_string();

            let mid = num_str.len() / 2;

            let (left, right) = num_str.split_at(mid);

            if left == right {
                total += number;
            }
        }
    }

    return total
}

fn second(input: String) -> i64 {
    let mut total = 0;
    let split = input.split(',');

    for part in split {
        let number_strings: Vec<&str> = part.split('-').collect();
        let left = number_strings[0].parse::<i64>().unwrap();
        let right = number_strings[1].parse::<i64>().unwrap();

        for number in left..=right {
            let num_str = number.to_string();
            let length = num_str.len();
            let mut invalid = false;

            for n in 1..=length/2 {
                if length % n != 0 {
                    continue;
                }

                let repeats = length / n;
                if repeats < 2 {
                    continue;
                }

                let unit = &num_str[0..n];
                let reconstructed = unit.repeat(repeats);

                if reconstructed == num_str {
                    invalid = true;
                    break;
                }
            }

            if invalid {
                total += number;
            }
        
        }
    }

    return total
}