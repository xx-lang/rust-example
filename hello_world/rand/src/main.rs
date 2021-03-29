extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guest number");

    let num = rand::thread_rng().gen_range(1,100);

    println!("The rand number is {}", num);

    println!("Please enter your guest");

    let mut guest = String::new();
    io::stdin().read_line(&mut guest).expect("failed to guest");
    println!("Your guest is {}", guest);
}
