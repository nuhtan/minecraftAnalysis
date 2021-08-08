mod mining;
mod techniques;

use mining::iterable_ore_expansion;
use techniques::Technique;

use mvp_anvil::region::Region;
use threadpool::ThreadPool;

use std::{
    collections::hash_map::HashMap,
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        todo!("Functionality for the interactive menu is not yet implemented...");
    } else if args.len() < 2 {
        display_help();
    } else {
        let first = args[1].clone();
        match first.as_str() {
            "single" => {
                if args.len() != 5 && args.len() != 6 {
                    single_help();
                } else {
                    let file = args[2].clone();
                    let technique = match args[3].as_str() {
                        "branch" => Technique::Branch,
                        "poke" => Technique::BranchWithPoke,
                        _ => panic!("Invalid technique, should be 'branch' or 'poke'."),
                    };
                    let y = args[4].parse().unwrap();
                    let verbosity = if args.len() == 6 {
                        match args[5].as_str() {
                            "high" => Verbosity::High,
                            "low" => Verbosity::Low,
                            _ => panic!(
                                "Invalid verbosity, should be 'high', 'low', or not included."
                            ),
                        }
                    } else {
                        Verbosity::None
                    };
                    simulate(file, technique, y, verbosity);
                }
            }
            // range file technique min max verbosity
            "range" => {
                if args.len() != 6 && args.len() != 7 {
                    range_help();
                } else {
                    let file = args[2].clone();
                    let technique = match args[3].as_str() {
                        "branch" => Technique::Branch,
                        "poke" => Technique::BranchWithPoke,
                        _ => panic!("Invalid technique, should be 'branch' or 'poke'."),
                    };
                    let min = args[4].parse().unwrap();
                    let max = args[5].parse().unwrap();
                    let verbosity = if args.len() == 7 {
                        match args[6].as_str() {
                            "high" => Verbosity::High,
                            "low" => Verbosity::Low,
                            _ => panic!(
                                "Invalid verbosity, should be 'high', 'low', or not included."
                            ),
                        }
                    } else {
                        Verbosity::None
                    };
                    simulate_range(file, technique, max, min, verbosity);
                }
            }
            // full threads min max verbosity
            "full" => {
                if args.len() != 5 && args.len() != 6 {
                    full_help();
                } else {
                    let threads = args[2].parse().unwrap();
                    let min = args[3].parse().unwrap();
                    let max = args[4].parse().unwrap();
                    let verbosity = if args.len() == 6 {
                        match args[5].as_str() {
                            "high" => Verbosity::High,
                            "low" => Verbosity::Low,
                            _ => panic!(
                                "Invalid verbosity, should be 'high', 'low', or not included."
                            ),
                        }
                    } else {
                        Verbosity::None
                    };
                    let pool = ThreadPool::new(threads);
                    for file in fs::read_dir("regions").unwrap() {
                        let verbosity = verbosity.clone();
                        let file = file.unwrap();
                        if file.file_name().to_str().unwrap().contains(".mca") {
                            pool.execute(move || {
                                for technique in &[Technique::Branch, Technique::BranchWithPoke] {
                                    simulate_range(
                                        file.file_name().to_str().unwrap().to_string(),
                                        technique.clone(),
                                        max,
                                        min,
                                        verbosity.clone(),
                                    );
                                }
                            });
                        }
                    }
                }
            }
            // chunk threads mix max verbosity
            "chunk" => {
                if args.len() != 5 && args.len() != 6 {
                    chunk_help();
                } else {
                    let threads = args[2].parse().unwrap();
                    let min = args[3].parse().unwrap();
                    let max = args[4].parse().unwrap();
                    let verbosity = if args.len() == 6 {
                        match args[5].as_str() {
                            "high" => Verbosity::High,
                            "low" => Verbosity::Low,
                            _ => panic!(
                                "Invalid verbosity, should be 'high', 'low', or not included."
                            ),
                        }
                    } else {
                        Verbosity::None
                    };
                    let pool = ThreadPool::new(threads);
                    for file in fs::read_dir("regions").unwrap() {
                        let verbosity = verbosity.clone();
                        let file = file.unwrap();
                        if file.file_name().to_str().unwrap().contains(".mca") {
                            pool.execute(move || {
                                chunk_analysis(
                                    file.file_name().to_str().unwrap().to_string(),
                                    max,
                                    min,
                                    verbosity,
                                );
                            });
                        }
                    }
                }
            }
            "help" => {
                display_help();
            }
            _ => {
                display_help();
            }
        }
    }
}

fn single_help() {
    println!("Single Simulation:");
    println!("./minecraft_analysis single file technique y verbosity");
    println!("Where 'file' is the name of the regions file, 'technique' is either 'branch' or 'poke', where 'y' is the y level that the simulation should take place at.");
    println!("'verbosity' is an optional parameter that is not required. Valid inputs are 'low' and 'high'.");
    println!("");
}

fn range_help() {
    println!("Simulation over a Range:");
    println!("./minecraft_analysis range file technique min max verbosity");
    println!("Where 'file' is the name of the regions file, 'technique' is either 'branch' or 'poke', where 'min' is the y level for the first simulation, and 'max' is the final y level that should be simulated.");
    println!("'verbosity' is an optional parameter that is not required. Valid inputs are 'low' and 'high'.");
    println!("");
}

fn full_help() {
    println!("Full Simulation:");
    println!("./minecraft_analysis full threads min max verbosity");
    println!("Where 'threads' is the number of threads allocated to the simulation, 'min' is the minimum y value that should be simulated, and 'max' is the maximum y level that should be simulated.");
    println!("'verbosity' is an optional parameter that is not required. Valid inputs are 'low' and 'high'.");
    println!("");
}

fn chunk_help() {
    println!("Chunk Data:");
    println!("./minecraft_analysis chunk threads min max verbosity");
    println!("Where 'threads' is the number of threads allocated to the simulation, 'min' is the minimum y value that should be simulated, and 'max' is the maximum y level that should be simulated.");
    println!("'verbosity' is an optional parameter that is not required. Valid inputs are 'low' and 'high'.");
    println!("");
}

fn display_help() {
    single_help();
    range_help();
    full_help();
    chunk_help();
}

#[derive(Clone)]
enum Verbosity {
    Low,
    High,
    None,
}

fn simulate_range(
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
    let tech_loop = technique.clone();
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
        let technique = tech_loop.clone();
        match verbosity {
            Verbosity::High => println!("Starting y level: {} for file {} with technique: {}", y, region_file_name, technique.name()),
            Verbosity::Low => println!("Starting y level: {}", y),
            Verbosity::None => {},
        }
        println!("Starting y: {}", y);
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

fn simulate(
    region_file_name: String,
    technique: Technique,
    y: i32,
    verbosity: Verbosity,
) -> HashMap<String, i32> {
    let region = Region::from_file(format!("regions/{}", region_file_name));
    let exp_region = region.clone();
    let mut sim_results = match technique {
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
        Verbosity::High => println!("{} mining sim finished with {} blocks exposed and checked.", technique.name(), sim_results.2),
        Verbosity::Low => println!("Finished mining sim"),
        Verbosity::None => {},
    }
    let start_mined = sim_results.1;
    let mut lava = Vec::new();
    let mut ores = Vec::new();
    let valid = get_valid_blocks();
    let exp_valid = valid.clone();
    for block in sim_results.0 {
        if block.block == "lava" || block.block == "flowing_lava" {
            lava.push(block);
        } else if valid.contains_key(&block.block) {
            ores.push(block);
        }
    }
    let mut expanded_ores = Vec::new();
    let mut more_exposed = Vec::new();
    let ores_starting = ores.len();
    match verbosity {
        Verbosity::High => println!("Ore expansion starting with {} ores.", ores.len()),
        Verbosity::Low => println!("Starting ore expansion"),
        Verbosity::None => {},
    }
    for ore in ores {
        let region = exp_region.clone();
        let valid = exp_valid.clone();
        let (mut expanded, mut new_exposed) = iterable_ore_expansion(region, valid, ore.get_coords());
        expanded_ores.append(&mut expanded);
        more_exposed.append(&mut new_exposed);
    }
    let mut trimmed = Vec::new();
    for ore in expanded_ores {
        let mut found = false;
        for comparison in &trimmed {
            if ore == *comparison {
                found = true;
            }
        }
        if !found {
            trimmed.push(ore);
        }
    }
    let mut trimmed_new_exposed = Vec::new();
    for exposed_new in more_exposed {
        let mut found = false;
        for comparison in trimmed_new_exposed.clone() {
            if exposed_new == comparison {
                found = true;
            }
        }
        if !found {
            trimmed_new_exposed.push(exposed_new);
        }
    }
    sim_results.2 += trimmed_new_exposed.len() as u32;

    let mut results = HashMap::new();
    results.insert(String::from("iron"), 0);
    results.insert(String::from("gold"), 0);
    results.insert(String::from("diamonds"), 0);
    results.insert(String::from("copper"), 0);
    results.insert(String::from("redstone"), 0);
    results.insert(String::from("lapis"), 0);
    results.insert(String::from("coal"), 0);
    results.insert(String::from("emeralds"), 0);
    let ores_ending = trimmed.len();

    for mut ore in trimmed {
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

    results.insert(
        String::from("blocks mined"),
        start_mined as i32 + (ores_ending - ores_starting) as i32,
    );
    results.insert(String::from("blocks exposed"), sim_results.2 as i32);
    results.insert(String::from("lava"), lava.len() as i32);
    return results;
}

fn chunk_analysis(region_file_name: String, max: i32, min: i32, verbosity: Verbosity) {}

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
