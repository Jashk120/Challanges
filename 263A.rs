use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let iter = input.lines();
    let mut swaps = 0;
    for (row, line) in iter.take(5).enumerate() {
        for (col, value) in line.split_whitespace().enumerate() {
            let value: u8 = value.parse().unwrap();
            if value == 1 {
                let mut c = col.clone();
                let mut r = row.clone();
                while c != 2 || r != 2 {
                    if c < 2 {
                        c += 1;
                        swaps += 1;
                    } else if c > 2 {
                        c -= 1;
                        swaps += 1;
                    }
                    if r > 2 {
                        r -= 1;
                        swaps += 1;
                    } else if r < 2 {
                        r += 1;
                        swaps += 1;
                    }
                }
            }
        }
    }
    println!("{}", swaps)
}
/* Better one after I got to knw about abs_diff
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let iter = input.lines();
    let mut swaps = 0;
    for (row, line) in iter.take(5).enumerate() {
        for (col, value) in line.split_whitespace().enumerate() {
            let value: u8 = value.parse().unwrap();
              if value == 1 {
                 swaps = row.abx_diff(2) + col.abs_diff(2);
            }
        }
    }
    println!("{}", swaps)
}

 */
