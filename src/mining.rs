use core::panic;
use std::{collections::{HashMap, VecDeque}, time::Instant};

use mvp_anvil::region::Region;

use crate::CachingRegion;

#[derive(Clone, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug)]
pub struct SimpleBlock {
    pub block: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl PartialEq for SimpleBlock {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y && self.z == other.z {
            return true;
        }
        return false;
    }
}

impl SimpleBlock {
    pub fn new(coords: (i32, i32, i32), block: String) -> SimpleBlock {
        return SimpleBlock {
            block,
            x: coords.0,
            y: coords.1,
            z: coords.2,
        };
    }

    pub fn get_coords(self) -> (i32, i32, i32) {
        return (self.x, self.y, self.z);
    }
}

pub fn shift_coords(
    direction: &Direction,
    mut coords: (i32, i32, i32),
    amount: i32,
) -> (i32, i32, i32) {
    match direction {
        Direction::North => coords.2 -= amount,
        Direction::South => coords.2 += amount,
        Direction::East => coords.0 += amount,
        Direction::West => coords.0 -= amount,
    };
    return coords;
}

/// Takes a region file and coordinates and returns the name of the block at that location. This handles the intermediary step of determining the x and z [chunks](`mvp_anvil::chunk::Chunk`) that correspond with the coordinates.
///
/// * `region` - The [region](`mvp_anvil::region::Region`) file that the block will be retrieved from.
/// * `coords` - The tuple of xyz coordinates of the block.
pub fn get_block(region: &mut CachingRegion, coords: (i32, i32, i32)) -> String {
    
    let chunk_x = coords.0 / 16;
    let chunk_z = coords.2 / 16;
    // let t2 = Instant::now();
    let chunk = region.get_chunk(chunk_x as usize, chunk_z as usize);
    // println!("Getting chunk took {} milliseconds", t2.elapsed().as_millis());
    let timer = Instant::now();
    let block = chunk.get_block(coords.0 % 16, coords.1, coords.2 % 16).id;
    // println!("Getting block took {} ns", timer.elapsed().as_nanos());
    return block;
}

fn two_by_one_single(
    region: &mut CachingRegion,
    direction: &Direction,
    coords: (i32, i32, i32),
) -> (Vec<SimpleBlock>, u32, u32) {
    let mut blocks = Vec::new();
    let mut x_range = 0..1;
    let mut z_range = 0..1;
    match direction {
        Direction::North | Direction::South => x_range = -1..2,
        Direction::East | Direction::West => z_range = -1..2,
    };
    for x in x_range {
        for y in 0..2 {
            for z in z_range.clone() {
                let new_coords = (coords.0 + x, coords.1 + y, coords.2 + z);
                blocks.push(SimpleBlock::new(new_coords, get_block(region, new_coords)));
            }
        }
    }
    blocks.push(SimpleBlock::new(
        (coords.0, coords.1 - 1, coords.2),
        get_block(region, (coords.0, coords.1 - 1, coords.2)),
    ));
    blocks.push(SimpleBlock::new(
        (coords.0, coords.1 + 2, coords.2),
        get_block(region, (coords.0, coords.1 + 2, coords.2)),
    ));
    return (blocks, 2, 8);
}

pub fn two_by_one_length(
    region: &mut CachingRegion,
    direction: &Direction,
    coords: (i32, i32, i32),
    length: i32,
) -> (Vec<SimpleBlock>, u32, u32) {
    let mut results = (Vec::new(), 0, 0);
    for n in 0..length {
        // let direction = direction.clone();
        // let direction_dup = direction.clone();
        let mut res = two_by_one_single(region, direction, shift_coords(direction, coords, n));
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
    }
    return results;
}

pub fn two_by_one_end(
    region: &mut CachingRegion,
    direction: &Direction,
    coords: (i32, i32, i32),
) -> (Vec<SimpleBlock>, u32, u32) {
    let mut results = (Vec::new(), 0, 2);
    match direction {
        Direction::North => {
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1, coords.2 - 1),
                get_block(region, (coords.0, coords.1, coords.2 - 1)),
            ));
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1 + 1, coords.2 - 1),
                get_block(region, (coords.0, coords.1 + 1, coords.2 - 1)),
            ));
        }
        Direction::South => {
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1, coords.2 + 1),
                get_block(region, (coords.0, coords.1, coords.2 + 1)),
            ));
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1 + 1, coords.2 + 1),
                get_block(region, (coords.0, coords.1 + 1, coords.2 + 1)),
            ));
        }
        Direction::East => {
            results.0.push(SimpleBlock::new(
                (coords.0 + 1, coords.1, coords.2),
                get_block(region, (coords.0 + 1, coords.1, coords.2)),
            ));
            results.0.push(SimpleBlock::new(
                (coords.0 + 1, coords.1 + 1, coords.2),
                get_block(region, (coords.0 + 1, coords.1 + 1, coords.2)),
            ));
        }
        Direction::West => {
            results.0.push(SimpleBlock::new(
                (coords.0 - 1, coords.1, coords.2),
                get_block(region, (coords.0 - 1, coords.1, coords.2)),
            ));
            results.0.push(SimpleBlock::new(
                (coords.0 - 1, coords.1 + 1, coords.2),
                get_block(region, (coords.0 - 1, coords.1 + 1, coords.2)),
            ));
        }
    }
    return results;
}

fn one_by_one_single(
    region: &mut CachingRegion,
    direction: &Direction,
    coords: (i32, i32, i32),
) -> (Vec<SimpleBlock>, u32, u32) {
    let mut blocks = Vec::new();
    let mut x_range = 0..1;
    let mut z_range = 0..1;
    match direction {
        Direction::North | Direction::South => x_range = -1..2,
        Direction::East | Direction::West => z_range = -1..2,
    };
    for x in x_range {
        for y in 0..1 {
            let z_range = z_range.clone();
            for z in z_range {
                let new_coords = (coords.0 + x, coords.1 + y, coords.2 + z);
                blocks.push(SimpleBlock::new(new_coords, get_block(region, new_coords)));
            }
        }
    }
    blocks.push(SimpleBlock::new(
        (coords.0, coords.1 - 1, coords.2),
        get_block(region, (coords.0, coords.1 - 1, coords.2)),
    ));
    blocks.push(SimpleBlock::new(
        (coords.0, coords.1 + 1, coords.2),
        get_block(region, (coords.0, coords.1 + 2, coords.2)),
    ));
    return (blocks, 1, 5);
}

fn one_by_one_length(
    region: &mut CachingRegion,
    direction: &Direction,
    coords: (i32, i32, i32),
    length: i32,
) -> (Vec<SimpleBlock>, u32, u32) {
    let mut results = (Vec::new(), 0, 0);
    for n in 0..length {
        let mut res = one_by_one_single(region, direction, shift_coords(direction, coords, n));
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
    }
    return results;
}

fn one_by_one_end(
    region: &mut CachingRegion,
    direction: &Direction,
    coords: (i32, i32, i32),
) -> (Vec<SimpleBlock>, u32, u32) {
    let mut results = (Vec::new(), 0, 1);
    match direction {
        Direction::North => {
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1, coords.2 - 1),
                get_block(region, (coords.0, coords.1, coords.2 - 1)),
            ));
        }
        Direction::South => {
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1, coords.2 + 1),
                get_block(region, (coords.0, coords.1, coords.2 + 1)),
            ));
        }
        Direction::East => {
            results.0.push(SimpleBlock::new(
                (coords.0 + 1, coords.1, coords.2),
                get_block(region, (coords.0 + 1, coords.1, coords.2)),
            ));
        }
        Direction::West => {
            results.0.push(SimpleBlock::new(
                (coords.0 - 1, coords.1, coords.2),
                get_block(region, (coords.0 - 1, coords.1, coords.2)),
            ));
        }
    }
    return results;
}

fn poke_start(region: &mut CachingRegion, coords: (i32, i32, i32)) -> (Vec<SimpleBlock>, u32, u32) {
    let mut results = (Vec::new(), 0, 1);
    results.0.push(SimpleBlock::new(
        (coords.0, coords.1 + 1, coords.2),
        get_block(region, (coords.0, coords.1 + 1, coords.2)),
    ));
    return results;
}

pub fn poke(
    region: &mut CachingRegion,
    direction: &Direction,
    coords: (i32, i32, i32),
    depth: i32,
) -> (Vec<SimpleBlock>, u32, u32) {
    if depth < 1 {
        panic!("Poke should be at least 1 block in depth")
    }
    let mut results = (Vec::new(), 0, 0);
    let mut res = poke_start(region, coords);
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    let mut res = one_by_one_length(
        region,
        direction,
        shift_coords(direction, coords, 1),
        depth - 1,
    );
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    let mut res = one_by_one_end(
        region,
        direction,
        shift_coords(direction, coords, depth - 1),
    );
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    return results;
}
