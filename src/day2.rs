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
    for line in reader.lines() {
        let line_res = line;
        let parts: Vec<i64> = line_res?
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .flatten()
            .collect();
        if is_good_way(&parts) {
            first_part += 1;
            second_part += 1;
            continue;
        }

        if (0..parts.len()).any(|i| {
            let mut temp = parts.clone();
            temp.remove(i);
            is_good_way(&temp)
        }) {
            second_part += 1;
        }
    }
    println!("Day 2, step 1 - {}", first_part);
    println!("Day 2, step 2 - {}", second_part);
    Ok(())
}

fn is_good_way(vec: &[i64]) -> bool {
    if is_vector_sorted(vec) {
        return vec
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .all(|v| (1..=3).contains(&v.abs()));
    }
    false
}

fn is_vector_sorted(vec: &[i64]) -> bool {
    vec.windows(2).all(|w| w[0] <= w[1]) || vec.windows(2).all(|w| w[0] >= w[1])
}
