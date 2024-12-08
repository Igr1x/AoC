use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve() {
    let map = parse_file().unwrap();
    let part_one = part_one(&map);
    println!("Day 7, part 1 - {}", part_one);
    let part_two = part_two(&map);
    println!("Day 7, part 2 - {}", part_two);
}

fn part_one(map: &HashMap<i128, Vec<i128>>) -> i128 {
    let mut result = 0;
    for (key, value) in map.iter() {
        let count_values = value.len();
        let all_perm = generate_permutations(count_values);
        for i in 0..all_perm.len() {
            let mut current_result: i128 = value[0];
            let current_perm: Vec<char> = all_perm[i].chars().collect();
            for j in 0..(count_values - 1) {
                match current_perm[j] {
                    '+' => current_result += value[j + 1],
                    '*' => current_result *= value[j + 1],
                    _ => panic!(),
                }
            }
            if current_result == *key {
                result += key;
                break;
            }
        }
    }
    return result;
}

fn part_two(map: &HashMap<i128, Vec<i128>>) -> i128 {
    let mut result = 0;
    for (key, value) in map.iter() {
        let count_values = value.len();
        let all_perm = generate_permutations_part_two(count_values);
        for i in 0..all_perm.len() {
            let mut current_result: i128 = value[0];
            let current_perm: Vec<char> = all_perm[i].chars().collect();
            for j in 0..(count_values - 1) {
                match current_perm[j] {
                    '+' => current_result += value[j + 1],
                    '*' => current_result *= value[j + 1],
                    '|' => {
                        let left = current_result.to_string();
                        let right = value[j + 1].to_string();
                        let str_result = format!("{}{}", left, right);
                            println!("Left - {} right - {}", left, right);
                        current_result = str_result.parse::<i128>().unwrap();
                    }
                    _ => panic!(),
                }
            }
            if current_result == *key {
                result += key;
                break;
            }
        }
    }
    return result;
}

fn generate_permutations_part_two(count: usize) -> Vec<String> {
    let base_permutations = generate_permutations(count);
    let mut result = base_permutations.clone();

    for line in base_permutations {
        let arr: Vec<char> = line.chars().collect();
        for i in 1..arr.len() {
            let mut new_line = arr.clone();
            new_line[i - 1] = '|';
            result.push(new_line.iter().collect::<String>());
            let mut new_line_after = arr.clone();
            new_line_after[i] = '|';
            result.push(new_line_after.iter().collect::<String>());
        }
    }

    result.dedup();
    result
}

fn generate_permutations(count: usize) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..(2_usize.pow((count - 1).try_into().unwrap())) {
        let mut permutation = String::new();
        for j in 0..(count - 1) {
            if (i & (1 << j)) != 0 {
                permutation.push('*');
            } else {
                permutation.push('+');
            }
        }
        result.push(permutation);
    }
    result
}

fn parse_file() -> Result<HashMap<i128, Vec<i128>>, Box<dyn Error>> {
    let file = File::open("resource/input_7.txt")?;
    let reader = BufReader::new(file);
    let mut result = HashMap::new();
    for line in reader.lines() {
        let current_line = line?;
        let mut split = current_line.split(':');
        let res: i128 = split.next().unwrap().parse()?;
        let digits: Vec<i128> = split
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse::<i128>().ok())
            .collect();
        result.insert(res, digits);
    }
    Ok(result)
}
