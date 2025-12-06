use core::{num, str};
use std::fs::read_to_string;

use regex::Regex;

pub fn day() {
    let input = read_to_string("src/inputs/day6.txt").unwrap();

    let parsed_input = parse_columns(input);
    print!("{:?}", parsed_input);
    let total_result = second(parsed_input);

    print!("{:?}\n", total_result);
}

fn parse_columns(input: String) -> Vec<Vec<String>> {
    let lines: Vec<&str> = input.lines().collect();
    
    if lines.is_empty() {
        return vec![];
    }
    
    let last_line = lines[lines.len() - 1];
    let mut col_ends = Vec::new();
    
    let chars: Vec<char> = last_line.chars().collect();
    for i in 0..chars.len() {
        if chars[i] != ' ' {
            let mut end = i + 1;
            while end < chars.len() && chars[end] == ' ' {
                end += 1;
            }
            col_ends.push(end);
        }
    }
    
    let mut result = Vec::new();
    for line in &lines {
        let chars: Vec<char> = line.chars().collect();
        let mut row = Vec::new();
        let mut start = 0;
        
        for &end in &col_ends {
            let act_end = end.min(chars.len());
            let cell: String = chars[start..act_end].iter().collect();
            row.push(cell);
            start = act_end;
        }
        
        result.push(row);
    }
    
    result
}





fn first(parsed: Vec<Vec<String>>) -> i64{
    let rows = parsed.len();

    let mut number_rows: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    let last_row = rows - 1;

    let mut total: i64 = 0;
    
    for op in &parsed[last_row] {
        operators.push(op.to_string());
    }

    for r in 0..last_row {
        let nums: Vec<i64> = parsed[r]
            .iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        number_rows.push(nums);
    }

    let cols = number_rows[0].len();
    let mut parsed_rows: Vec<Vec<i64>> = vec![Vec::new(); cols];

    for row in number_rows {
        for (c, &num) in row.iter().enumerate() {
            parsed_rows[c].push(num);
        }
    }

    for (index, par) in parsed_rows.iter().enumerate() {
        let op = operators[index].clone();
        let mut small_total: i64 = 0;
        if op == "+" {
            small_total = 0;
            for num in par {
                small_total += num;
            }
        } else if op == "*" {
            small_total = 1;
            for num in par {
                small_total *= num;
            }
        }
        
        total += small_total;
    }

    return total
}



fn second(parsed: Vec<Vec<String>>) -> i64 {
    let rows = parsed.len();
    let cols = parsed[0].len();
    
    let mut operators: Vec<String> = Vec::new();
    for op in &parsed[rows - 1] {
        operators.push(op.trim().to_string());
    }
    
    let mut columns: Vec<Vec<String>> = vec![Vec::new(); cols];
    for row in &parsed {
        for (c, num) in row.iter().enumerate() {
            columns[c].push(num.clone());
        }
    }
    
    let mut total: i64 = 0;
 
    for (i, column) in columns.iter().enumerate().rev() {
        let operator = &operators[i];
        
     
        if operator.trim().is_empty() {
            continue;
        }
        
        let number_cells = &column[..column.len() - 1];
        
     
        let max_len = number_cells.iter().map(|s| s.len()).max().unwrap_or(0);
        
        let mut numbers: Vec<i64> = Vec::new();
     
        for char_pos in (0..max_len).rev() {
            let mut num_str = String::new();
         
            for row in 0..number_cells.len() {
                let cell = &number_cells[row];
                if let Some(ch) = cell.chars().nth(char_pos) {
                    if ch != ' ' {
                        num_str.push(ch);
                    }
                }
            }
            
            if !num_str.is_empty() {
                if let Ok(num) = num_str.parse::<i64>() {
                    numbers.push(num);
                }
            }
        }

        if !numbers.is_empty() {
            let result = match operator.as_str() {
                "+" => numbers.iter().sum(),
                "*" => numbers.iter().product(),
                _ => 0,
            };
            
            total += result;
        }
    }
    
    total
}
