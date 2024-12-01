use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn solve() {
    let result = parse_input_file();
    match result {
        Ok((mut vector1, mut vector2)) => {
            vector1.sort_unstable();
            vector2.sort_unstable();
            let step1: i64 = calculate_paths(&vector1, &vector2);
            let step2: i64 = calculate_multiplications(&vector1, &vector2);
            println!("Step 1: {}", step1);
            println!("Step 2: {}", step2);
        }
        _ => {}
    }
}

fn calculate_paths(vector1: &[i64], vector2: &[i64]) -> i64 {
    vector1
        .iter()
        .zip(vector2)
        .map(|(v1, v2)| (v1 - v2).abs())
        .sum()
}

fn calculate_multiplications(vector1: &[i64], vector2: &[i64]) -> i64 {
    let counts = count_elements(vector2);
    vector1
        .iter()
        .map(|&v1| v1 * counts.get(&v1).copied().unwrap_or(0))
        .sum()
}

fn count_elements(vec: &[i64]) -> HashMap<i64, i64> {
    let mut counts = HashMap::new();

    for &value in vec {
        *counts.entry(value).or_insert(0) += 1;
    }
    counts
}

fn parse_input_file() -> Result<(Vec<i64>, Vec<i64>), io::Error> {
    let file = File::open("resource/input_1.txt")?;
    let reader = BufReader::new(file);
    let mut vec1: Vec<i64> = Vec::new();
    let mut vec2: Vec<i64> = Vec::new();
    for line in reader.lines() {
        let line_res = line?;
        let mut parts = line_res.split_whitespace().map(|s| s.parse::<i64>());
        match (parts.next(), parts.next()) {
            (Some(Ok(a)), Some(Ok(b))) => {
                vec1.push(a);
                vec2.push(b);
            }
            _ => {}
        }
    }
    Ok((vec1, vec2))
}
