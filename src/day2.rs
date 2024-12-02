use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn solve() {
    let _ = parse_file();
}

fn parse_file() -> Result<(), io::Error> {
    let file = File::open("resource/input_2.txt")?;
    let reader = BufReader::new(file);
    let mut first_part: i64 = 0;
    let mut second_part: i64 = 0;
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    for line in &lines {
        let line_res = line;
        let parts: Vec<i64> = line_res
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .flatten()
            .collect();
        if is_good_way(&parts) {
            first_part += 1;
        }
        for i in 0..parts.len() {
            if is_good_way(&parts) {
                second_part += 1;
                break;
            }
            let mut new_vec = parts.clone();
            new_vec.remove(i);
            if is_good_way(&new_vec) {
                second_part += 1;
                break;
            }
        }
    }
    println!("Day 2, step 1 - {}", first_part);
    println!("Day 2, step 2 - {}", second_part);
    Ok(())
}

fn is_good_way(vec: &[i64]) -> bool {
    if is_vector_sorted(&vec) {
        let diff_vec: Vec<i64> = vec.windows(2).map(|slice| slice[1] - slice[0]).collect();
        return diff_vec.iter().all(|v| v.abs() <= 3 && v.abs() >= 1);
    }
    false
}

fn is_vector_sorted(vec: &[i64]) -> bool {
    vec.windows(2).all(|w| w[0] <= w[1]) || vec.windows(2).all(|w| w[0] >= w[1])
}
