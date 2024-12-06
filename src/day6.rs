use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use Direction::*;

pub fn solve() {
    let vec = parse_file().unwrap();
    let part_1 = part_one(vec.clone());
    println!("{}", part_1);
}

fn part_one(mut vec: Vec<Vec<char>>) -> usize {
    let start = find_start(&vec);
    let x_start = start.1;
    let y_start = start.0;
    let mut current_state = Direction::UP;
    let mut x = x_start;
    let mut y = y_start;
    loop {
        let mut next_x = x;
        let mut next_y = y;
        match current_state {
            UP => {
                next_y -= 1;
            }
            DOWN => {
                next_y += 1;
            }
            RIGHT => {
                next_x += 1;
            }
            LEFT => {
                next_x -= 1;
            }
        }
        let next_element = get_next_element(&mut vec, next_x, next_y);
        match next_element {
            Some(c) => {
                if c == '#' {
                    update_pos(&mut current_state);
                } else {
                                    vec[y][x] = 'X';
                    x = next_x;
                    y = next_y;
                }
            }
            None => {
                vec[y][x] = 'X';
                break;
            }
        }
    }
    vec.iter().flatten().filter(|&&v| v == 'X').count()
}

fn update_pos(state: &mut Direction) {
    *state = match state {
        UP => Direction::RIGHT,
        RIGHT => Direction::DOWN,
        DOWN => Direction::LEFT,
        LEFT => Direction::UP,
    }
}

fn get_next_element(vec: &Vec<Vec<char>>, x: usize, y: usize) -> Option<char> {
    vec.get(y as usize)
        .and_then(|row| row.get(x as usize))
        .copied()
}

enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

fn find_start(vec: &Vec<Vec<char>>) -> (usize, usize) {
    let count_lines = vec.len();
    for i in 0..count_lines {
        let current_size = vec[i].len();
        for j in 0..current_size {
            if vec[i][j] == '^' {
                return (i, j);
            }
        }
    }
    panic!("Not found")
}

fn parse_file() -> Result<Vec<Vec<char>>, io::Error> {
    let file = File::open("resource/input_6.txt")?;
    let reader = BufReader::new(file);
    let mut result_vec: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let current_line = line?;
        result_vec.push(current_line.chars().collect());
    }
    Ok(result_vec)
}
