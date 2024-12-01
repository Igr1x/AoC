use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn solve() {
    let result = parse_input_file();
    let mut step1: i64 = 0;
    let mut step2: i64 = 0;
    match result {
        Ok((mut vector1, mut vector2)) => {
            vector1.sort_by(|a, b| a.cmp(b));
            vector2.sort_by(|a, b| a.cmp(b));
            for (index, value) in vector1.iter().enumerate() {
                step1 += (value - vector2[index]).abs();
                step2 += value * vector2.iter().filter(|&n| *n == *value).count() as i64;
            }
            for (index, value) in vector1.iter().enumerate() {}
        }
        _ => {}
    }

    println!("Step 1: {}", step1);
    println!("Step 2: {}", step2);
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
