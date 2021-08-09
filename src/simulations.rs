use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
};

use mvp_anvil::region::Region;

use crate::{
    mining,
    techniques::{self, Technique},
    Verbosity,
};

pub fn simulate_range(
    region_file_name: String,
    technique: Technique,
    max: i32,
    min: i32,
    verbosity: Verbosity,
) {
    let f_name = region_file_name.clone();
    let t = technique.clone();
    let t1 = technique.clone();
    let t2 = technique.clone();
    fs::remove_file(format!(
        "mining_data/result-{}-{}.csv",
        region_file_name,
        t1.name()
    ))
    .ok();
    File::create(format!(
        "mining_data/result-{}-{}.csv",
        region_file_name,
        t2.name()
    ))
    .unwrap();
    let mut csv_writer = csv::Writer::from_path(format!(
        "mining_data/result-{}-{}.csv",
        region_file_name,
        technique.name()
    ))
    .unwrap();
    csv_writer
        .write_record(&[
            "y",
            "blocks mined",
            "blocks exposed",
            "lava",
            "coal",
            "copper",
            "iron",
            "lapis",
            "redstone",
            "gold",
            "emeralds",
            "diamonds",
        ])
        .unwrap();
    for y in min..max {
        let verbosity = verbosity.clone();
        let file_name = f_name.clone();
        let tech = t.clone();
        let results = simulate(file_name, tech, y, verbosity);
        csv_writer
            .write_record(&[
                y.to_string(),
                results.get("blocks mined").unwrap().to_string(),
                results.get("blocks exposed").unwrap().to_string(),
                results.get("lava").unwrap().to_string(),
                results.get("coal").unwrap().to_string(),
                results.get("copper").unwrap().to_string(),
                results.get("iron").unwrap().to_string(),
                results.get("lapis").unwrap().to_string(),
                results.get("redstone").unwrap().to_string(),
                results.get("gold").unwrap().to_string(),
                results.get("emeralds").unwrap().to_string(),
                results.get("diamonds").unwrap().to_string(),
            ])
            .unwrap();
    }
}

pub fn simulate(
    region_file_name: String,
    technique: Technique,
    y: i32,
    verbosity: Verbosity,
) -> HashMap<String, i32> {
    let technique_v = technique.clone();
    match verbosity {
        Verbosity::High => println!(
            "starting y level: {} for file {} with technique: {}.",
            y,
            region_file_name,
            technique_v.name()
        ),
        Verbosity::Low => println!("starting y level: {}.", y),
        Verbosity::None => {}
    }
    let region = Region::from_file(format!("regions/{}", region_file_name));
    let sim_results = match technique {
        Technique::Branch => {
            techniques::branch_mining(region, mining::Direction::South, (255, y, 255), 16, 160, 5)
        }
        Technique::BranchWithPoke => techniques::branch_mining_with_poke_holes(
            region,
            mining::Direction::South,
            (255, y, 255),
            10,
            25,
            5,
            12,
        ),
    };
    match verbosity {
        Verbosity::High => println!(
            "{} mining sim finished with {} blocks exposed and checked.",
            technique.name(),
            sim_results.2
        ),
        Verbosity::Low => println!("finished mining sim"),
        Verbosity::None => {}
    }
    let mut lava = Vec::new();
    let mut ores = Vec::new();
    let valid = get_valid_blocks();
    for block in sim_results.0 {
        if block.block == "lava" || block.block == "flowing_lava" {
            lava.push(block);
        } else if valid.contains_key(&block.block) {
            ores.push(block);
        }
    }

    let mut results = HashMap::new();
    results.insert(String::from("iron"), 0);
    results.insert(String::from("gold"), 0);
    results.insert(String::from("diamonds"), 0);
    results.insert(String::from("copper"), 0);
    results.insert(String::from("redstone"), 0);
    results.insert(String::from("lapis"), 0);
    results.insert(String::from("coal"), 0);
    results.insert(String::from("emeralds"), 0);

    for mut ore in ores {
        if results.contains_key(valid.get(&mut ore.block).unwrap()) {
            let key = valid.get(&mut ore.block).unwrap();
            if let Some(c) = results.get_mut(key) {
                *c += 1
            }
        } else {
            let temp = valid.get(&mut ore.block).unwrap();
            let test = temp.clone();
            results.insert(test, 1);
        }
    }

    results.insert(String::from("blocks mined"), sim_results.1 as i32);
    results.insert(String::from("blocks exposed"), sim_results.2 as i32);
    results.insert(String::from("lava"), lava.len() as i32);
    return results;
}

pub fn chunk_analysis(region_file_name: String, max: i32, min: i32, verbosity: Verbosity) {
    unimplemented!("Not yet cuh")
}

fn get_valid_blocks() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in BufReader::new(File::open("ValidBlocks.txt").unwrap()).lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(":").collect();
        let input = String::from(parts[0]);
        let output = String::from(parts[1]);
        map.insert(input, output);
    }
    return map;
}
