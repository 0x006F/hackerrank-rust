/*
https://www.hackerrank.com/challenges/one-month-preparation-kit-plus-minus/problem
 */

 /* [UNSUBMITTED] */

use std::io;

fn solve(arr: &Vec<i32>) {
    let mut p = 0;
    let mut n = 0;
    let mut z = 0;

    for i in arr.iter() {
        match i {
            i if *i < 0 => n = n + 1,
            i if *i > 0 => p = p + 1,
            i if *i == 0 => z = z + 1,
            _ => {}
        }
    }
    println!("{} {} {}", p, n, z);
    println!(
        "{:.6}\n{:.6}\n{:.6}",
        p as f32 / arr.len() as f32,
        n as f32 / arr.len() as f32,
        z as f32 / arr.len() as f32
    )
}

fn main() {
    let mut buf = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().parse::<i32>().unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let arr: Vec<i32> = buf
        .trim()
        .split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    solve(&arr);
}
