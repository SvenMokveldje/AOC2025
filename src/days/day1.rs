use core::num;
use std::fs::read_to_string;

pub fn day() {
    let mut input = Vec::new();

    for line in read_to_string("src/inputs/day1.txt").unwrap().lines() {
        input.push(line.to_string());
    }

    let total_result = first(input);

    print!("{:?}\n", total_result);
}

fn first(input: Vec<String>) -> i32 {
    let mut initial = 50;
    let mut total = 0;

    for i in input {
        let letter = i.chars().next().unwrap();
        let number: i32 = i[1..].parse().unwrap();

        let old = initial; 

        if letter == 'L' {
            if old == 0 {
                total += number / 100;
            } else {
                total += (number + (100 - old)) / 100;
            }

            initial = (old - number).rem_euclid(100);
        }

        if letter == 'R' {
            total += (old + number) / 100;

            initial = (old + number).rem_euclid(100);
        }
    }

    total
}


fn second() {

}