pub mod stages;

use std::{env, io, process::Command};

fn main() {
    println!("This is malfs - matrix automated linux from scratch");

    let lfs = env::var("LFS").unwrap_or_else(|_| panic!("need env variable LFS!"));

    println!("lfs is {lfs}");
}
