use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let chars: Vec<char> = input.trim().chars().collect();
    let mut output = String::new();
    let mut i: usize = 0;

    while i < chars.len() {
        match chars[i] {
            '.' => {
                output.push('0');
                i += 1
            }
            '-' => {
                if chars[i + 1] == '.' {
                    output.push('1');
                } else if chars[i + 1] == '-' {
                    output.push('2')
                };
                i += 2
            }
            _ => print!("Unknown value"),
        }
    }
    println!("{}", output.trim())
}

/*
 *
 use std::io::{self, Read};
 
 fn main() {
     let mut input = String::new();
     io::stdin().read_to_string(&mut input).unwrap();
 
     let mut bytes = input.trim().bytes();
     let mut out = String::new();
 
     while let Some(b) = bytes.next() {
         out.push(match b {
             b'.' => '0',
             _ => match bytes.next() {
                 Some(b'.') => '1',
                 _ => '2',
             },
         });
     }
     println!("{out}");
 }
 */