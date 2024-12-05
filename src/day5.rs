use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn solve() {
    let _ = parse_file();
}

fn parse_file() -> Result<(), io::Error> {
    let file = File::open("resource/input_5.txt")?;
    let reader = BufReader::new(file);
    let mut map_before: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut map_after: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut rule = true;
    let mut lines: Vec<Vec<i64>> = Vec::new();
    let mut matrix: [[std::cmp::Ordering; 100]; 100] = [[Equal; 100]; 100];
    for line in reader.lines() {
        let current_line = line?;
        if rule {
            if current_line.trim().is_empty() {
                rule = false;
                continue;
            }
            let arr: Vec<&str> = current_line.split("|").collect();
            let first = arr[0].parse::<i64>().unwrap();
            let second = arr[1].parse::<i64>().unwrap();
            matrix[first as usize][second as usize] = Less;
            matrix[second as usize][first as usize] = Greater;
            map_after.entry(first).or_insert_with(Vec::new).push(second);
            map_before
                .entry(second)
                .or_insert_with(Vec::new)
                .push(first);
        } else {
            lines.push(
                current_line
                    .trim()
                    .split(",")
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect(),
            );
        }
    }
    let mut result1 = 0;
    let mut result2 = 0;
    let all_str = lines.len();
    for line in 0..all_str {
        let all_digit = lines[line].len();
        let mut is_valid = true;
        for str in 0..all_digit {
            for prev in 0..str {
                if !map_before
                    .get(&lines[line][str])
                    .unwrap()
                    .contains(&lines[line][prev])
                {
                    is_valid = false;
                    break;
                }
            }
            for next in (str + 1)..all_digit {
                if !map_after
                    .get(&lines[line][str])
                    .unwrap()
                    .contains(&lines[line][next])
                {
                    is_valid = false;
                    break;
                }
            }
        }
        if is_valid {
            let mid = lines[line][all_digit / 2];
            result1 += mid;
        } else {
            let current_line = &mut lines[line];
            current_line.select_nth_unstable_by(all_digit / 2, |&from, &to| {
                matrix[from as usize][to as usize]
            });
            result2 += current_line[all_digit / 2];
        }
    }

    println!("Day 5, part 1 - {}", result1);
    println!("Day 5, part 2 - {}", result2);
    Ok(())
}
