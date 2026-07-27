#![allow(unused, unused_mut)]

use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    for (i, arg) in args.iter().enumerate() {
        println!("Arg {}: {}", i, arg)
    }
}
