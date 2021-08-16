use std::{env, fs};

use threadpool::ThreadPool;

/// Determines what the program should do depending on the arguments or lack of arguments passed to the program on launch.
///
/// There are six paths that this could take.
/// 1. Single - This runs a single simulation with a specified mining technique at a specified y level on a specified region file.
/// 2. Range - This runs a series of simulations with a specified mining technique through an input range of y levels on a specified region file.
/// 3. Full - This runs all mining techniques through a specified range of y levels through all region files in the 'regions' directory.
/// 4. Chunk - This runs a data gathering program on all chunks in a region file for all region files in the 'regions' directory.
/// 5. None - If there are no arguments passed to the program then an interactive menu will be presented to the user to determine which of the above paths to take.
/// 6. Help - If the user only inputs help as an argument then the help for each path will be printed to the terminal.
///
/// Each of the first four paths has an optional argument `verbosity` that represents how verbose output from the program should be. There are three tiers, high, low, and none.
pub fn handle() {
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
                    let results = simulate(file, technique, y, verbosity);
                    println!("{:?}", results);
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
                    pool.join();
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