use std::fs;
use std::str;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    return result;
}

fn generate_cols(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in lines {
        let parts: Vec<i32> = line.split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
        let first: i32 = *parts.first().unwrap();
        let last: i32 = *parts.last().unwrap();
        col1.push(first);
        col2.push(last);
    }
    
    col1.sort();
    col2.sort();
    result.push(col1);
    result.push(col2);

    return result;
}

fn get_total_distance(values: &Vec<Vec<i32>>) -> i32 {
    let mut total: i32 = 0;
    for i in 0..values[0].len() {
        if values[0][i] > values[1][i] {
            total += values[0][i] - values[1][i];
        } else {
            total += values[1][i] - values[0][i];
        }
    }
    return total;
}

fn main() {
    let file_conent = read_lines("./input_debug");

    // println!("contents: \n{:#?}", file_conent);
    let cols: Vec<Vec<i32>> = generate_cols(&file_conent);

    // println!("Columns: \n{:#?}", cols);

    let total = get_total_distance(&cols);

    println!("total: {total}");


}
