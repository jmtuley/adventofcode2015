use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let total: i32 = stdin.lock().lines().map(|l| l.unwrap()).map(|l| calc(l)).fold(0, |sum, a| sum + a);

    println!("Total is {} square feet.", total)
}

fn calc(l: String) -> i32 {
    let split: Vec<i32> = l.split('x').map(|s| s.parse::<i32>().unwrap()).collect();
    sqfootage(split[0], split[1], split[2])
}

fn sqfootage(x:i32, y:i32, z:i32) -> i32 {
    let side1 = x * y;
    let side2 = y * z;
    let side3 = z * x;
    let slack = vmin(vec![side1, side2, side3]);
    return (2 * side1) + (2 * side2) + (2 * side3) + slack;
}

fn vmin(v:Vec<i32>) -> i32 {
    let mut m = v[0];
    for e in v {
        if e < m {
            m = e;
        }
    }

    m
}

#[test]
fn sqfootage_computes_area_with_slack() {
    assert_eq!(58, sqfootage(2, 3, 4));
    assert_eq!(43, sqfootage(1, 1, 10));
}

#[test]
fn min_works() {
    assert_eq!(1, vmin(vec![1]));
    assert_eq!(1, vmin(vec![2, 1]));
    assert_eq!(1, vmin(vec![1, 2, 3]));
    assert_eq!(1, vmin(vec![2, 1, 3]));
}
