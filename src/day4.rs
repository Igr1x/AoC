use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const MATRIX_P1: [[i32; 2]; 8] = [
    [1, 0],
    [-1, 0],
    [0, 1],
    [0, -1],
    [1, 1],
    [-1, 1],
    [1, -1],
    [-1, -1],
];

pub fn solve() {
    let parse_res = parse_file().unwrap();
    let high = parse_res.len() as i32;
    let width = parse_res[0].len() as i32;
    let mut res_p1 = 0;
    let mut res_p2 = 0;
    for i in 0..high {
        for j in 0..width {
            for y in MATRIX_P1.iter() {
                if get_value(&parse_res, i, j) == Some('X')
                    && get_value(&parse_res, i + y[0], j + y[1]) == Some('M')
                    && get_value(&parse_res, i + y[0] * 2, j + y[1] * 2) == Some('A')
                    && get_value(&parse_res, i + y[0] * 3, j + y[1] * 3) == Some('S')
                {
                    res_p1 += 1;
                }
            }
        }
    }
    for i in 1..(high - 1) as usize {
        for j in 1..(width - 1) as usize {
            if parse_res[i][j] == 'A' {
                let ul = parse_res[i + 1][j - 1];
                let ur = parse_res[i + 1][j + 1];
                let ll = parse_res[i - 1][j - 1];
                let rl = parse_res[i - 1][j + 1];
                if [ul, ur, ll, rl].iter().all(|&ch| ch == 'M' || ch == 'S') {
                    if ul == ur && ll == rl && ul != ll || ul == ll && ur == rl && ul != ur {
                        res_p2 += 1;
                    }
                }
            }
        }
    }
    println!("Day 4, part 1 - {}", res_p1);
    println!("Day 4, part 2 - {}", res_p2);
}

fn get_value(matrix: &[Vec<char>], i: i32, j: i32) -> Option<char> {
    matrix
        .get(i as usize)
        .and_then(|row| row.get(j as usize))
        .copied()
}

fn parse_file() -> Result<Vec<Vec<char>>, io::Error> {
    let file = File::open("resource/input_4.txt")?;
    let reader = BufReader::new(file);
    let mut parse_res: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        parse_res.push(line?.chars().collect());
    }
    Ok(parse_res)
}
