use std::{collections::HashSet, fs::read_to_string};

struct Input {
    ranges: Vec<(i64, i64)>,
    ingredients: Vec<i64>,
}

pub fn day() {
    let input = read_to_string("src/inputs/day5.txt").unwrap();

    let parsed_input = parse_input(input);

    let total_result = second(parsed_input);

    print!("{:?}\n", total_result);
}

fn parse_input(input: String) -> Input{
    let mut after = false;

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut ingredients: Vec<i64> = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            after = true;
            continue;
        }
        
        if !after {
            let parts: Vec<i64> = line.split('-')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();

            ranges.push((parts[0], parts[1]));
        } else {
            let num: i64 = line.trim().parse::<i64>().unwrap();
            ingredients.push(num);
        }
    }

    return Input{ ranges: ranges, ingredients: ingredients};
}

fn first(input: Input) -> i64 {
    let mut total = 0;

    for ing in input.ingredients {
        let mut fresh = false;
        
        for range in &input.ranges {
            if ing >= range.0 && ing <= range.1 {
                fresh = true;
                break;
            }
        }

        if fresh {
            total += 1;
        }
    }

    return total
}

fn second(input: Input) -> i64{
    let mut total: HashSet<i64> = HashSet::new();

    let mut ranges = input.ranges.clone();
    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(i64, i64)> = Vec::new();
    
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }
    

    merged.iter()
        .map(|(s, e)| e- s + 1 )
            .sum()
}