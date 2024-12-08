use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn solve() {
    let parse_res = parse_file().unwrap();
    let part_one = part_one(&parse_res.0, parse_res.1, parse_res.2);
    println!("Day 8, part 1 - {}", part_one);
}

fn part_one(map: &HashMap<char, Vec<(i64, i64)>>, height: usize, width: usize) -> i64 {
    let mut unique_antinodes = HashSet::new();
    println!("{:?}", &map);
    for value in map.values() {
        for first in value {
            for second in value {
                if first != second {
                    let y_dist = second.0 - first.0;
                    let x_dist = second.1 - first.1;
                    let f_node = (first.0 + y_dist, first.1 + x_dist);   

                    let s_node = (first.0 - y_dist, first.1 - x_dist);   
                    if f_node.0 >= 0 && f_node.0 < height as i64 
                        && f_node.1 >= 0 && f_node.1 < width as i64 {

                        unique_antinodes.insert(f_node);
                    }
                    if s_node.0 >= 0 && s_node.0 < height as i64 
                        && s_node.1 >= 0 && s_node.1 < width as i64 {
                        unique_antinodes.insert(s_node);
                    }
                }
            }
        }
    }
    return unique_antinodes.len() as i64;
}

fn parse_file() -> Result<(HashMap<char, Vec<(i64, i64)>>, usize, usize), io::Error> {
    let file = File::open("resource/input_8.txt")?;
    let reader = BufReader::new(file);
    let mut result = HashMap::new();
    let mut height = 0;
    let mut width = 0;
    for line in reader.lines() {
        height += 1;
        let current_line = line?;
        width = current_line.len();
        let char_arr: Vec<char> = current_line.chars().collect();
        for i in 0..char_arr.len() {
            let current_char = char_arr[i];
            if current_char != '.' {
                result
                    .entry(current_char)
                    .or_insert_with(Vec::new)
                    .push((height as i64, i as i64));
            }
        }
    }
    Ok((result, height, width))
}
