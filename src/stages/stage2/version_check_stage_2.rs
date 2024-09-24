use std::process::{Command, exit};
use std::str;

fn bail(message: &str) {
    eprintln!("FATAL: {}", message);
    exit(1);
}

fn check_command(cmd: &str, flag: &str) -> bool {
    Command::new(cmd)
        .arg(flag)
        .output()
        .is_ok()
}

fn ver_check(name: &str, cmd: &str, required_version: &str) -> bool {
    if !check_command(cmd, "--version") {
        eprintln!("ERROR: Cannot find {} ({})", cmd, name);
        return false;
    }

    let output = Command::new(cmd)
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    let version = str::from_utf8(&output.stdout)
        .unwrap()
        .lines()
        .find_map(|line| {
            let re = regex::Regex::new(r"[0-9]+\.[0-9\.]+[a-z]*").unwrap();
            re.find(line).map(|m| m.as_str())
        })
        .unwrap_or("");

    let version_sort = Command::new("sort")
        .arg("--version-sort")
        .arg("--check")
        .arg(required_version)
        .arg(version)
        .output()
        .is_ok();

    if version_sort {
        println!("OK:    {:<9} {:<6} >= {}", name, version, required_version);
        return true;
    } else {
        eprintln!("ERROR: {:<9} is TOO OLD ({} or later required)", name, required_version);
        return false;
    }
}

fn ver_kernel(required_version: &str) -> bool {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute uname");

    let kernel_version = str::from_utf8(&output.stdout)
        .unwrap()
        .split('.')
        .take(2)
        .collect::<Vec<&str>>()
        .join(".");

    let version_sort = Command::new("sort")
        .arg("--version-sort")
        .arg("--check")
        .arg(required_version)
        .arg(&kernel_version)
        .output()
        .is_ok();

    if version_sort {
        println!("OK:    Linux Kernel {} >= {}", kernel_version, required_version);
        return true;
    } else {
        eprintln!("ERROR: Linux Kernel ({}) is TOO OLD ({} or later required)", kernel_version, required_version);
        return false;
    }
}

fn alias_check(cmd: &str, expected_output: &str) {
    let output = Command::new(cmd)
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    if str::from_utf8(&output.stdout)
        .unwrap()
        .to_lowercase()
        .contains(&expected_output.to_lowercase())
    {
        println!("OK:    {:<4} is {}", cmd, expected_output);
    } else {
        eprintln!("ERROR: {:<4} is NOT {}", cmd, expected_output);
    }
}

fn compiler_check() -> bool {
    let source_code = "int main(){}";
    let mut child = Command::new("g++")
        .arg("-x")
        .arg("c++")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute g++");

    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin.write_all(source_code.as_bytes()).unwrap();
    }

    let output = child.wait_with_output().expect("Failed to read g++ output");
    output.status.success()
}

pub fn check_all_versions_stage_2() {
    if !ver_check("Coreutils", "sort", "8.1") { bail("Coreutils too old, stop"); }
    ver_check("Bash", "bash", "3.2");
    ver_check("Binutils", "ld", "2.13.1");
    ver_check("Bison", "bison", "2.7");
    ver_check("Diffutils", "diff", "2.8.1");
    ver_check("Findutils", "find", "4.2.31");
    ver_check("Gawk", "gawk", "4.0.1");
    ver_check("GCC", "gcc", "5.2");
    ver_check("GCC (C++)", "g++", "5.2");
    ver_check("Grep", "grep", "2.5.1a");
    ver_check("Gzip", "gzip", "1.3.12");
    ver_check("M4", "m4", "1.4.10");
    ver_check("Make", "make", "4.0");
    ver_check("Patch", "patch", "2.5.4");
    ver_check("Perl", "perl", "5.8.8");
    ver_check("Python", "python3", "3.4");
    ver_check("Sed", "sed", "4.1.5");
    ver_check("Tar", "tar", "1.22");
    ver_check("Texinfo", "texi2any", "5.0");
    ver_check("Xz", "xz", "5.0.0");
    ver_kernel("4.19");

    if let Ok(output) = Command::new("mount").output() {
        let output_str = str::from_utf8(&output.stdout).unwrap();
        if output_str.contains("devpts on /dev/pts") && std::path::Path::new("/dev/ptmx").exists() {
            println!("OK:    Linux Kernel supports UNIX 98 PTY");
        } else {
            eprintln!("ERROR: Linux Kernel does NOT support UNIX 98 PTY");
        }
    }

    println!("Aliases:");
    alias_check("awk", "GNU");
    alias_check("yacc", "Bison");
    alias_check("sh", "Bash");

    println!("Compiler check:");
    if compiler_check() {
        println!("OK:    g++ works");
    } else {
        eprintln!("ERROR: g++ does NOT work");
    }

    if let Ok(output) = Command::new("nproc").output() {
        let num_cores = str::from_utf8(&output.stdout).unwrap().trim();
        if !num_cores.is_empty() {
            println!("OK: nproc reports {} logical cores are available", num_cores);
        } else {
            eprintln!("ERROR: nproc is not available or it produces empty output");
        }
    }
}
