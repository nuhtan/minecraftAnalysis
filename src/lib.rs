use std::time::Instant;

use techniques::Technique;

pub mod mining;
pub mod simulations;
pub mod techniques;

#[derive(Clone)]
pub enum ProgramStatus {
    // id, simulation_type, region_file, start_time, y
    StartingSim(u32, Technique, String, Instant, i32),
    // id, activity, blocks, exposed, lava, ores
    UpdateSim(u32, String, u32, u32, u32, u32),
    // id, end_time
    FinishSim(u32)
}