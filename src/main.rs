#![warn(missing_docs)]

//! Uhhhh add this later lmaooooo

mod table;

use table::*;

fn main() {
    println!("{}", generate_table(3, &mut vec![vec![0; 2]; 2_i32.pow(3) as usize], vec!["a", "b", "c", "d", "sel2", "sel1", "out2", "out1"]))
}