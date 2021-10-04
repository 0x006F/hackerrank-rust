/*
https://www.hackerrank.com/challenges/one-month-preparation-kit-two-arrays/problem
 */

use std::io::{self, BufRead};

fn solve(a: &mut Vec<i32>, b: &mut Vec<i32>, k: i32) {
    if a.len() != b.len() {
        return;
    }
    a.sort();
    b.sort_by(|a, b| b.cmp(a));

    for i in 0..a.len() {
        if a[i] + b[i] < k {
            println!("NO");
            return;
        }
    }
    println!("YES");
}

fn main() {
    let mut t_buf = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut t_buf).unwrap();

    for _ in 0..t_buf.trim().parse::<i32>().unwrap() {
        t_buf.clear();
        stdin.lock().read_line(&mut t_buf).unwrap();
        let k = t_buf
            .trim()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let mut a: Vec<i32> = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut b: Vec<i32> = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        solve(&mut a, &mut b, k);
    }
}
