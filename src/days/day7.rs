use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string, io::Empty};

pub fn day() {
    let input = read_to_string("src/inputs/day7.txt").unwrap();

    let parsed_input = parse_input(input);
    
    let total_result = second(parsed_input);

    print!("{:?}\n", total_result);
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let parsed: Vec<Vec<char>> = input.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    return parsed;
}

fn find_start_col(first_row: Vec<char>) -> i64 {
    let mut col: i64 = 0;

    for (num, col_char) in first_row.iter().enumerate() {
        if *col_char == 'S' {
            col = num as i64;
        }
    }

    return col;
}

fn find_splitters(row: Vec<char>, beam_cols: &HashSet<i64>) -> Vec<i64> {
    let mut split_cols: Vec<i64> = Vec::new();
    
    for (col, char) in row.iter().enumerate() {
        if *char == '^' {
            if beam_cols.contains(&(col as i64)) {
                split_cols.push(col as i64);
            }
        }
    }

    return  split_cols;
}

fn first(input: Vec<Vec<char>>) -> i64 {
    let start_col = find_start_col(input[0].clone());
    
    let mut beam_cols: HashSet<i64> = HashSet::from([start_col]);
    let mut total: i64 = 0;
    for row in 1..input.len() {
        let split_cols = find_splitters(input[row].clone(), &beam_cols);
        print!("{:?}\n", beam_cols);
        if split_cols.is_empty() {
            continue;
        }

        for col in split_cols {
            beam_cols.remove(&col);
            total += 1;
            if col - 1 >= 0 {
                beam_cols.insert(col - 1);
                
            } 
            if col + 1 <= input[0].len().try_into().unwrap() {
                beam_cols.insert(col + 1);
            }
            
        }

    }

    return total;
}

fn new_find_splitters(row: Vec<char>, beam_col: i64) -> Option<i64> {
    for (col, ch) in row.iter().enumerate() {
        if *ch == '^' && beam_col == col as i64 {
            return Some(col as i64);
        }
    }
    None
}

fn second(input: Vec<Vec<char>>) -> i64 {
    let rows = input.len();
    let cols = input[0].len();
    let start_col = find_start_col(input[0].clone());

    let mut timeline_count = vec![vec![0i64; cols]; rows];
    timeline_count[0][start_col as usize] = 1;

    for row in 0..rows - 1 {
        for col in 0..cols {
            let count = timeline_count[row][col];
            if count == 0 {
                continue;
            }

            if input[row][col] == '^' {
                if col > 0 {
                    timeline_count[row + 1][col - 1] += count;
                }
                if col + 1 < cols {
                    timeline_count[row + 1][col + 1] += count;
                }
            } else {
                timeline_count[row + 1][col] += count;
            }
        }
    }

    timeline_count[rows - 1].iter().sum()
}