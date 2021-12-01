// Note: this is my first foray into rust, and the code is garbage.

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| -> i32 { l.expect("Could not parse line").parse::<i32>().unwrap() })
        .collect()
}

fn pt1(data: &Vec<i32>) -> i32 {
    let mut ct = 0;
    for (i, val) in data.iter().enumerate() {
        if i > 0 && val > &data[i - 1] {
            ct += 1;
        }
    }
    return ct;
}

fn pt2(data: &Vec<i32>) -> i32 {
    let mut ct = 0;
    for (i, _val) in data.iter().enumerate() {
        // println!("{:?}", (&data[i..i + 3]).iter().sum::<i32>());
        println!("{:?}", i);
        if i < data.len() - 2
            && i > 0
            && (&data[i - 1..i+2]).iter().sum::<i32>() < (&data[i..i + 3]).iter().sum::<i32>()
        {
            ct += 1;
        }
    }
    return ct;
}

fn main() {
    let data = lines_from_file("input.txt");
    println!("{:?}", pt1(&data));
    println!("{:?}", pt2(&data));
}

// Need to learn how to:
// stride on vectors cleanly
// sum vector subsets cleanly

