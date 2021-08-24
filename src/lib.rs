use std::{collections::VecDeque, fs, iter::FromIterator, path::Path, process, time::Instant};

use techniques::Technique;
use tui::widgets::ListState;

pub mod mining;
pub mod simulations;
pub mod techniques;
pub mod ui;

#[derive(Clone)]
pub enum ProgramStatus {
    // id, simulation_type, region_file, start_time, y
    StartingSim(u32, Technique, String, Instant, i32),
    // id, activity, blocks, exposed, lava, ores
    UpdateSim(u32, String, u32, u32, u32, u32),
    // id, end_time
    FinishSim(u32)
}
