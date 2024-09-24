use std::{io, process::Command};

use regex::Regex;

#[derive(Debug)]
pub struct SelectedPartitions {
    pub boot: String,
    pub main: String,
}

pub fn select_all_patitions() -> Option<SelectedPartitions> {
    let partitions = list_partitions();

    if partitions.is_empty() {
        panic!("No partitions available.");
    }

    println!("Please select the boot partition (or type 'exit' to quit): ");
    let boot_partition = select_partition(&partitions)?;

    println!("Please select the main partition (or type 'exit' to quit): ");
    let main_partition = select_partition(&partitions)?;

    println!(
        "Boot Partition: {}\nMain Partition: {}",
        boot_partition, main_partition
    );
    if confirm_selection() {
        println!("Partition selection completed.");
        return Some(SelectedPartitions {
            boot: boot_partition,
            main: main_partition,
        });
    } else {
        return None;
    }
}

fn list_partitions() -> Vec<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("sudo fdisk -l | grep -E '/dev/[a-z0-9]+\\ '")
        .output()
        .expect("Failed to list partitions");

    let partitions: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.to_string())
        .collect();

    for (i, partition) in partitions.iter().enumerate() {
        println!("{}: {}", i + 1, partition);
    }

    partitions
}

fn select_partition(partitions: &[String]) -> Option<String> {
    let regex = Regex::new("/dev/[a-z0-9]+").unwrap();
    loop {
        let input = get_user_input();
        if input == "exit" {
            return None;
        }

        if let Ok(num) = input.parse::<usize>() {
            if num > 0 && num <= partitions.len() {
                return Some(
                    regex
                        .find(&partitions[num - 1])
                        .unwrap()
                        .as_str()
                        .to_string(),
                );
            }
        }
        println!("Invalid selection. Please enter a valid number.");
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn confirm_selection() -> bool {
    println!("Is this correct? (y/n)");
    get_user_input().to_lowercase() == "y"
}
