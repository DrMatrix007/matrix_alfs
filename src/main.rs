pub mod commands;
pub mod stages;

use std::env;

use stages::{stage2::Stage2, StageRunner};

fn main() {
    println!("This is malfs - matrix automated linux from scratch");

    command!(echo hello wolrd);

    let lfs = env::var("LFS").unwrap_or_else(|_| panic!("need env variable LFS!"));

    println!("lfs is {lfs}");

    let mut runner = StageRunner::new();

    runner.add(Stage2::new());

    runner.run_all();
}
