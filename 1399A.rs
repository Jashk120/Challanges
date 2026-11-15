use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut num = input.split_whitespace();
    let t: usize = num.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = num.next().unwrap().parse().unwrap();

        let mut val: Vec<usize> = (0..n)
            .map(|_| num.next().unwrap().parse().unwrap())
            .collect();

        val.sort();
        let mut count = val.len();
        let mut i = 0;
        while i + 1 < val.len() {
            if val[i].abs_diff(val[i + 1]) <= 1 {
                count -= 1
            } else {
                break;
            }
            i += 1;
        }
        if count == 1 {
            println!("YES")
        } else {
            println!("NO")
        }
    }
}

/*
use std::fmt::Write as _;
use std::io::{self, Read};

fn can_reduce(a: &mut [u32]) -> bool {
    a.sort_unstable();
    a.windows(2).all(|w| w[1] - w[0] <= 1)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap());
    let t = tokens.next().unwrap();

    let mut out = String::new();
    for _ in 0..t {
        let n = tokens.next().unwrap() as usize;
        let mut a: Vec<u32> = tokens.by_ref().take(n).collect();
        let ans = if can_reduce(&mut a) { "YES" } else { "NO" };
        writeln!(out, "{ans}").unwrap();
    }
    print!("{out}");
}
 */
