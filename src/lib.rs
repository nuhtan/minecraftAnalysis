use std::{collections::VecDeque, fs, iter::FromIterator, path::Path, process, time::Instant};

use mvp_anvil::{chunk::Chunk, region::Region};
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
pub struct CachingRegion<'a> {
    region: &'a Region<'a>,
    chunks: Vec<Vec<Option<Chunk>>>,
}

impl<'a> CachingRegion<'a> {
    fn new(region: &'a Region<'a>) -> CachingRegion<'a> {
        return CachingRegion {
            region,
            chunks: vec![vec![None; 32]; 32]
        }
    }

    fn get_chunk(&mut self, x: usize, z: usize) -> &Chunk {
        if self.chunks[x][z].is_none() {
            self.chunks[x][z] = Some(self.region.get_chunk(x as u32, z as u32));
        }

        return self.chunks[x][z].as_ref().unwrap();
    }
}