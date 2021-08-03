use std::{fs::read_dir, process::Command, thread::{self, JoinHandle}};

fn main() {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for file in read_dir("regions").unwrap() {
        let file = file.unwrap();
        handles.push(
            thread::spawn(move || {
                let mut command = Command::new("python");
                let args = command.args(&["program.py", file.file_name().to_str().unwrap(), "basic"]);
                let mut child = args.spawn().expect("Failed to start python program for basic");
                child.wait().unwrap();
                let mut command = Command::new("python");
                let args = command.args(&["program.py", file.file_name().to_str().unwrap(), "poke"]);
                let mut child = args.spawn().expect("Failed to start python program for poke");
                child.wait().unwrap();
            })
        );
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
