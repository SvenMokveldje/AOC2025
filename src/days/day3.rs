use std::{fmt::format, fs::read_to_string};

pub fn day() {
    let input = read_to_string("src/inputs/day3.txt").unwrap();

    let total_result = second(input);

    print!("{:?}\n", total_result);
}

fn first(input: String) -> u32{
    let mut total = 0;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let number: Vec<u32> = chars.iter().filter_map(|c| c.to_digit(10)).map(|d| d as u32).collect();

        let mut largest: u32 = 0;

        for (index, num) in number.iter().enumerate() {
            for another in index + 1..number.len() {
                let other_value = number[another];
                let combined = (num.to_string() + &other_value.to_string()).parse::<u32>().unwrap();

                if combined > largest {
                    largest = combined;
                }
            } 
        }
        println!("{:?}", largest);
        total += largest;

    }
    return total;
}

fn second(input: String) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let number: Vec<u32> = chars.iter().filter_map(|c| c.to_digit(10)).map(|d| d as u32).collect();

        let k = 12;
        let mut result: Vec<u32> = Vec::with_capacity(k);
        let mut start = 0;

        while result.len() < k {
            let remaining = k - result.len();
            let end = number.len() - remaining + 1;
            let slice = &number[start..end];

            let mut max_pos = 0;
            let mut max_digit = slice[0];
            for (i, &d) in slice.iter().enumerate() {
                if d > max_digit {
                    max_digit = d;
                    max_pos = i;
                }
            }

            result.push(max_digit);
            start += max_pos + 1;
        }
        
        let number_str: String = result.iter().map(|d| format!("{}", d)).collect();
        let number_value = number_str.parse::<u64>().unwrap();
        total += number_value;
    }

    return total;
}