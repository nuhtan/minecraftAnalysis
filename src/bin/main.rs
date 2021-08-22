use std::{fs, io::Error, sync::{Arc, Mutex, mpsc}, thread, time::Instant};

use threadpool::ThreadPool;

fn main() -> Result<(), Error> {
    match mcsim::ui::determine_simulation() {
        Ok(cont) => {
            match cont.0 {
                true => {
                    // Create mpsc channels
                    let (transmitter, receiver) = mpsc::channel();
                    let end = Arc::new(Mutex::new(false));
                    let sim_end = end.clone();
                    let title;
                    let allocated_threads;
                    let files;
                    let techniques;
                    let y_range;
                    let start = Instant::now();
                    // Spawn threads for sims
                    let mut pool = ThreadPool::new(1);
                    let mut id = 0;
                    match cont.1.unwrap() {
                        mcsim::Simulations::Single(tech, file_name, y) => {
                            pool.execute(move || {
                                mcsim::simulations::simulate(file_name, tech, y, id, transmitter);
                            });
                            title = String::from("Single Simulation");
                            allocated_threads = 1;
                            files = 1;
                            techniques = 1;
                            y_range = (y, y);
                        }
                        mcsim::Simulations::Range(tech, file_name, min, max) => {
                            pool.execute(move || {
                                mcsim::simulations::simulate_range(
                                    file_name,
                                    &tech,
                                    max,
                                    min,
                                    id,
                                    transmitter,
                                );
                            });
                            title = String::from("Range Simulation");
                            allocated_threads = 1;
                            files = 1;
                            techniques = 1;
                            y_range = (min, max);
                        }
                        mcsim::Simulations::Techniques(techs, min, max, threads) => {
                            let mut file_count = 0;
                            pool = ThreadPool::new(threads as usize);
                            for file in fs::read_dir("regions").unwrap() {
                                let file = file.unwrap();
                                if file.file_name().to_str().unwrap().contains(".mca") {
                                    file_count += 1;
                                    let transmitter = transmitter.clone();
                                    let techs = techs.clone();
                                    pool.execute(move || {
                                        for tech in techs {
                                            mcsim::simulations::simulate_range(
                                                file.file_name().to_str().unwrap().to_string(),
                                                &tech,
                                                max,
                                                min,
                                                id,
                                                transmitter.clone(),
                                            );
                                        }
                                    });
                                }
                                id += 1;
                            }
                            title = String::from("Technique Comparison Simulation");
                            allocated_threads = threads;
                            files = file_count;
                            techniques = techs.len();
                            y_range = (min, max);
                        }
                        mcsim::Simulations::TechniqueParameters(techs, min, max, threads) => {
                            todo!("yeah, not yet");
                            title = String::from("Technique Parameters Simulation");
                            allocated_threads = threads;
                            files = 0;
                            techniques = techs.len();
                            y_range = (min, max);
                        }
                        mcsim::Simulations::Chunks(min, max, threads) => {
                            pool = ThreadPool::new(threads as usize);
                            for file in fs::read_dir("regions").unwrap() {
                                let file = file.unwrap();
                                if file.file_name().to_str().unwrap().contains(".mca") {
                                    let transmitter = transmitter.clone();
                                    pool.execute(move || {
                                        mcsim::simulations::chunk_analysis(
                                            file.file_name().to_str().unwrap().to_string(),
                                            max,
                                            min,
                                            id,
                                            transmitter,
                                        );
                                    });
                                }
                                id += 1;
                            }
                            title = String::from("Technique Comparison Simulation");
                            allocated_threads = threads;
                            files = 0;
                            techniques = 1;
                            y_range = (min, max);
                        }
                    }
                    // Create thread with sim ui
                    let handle = thread::spawn(move || {
                        mcsim::ui::simulation_ui(
                            receiver,
                            sim_end,
                            title,
                            files,
                            allocated_threads,
                            techniques as u32,
                            y_range,
                            start,
                        )
                    });
                    pool.join();
                    {
                        let mut ending = end.lock().unwrap();
                        *ending = true;
                    }
                    handle.join().unwrap();
                    println!("Took {} seconds", start.elapsed().as_secs());
                }
                false => {}
            }
        }
        Err(_) => {}
    }
    Ok(())
}

