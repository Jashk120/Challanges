use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut num: Vec<usize> = input
        .split("+")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();
    num.sort();

    for (index, n) in num.iter().enumerate() {
        if index == (num.len() - 1){
            print!("{}", n);
        }else{
        print!("{}+", n);
        }
        }
}

/*
 * Better Alternative
 * use std::io::{self, Read};
 
 fn main() {
     let mut input = String::new();
     io::stdin().read_to_string(&mut input).unwrap();
 
     let mut num: Vec<usize> = input
         .split('+')
         .map(|x| x.trim().parse::<usize>().unwrap())
         .collect();
     num.sort();
 
     for (index, n) in num.iter().enumerate() {
         if index > 0 {
             print!("+");
         }
         print!("{}", n);
         
         }
 }
 */