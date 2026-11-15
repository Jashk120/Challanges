use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let _n: usize = iter.next().unwrap().parse().unwrap();
        let string = iter.next().unwrap();

        let bytes = string.as_bytes();
        if bytes.is_empty() {
            println!("0");
        }

        let mut left = 0;
        let mut right = bytes.len() - 1;

        while left < right {
            if bytes[left] != bytes[right] {
                left += 1;
                right -= 1;
            } else {
                break;
            }
        }
        let remaning = if left > right { 0 } else { right - left + 1 };
        println!("{}", remaning)
    }
}

/*
use std::io::{self, Read};

fn shortest_len(s: &[u8]) -> usize {
    let peeled = s
        .iter()
        .zip(s.iter().rev())
        .take(s.len() / 2)
        .take_while(|(a, b)| a != b)
        .count();
    s.len() - 2 * peeled
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let out: Vec<String> = input
        .split_ascii_whitespace()
        .skip(1) // t
        .collect::<Vec<_>>()
        .chunks_exact(2) // (n, s) pairs
        .map(|pair| shortest_len(pair[1].as_bytes()).to_string())
        .collect();

    println!("{}", out.join("\n"));
}
 */
