use core::panic;
use std::collections::{HashMap, VecDeque};

use mvp_anvil::region::Region;

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
    direction: Direction,
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

pub fn iterable_ore_expansion(
    region: Region,
    valid: HashMap<String, String>,
    coords: (i32, i32, i32),
) -> (Vec<SimpleBlock>, Vec<SimpleBlock>) {
    let mut deq = VecDeque::new();
    let loop_reg = region.clone();
    deq.push_back(SimpleBlock::new(coords, get_block(region, coords)));
    let mut expanded = Vec::new();
    let mut exposed = Vec::new();

    while deq.len() > 0 {
        let reg = loop_reg.clone();
        let block = deq.pop_front().unwrap();
        let cen_block = block.clone();
        &expanded.push(block);
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    let adj_reg = reg.clone();
                    let adj_reg2 = reg.clone();
                    let new_coords = (cen_block.x + x, cen_block.y + y, cen_block.z + z);
                    let exp = SimpleBlock::new(new_coords, get_block(adj_reg, new_coords));
                    exposed.push(exp);
                    if !(x == 0 && y == 0 && z == 0) {
                        let adj = SimpleBlock::new(new_coords, get_block(adj_reg2, new_coords));
                        if valid.contains_key(&adj.block) {
                            let mut found = false;
                            let exp = expanded.clone();
                            for b in exp {
                                if b == adj {
                                    found = true;
                                }
                            }
                            for b in &deq {
                                if *b == adj {
                                    found = true;
                                }
                            }
                            if !found {
                                deq.push_back(adj);
                            }
                        }
                    }
                }
            }
        }
    }

    return (expanded, exposed);
}

pub fn get_block(region: Region, coords: (i32, i32, i32)) -> String {
    let chunk_x = coords.0 / 16;
    let chunk_z = coords.2 / 16;
    let chunk = region.get_chunk(chunk_x as u32, chunk_z as u32);
    return chunk.get_block(coords.0 % 16, coords.1, coords.2 % 16).id;
}

fn two_by_one_single(
    region: Region,
    direction: Direction,
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
            let z_range = z_range.clone();
            for z in z_range {
                let region = region.clone();
                let new_coords = (coords.0 + x, coords.1 + y, coords.2 + z);
                blocks.push(SimpleBlock::new(new_coords, get_block(region, new_coords)));
            }
        }
    }
    let y_clone = region.clone();
    blocks.push(SimpleBlock::new(
        (coords.0, coords.1 - 1, coords.2),
        get_block(region, (coords.0, coords.1 - 1, coords.2)),
    ));
    blocks.push(SimpleBlock::new(
        (coords.0, coords.1 + 2, coords.2),
        get_block(y_clone, (coords.0, coords.1 + 2, coords.2)),
    ));
    return (blocks, 2, 8);
}

pub fn two_by_one_length(
    region: Region,
    direction: Direction,
    coords: (i32, i32, i32),
    length: i32,
) -> (Vec<SimpleBlock>, u32, u32) {
    let mut results = (Vec::new(), 0, 0);
    for n in 0..length {
        let region = region.clone();
        let direction = direction.clone();
        let direction_dup = direction.clone();
        let mut res = two_by_one_single(region, direction, shift_coords(direction_dup, coords, n));
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
    }
    return results;
}

pub fn two_by_one_end(
    region: Region,
    direction: Direction,
    coords: (i32, i32, i32),
) -> (Vec<SimpleBlock>, u32, u32) {
    let mut results = (Vec::new(), 0, 2);
    let region_copy = region.clone();
    match direction {
        Direction::North => {
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1, coords.2 - 1),
                get_block(region, (coords.0, coords.1, coords.2 - 1)),
            ));
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1 + 1, coords.2 - 1),
                get_block(region_copy, (coords.0, coords.1 + 1, coords.2 - 1)),
            ));
        }
        Direction::South => {
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1, coords.2 + 1),
                get_block(region, (coords.0, coords.1, coords.2 + 1)),
            ));
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1 + 1, coords.2 + 1),
                get_block(region_copy, (coords.0, coords.1 + 1, coords.2 + 1)),
            ));
        }
        Direction::East => {
            results.0.push(SimpleBlock::new(
                (coords.0 + 1, coords.1, coords.2),
                get_block(region, (coords.0 + 1, coords.1, coords.2)),
            ));
            results.0.push(SimpleBlock::new(
                (coords.0 + 1, coords.1 + 1, coords.2),
                get_block(region_copy, (coords.0 + 1, coords.1 + 1, coords.2)),
            ));
        }
        Direction::West => {
            results.0.push(SimpleBlock::new(
                (coords.0 - 1, coords.1, coords.2),
                get_block(region, (coords.0 - 1, coords.1, coords.2)),
            ));
            results.0.push(SimpleBlock::new(
                (coords.0 - 1, coords.1 + 1, coords.2),
                get_block(region_copy, (coords.0 - 1, coords.1 + 1, coords.2)),
            ));
        }
    }
    return results;
}

fn one_by_one_single(
    region: Region,
    direction: Direction,
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
                let region = region.clone();
                let new_coords = (coords.0 + x, coords.1 + y, coords.2 + z);
                blocks.push(SimpleBlock::new(new_coords, get_block(region, new_coords)));
            }
        }
    }
    let y_clone = region.clone();
    blocks.push(SimpleBlock::new(
        (coords.0, coords.1 - 1, coords.2),
        get_block(region, (coords.0, coords.1 - 1, coords.2)),
    ));
    blocks.push(SimpleBlock::new(
        (coords.0, coords.1 + 1, coords.2),
        get_block(y_clone, (coords.0, coords.1 + 2, coords.2)),
    ));
    return (blocks, 1, 5);
}

fn one_by_one_length(
    region: Region,
    direction: Direction,
    coords: (i32, i32, i32),
    length: i32,
) -> (Vec<SimpleBlock>, u32, u32) {
    let mut results = (Vec::new(), 0, 0);
    for n in 0..length {
        let region = region.clone();
        let direction = direction.clone();
        let direction_dup = direction.clone();
        let mut res = one_by_one_single(region, direction, shift_coords(direction_dup, coords, n));
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
    }
    return results;
}

fn one_by_one_end(
    region: Region,
    direction: Direction,
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

fn poke_start(region: Region, coords: (i32, i32, i32)) -> (Vec<SimpleBlock>, u32, u32) {
    let mut results = (Vec::new(), 0, 1);
    results.0.push(SimpleBlock::new(
        (coords.0, coords.1 + 1, coords.2),
        get_block(region, (coords.0, coords.1 + 1, coords.2)),
    ));
    return results;
}

pub fn poke(
    region: Region,
    direction: Direction,
    coords: (i32, i32, i32),
    depth: i32,
) -> (Vec<SimpleBlock>, u32, u32) {
    if depth < 1 {
        panic!("Poke should be at least 1 block in depth")
    }
    let length_region = region.clone();
    let length_direction2 = direction.clone();
    let end_region = region.clone();
    let end_direction = direction.clone();
    let end_direction2 = direction.clone();
    let mut results = (Vec::new(), 0, 0);
    let mut res = poke_start(region, coords);
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    let mut res = one_by_one_length(
        length_region,
        direction,
        shift_coords(length_direction2, coords, 1),
        depth - 1,
    );
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    let mut res = one_by_one_end(
        end_region,
        end_direction,
        shift_coords(end_direction2, coords, depth - 1),
    );
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    return results;
}
