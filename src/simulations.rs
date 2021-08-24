use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
    sync::mpsc::Sender,
    time::Instant,
};

use mvp_anvil::region::Region;

use crate::{
    mining::Direction,
    techniques::{self, branch_mining, branch_mining_with_poke_holes, chunks, Technique},
    ProgramStatus,
};

pub fn simulate_range(
    region_file_name: String,
    technique: &Technique,
    max: i32,
    min: i32,
    id: u32,
    sender: Sender<ProgramStatus>,
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
        technique.clone().name()
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
        let file_name = f_name.clone();
        let tech = t.clone();
        let results = simulate(file_name, tech, y, id, sender.clone());
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
    id: u32,
    sender: Sender<ProgramStatus>,
) -> HashMap<String, i32> {
    sender
        .send(ProgramStatus::StartingSim(
            id,
            technique.clone(),
            region_file_name.clone(),
            Instant::now(),
            y,
        ))
        .unwrap();
    let region = Region::from_file(format!("regions/{}", region_file_name));
    let sim_results = match technique {
        Technique::Branch => branch_mining(
            &region,
            &Direction::South,
            (255, y, 255),
            16,
            160,
            5,
            id,
            sender.clone(),
        ),
        Technique::BranchWithPoke => branch_mining_with_poke_holes(
            &region,
            &Direction::South,
            (255, y, 255),
            10,
            25,
            5,
            12,
            id,
            sender.clone(),
        ),
        _ => unreachable!("Don't do a basic simulation on a non standard technique."),
    };
    let mut lava = 0;
    let mut ores = Vec::new();
    sender
        .send(ProgramStatus::UpdateSim(
            id,
            format!("Filtering Blocks"),
            sim_results.1,
            sim_results.2,
            lava as u32,
            ores.len() as u32,
        ))
        .unwrap();
    let valid = get_valid_blocks();
    for block in sim_results.0 {
        if block.block == "lava" || block.block == "flowing_lava" {
            lava += 1;
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
    sender
        .send(ProgramStatus::UpdateSim(
            id,
            format!("Compiling Results"),
            sim_results.1,
            sim_results.2,
            lava as u32,
            ores.len() as u32,
        ))
        .unwrap();
    for mut ore in ores {
        let key = valid.get(&mut ore.block).unwrap();
        if let Some(c) = results.get_mut(key) {
            *c += 1
        }
    }

    results.insert(String::from("blocks mined"), sim_results.1 as i32);
    results.insert(String::from("blocks exposed"), sim_results.2 as i32);
    results.insert(String::from("lava"), lava as i32);
    // println!("Simulation took {} secs", timer.elapsed().as_secs());
    sender.send(ProgramStatus::FinishSim(id)).unwrap();
    return results;
}

pub fn chunk_analysis(
    region_file_name: String,
    max: i32,
    min: i32,
    id: u32,
    sender: Sender<ProgramStatus>,
) {
    fs::remove_file(format!("chunk_data/{}_chunks.csv", region_file_name,)).ok();
    File::create(format!("chunk_data/{}_chunks.csv", region_file_name,)).unwrap();
    let mut csv_writer =
        csv::Writer::from_path(format!("chunk_data/{}_chunks.csv", region_file_name,)).unwrap();
    csv_writer
        .write_record(&[
            "chunk_x", "chunk_z", "y", "air", "lava", "coal", "copper", "iron", "lapis",
            "redstone", "gold", "emeralds", "diamonds",
        ])
        .unwrap();
    let region = Region::from_file(format!("regions/{}", region_file_name));
    sender
        .send(ProgramStatus::StartingSim(
            id,
            Technique::Chunk,
            region_file_name.clone(),
            Instant::now(),
            0,
        ))
        .unwrap();
    sender
        .send(ProgramStatus::UpdateSim(
            id,
            format!("Processing Chunks"),
            0,
            0,
            0,
            0,
        ))
        .unwrap();
    for x in 0..32 {
        for z in 0..32 {
            let chunk = region.get_chunk(x, z);
            for y in min..max {
                let blocks = techniques::chunks(&chunk, y);
                let mut lava = 0;
                let mut ores = Vec::new();
                let mut air = 0;
                let valid = get_valid_blocks();
                for block in blocks {
                    if block.as_str() == "lava" || block.as_str() == "flowing_lava" {
                        lava += 1;
                    } else if block.as_str() == "air" {
                        air += 1;
                    } else if valid.contains_key(&block) {
                        ores.push(block);
                    }
                }
                // println!("{}", ores.len());
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
                    let key = valid.get(&mut ore).unwrap();
                    if let Some(c) = results.get_mut(key) {
                        *c += 1
                    }
                }

                csv_writer
                    .write_record(&[
                        x.to_string(),
                        z.to_string(),
                        y.to_string(),
                        air.to_string(),
                        lava.to_string(),
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
    }
    sender.send(ProgramStatus::FinishSim(id)).unwrap();
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
