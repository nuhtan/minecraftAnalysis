use std::{fs, path::Path, process};

fn main() {
    if verify_directory_structure() {
        arguments::handle();
    }
}

// Create mining_data, regions, if they are not already present. Fetch ValidBlocks.txt if it is not present.
fn verify_directory_structure() -> bool {
    let mut regions = true;
    if !Path::new("mining_data/").exists() {
        fs::create_dir("mining_data/").unwrap();
    }

    if !Path::new("regions/").exists() {
        fs::create_dir("regions/").unwrap();
        regions = false;
    } else if Path::new("regions/").read_dir().unwrap().next().is_none() {
        regions = false;
    }

    if !Path::new("ValidBlocks.txt").exists() {
        process::Command::new("curl").args(&["https://raw.githubusercontent.com/nuhtan/minecraft_analysis/master/ValidBlocks.txt", "-o", "ValidBlocks.txt"]).spawn().unwrap();
    }

    return regions;
}