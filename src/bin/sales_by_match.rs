/*
https://www.hackerrank.com/challenges/one-month-preparation-kit-sock-merchant/problem
 */

use std::{collections::HashMap, io};
fn main() {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).unwrap();
    let mut count = 0;

    let mut hm: HashMap<i32, i32> = HashMap::new();
    input_buffer.clear();
    io::stdin().read_line(&mut input_buffer).unwrap();
    let arr: Vec<i32> = input_buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for i in 0..arr.len() {
        let entry = hm.entry(arr[i]).or_insert(0);
        *entry += 1;
    }

    for (&_key, &value) in hm.iter() {
        if value == 1 {
            continue;
        }

        if value % 2 == 0 {
            count += value / 2;
            continue;
        }

        count += (value - 1) / 2;
    }
    println!("{}", count);
}
