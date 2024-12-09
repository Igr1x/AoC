use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn solve() {
    let mut res = parse_file().unwrap();
    let part_one = part_one(&mut res);
    println!("Day 9, part 1 - {:?}", part_one);
}

fn part_one(vec: &mut Vec<i64>) -> usize {
    let mut last = vec.len() - 1;
    let mut i = 0;
    while i <= last {
        if vec[i] == -1 && vec[last] != -1 {
            vec[i] = vec[last];
            vec[last] = -1;
            last -= 1;
            i += 1;
            continue;
        }
        if vec[last] == -1 && vec[i] != -1 {
            i += 1;
            last -= 1;
            continue;
        }
        if vec[last] == -1 && vec[i] == -1 {
            last -= 1;
            continue;
        }
        i += 1;
    }
    return checksum(vec);
}

fn checksum(vec: &Vec<i64>) -> usize {
    vec.iter()
        .filter(|&v| *v != -1)
        .enumerate()
        .map(|(i, &v)| i * usize::try_from(v).unwrap())
        .sum()
}

fn parse_file() -> Result<Vec<i64>, io::Error> {
    let file = File::open("resource/input_9.txt")?;
    let reader = BufReader::new(file);
    let mut result: Vec<i64> = Vec::new();
    let mut id = 0;
    let mut new_str = String::new();
    for line in reader.lines() {
        let current_line = line?;
        new_str.push_str(&current_line);
    }
    let char_arr: Vec<char> = new_str.trim().chars().collect();
    let length = new_str.len() - 1;
    let last_char = char_arr[length];
    for chunk in char_arr.chunks(2) {
        if let [a, b] = chunk {
            let first = a.to_digit(10).unwrap();
            let second = b.to_digit(10).unwrap();
            result.extend(std::iter::repeat(id as i64).take(first as usize));
            result.extend(std::iter::repeat(-1).take(second as usize));
        }
        
            id += 1;
    }
    id -= 1; 
    result.extend(std::iter::repeat(id as i64).take(last_char.to_digit(10).unwrap() as usize));


    Ok(result)
}
