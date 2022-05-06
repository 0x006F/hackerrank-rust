/*
https://www.hackerrank.com/challenges/minimum-distances/problem
*/
use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let t = buffer.trim().parse::<i32>().unwrap();

    for _i in 0..t {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let arr: Vec<i32> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let reversed: Vec<i32> = arr.clone().into_iter().rev().collect();
    }
}
