/*
https://www.hackerrank.com/challenges/one-month-preparation-kit-the-birthday-bar/problem
 */

// [UNSUBMITTED]
use std::io;

fn main() {
    let mut input_buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input_buffer).unwrap();
    input_buffer.clear();
    stdin.read_line(&mut input_buffer).unwrap();
    let s: Vec<i32> = input_buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    input_buffer.clear();
    stdin.read_line(&mut input_buffer).unwrap();
    let settings: Vec<i32> = input_buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut count = 0;
    let d = settings.first().unwrap().clone();
    let mut i = 0;
    let mut j = settings.last().unwrap().clone();

    while (j as usize) <= s.len() {
        if s[(i as usize)..(j as usize)].iter().sum::<i32>() == d {
            count = count + 1
        }
        i = i + 1;
        j = j + 1;
    }

    println!("{}", count);
}
