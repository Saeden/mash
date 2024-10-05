use is_executable::IsExecutable;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let args = tokenize(line);
        println!("{:?}", args);
        if !args.is_empty() {
            if let Ok(path) = find_bin(&args[0]) {
                let proc = Command::new(path.into_os_string())
                    .args(&args[1..])
                    .spawn()
                    .expect("Failed to start {args[0]}");
                proc.wait_with_output().expect("Failed to retrieve output");
            } else {
                println!("Failed to find binary for `{}`", args[0]);
            }
        }
    }
}

fn find_bin(arg: &str) -> Result<PathBuf, std::io::Error> {
    let executable_dirs = std::env::var("PATH").unwrap();
    //println!("{:?}", executable_dirs);
    for dir in executable_dirs.split(":") {
        //println!("{dir}");
        let mut bin_path = PathBuf::from(dir);
        bin_path.push(arg);
        if bin_path.is_executable() {
            //println!("{:?}", bin_path);
            return Ok(bin_path);
        }
    }

    Err(std::io::ErrorKind::NotFound.into())
}

fn tokenize(line: String) -> Vec<String> {
    let mut buffer: Vec<char> = Vec::new();
    let mut tokens: Vec<String> = Vec::new();
    for c in line.chars() {
        if c.is_whitespace() && !buffer.is_empty() {
            tokens.push(buffer.iter().collect());
            Vec::clear(&mut buffer);
        } else if !c.is_whitespace() {
            buffer.push(c);
        }
    }
    if !buffer.is_empty() {
        tokens.push(buffer.iter().collect());
    }
    tokens
}
