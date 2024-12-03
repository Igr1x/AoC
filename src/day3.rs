use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufRead, Seek, SeekFrom};

pub fn solve() -> Result<(), io::Error> {
    let file = File::open("resource/input_3.txt")?;
    let mut reader = BufReader::new(file);
    let matcher = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut res = Vec::new();
    for line in reader.by_ref().lines() {
        let current_line = line?;
        for captures in matcher.captures_iter(&current_line) {
            let num1 = captures[1].parse::<i64>().unwrap();
            let num2 = captures[2].parse::<i64>().unwrap();
            res.push(num1 * num2);
        }
    }
    let sum: i64 = res.iter().sum();
    println!("Day 3, part 1 - {}", sum);

    let matcher2 = Regex::new(r"(mul\(\d+,\d+\)|don't\(\)|do\(\))").unwrap();
    let mut res2 = Vec::new();
    reader.seek(SeekFrom::Start(0))?;
    for line in reader.by_ref().lines() {
        let current_line = line?;
        for captures in matcher2.captures_iter(&current_line) {
            res2.push(captures[0].to_string());
        }
    }
    let mut sum2 = 0;
    let mut can_mul = true;
    let digit_matcher = Regex::new(r"(\d+),(\d+)").unwrap();
    for i in 0..res2.len() {
        if res2[i].eq("do()") {
            can_mul = true;
            continue;
        }
        if can_mul == false {
            continue;
        }
        if res2[i].eq("don't()") {
            can_mul = false;
            continue;
        }
        if can_mul == true {
            for captures in digit_matcher.captures_iter(&(res2[i])) {
                let d1 = captures[1].parse::<i64>().unwrap();
                let d2 = captures[2].parse::<i64>().unwrap();
                sum2 += d1 * d2;
               } 
        }
    }
    println!("Day 3, part 2 - {}", sum2);
    Ok(())
}
