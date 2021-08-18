use std::sync::mpsc::Sender;

use mvp_anvil::region::Region;

use crate::{ProgramStatus, mining::*};

#[derive(Clone)]
pub enum Technique {
    Branch,
    BranchWithPoke,
}

impl Technique {
    pub fn name(self) -> String {
        match self {
            Self::Branch => String::from("branch"),
            Self::BranchWithPoke => String::from("poke"),
        }
    }

    pub fn iterable() -> Vec<String> {
        return vec!["Branch", "Branch with Poke Holes"].iter().map(|f| f.to_string()).collect();
    }

    pub fn from_string(text: String) -> Technique {
        match text.as_str() {
            "Branch" => Technique::Branch,
            "Branch with Poke Holes" => Technique::BranchWithPoke,
            _ => unreachable!("only techs")
        }
    }
}

pub fn branch_mining(
    region: &Region,
    base_direction: &Direction,
    starting_coords: (i32, i32, i32),
    branch_pair_count: i32,
    branch_length: i32,
    branch_spacing: i32,
    id: u32,
    sender: Sender<ProgramStatus>,
) -> (Vec<SimpleBlock>, u32, u32) {
    if branch_spacing < 2 {
        panic!("Branch spacing should be at least two to avoid duplicates")
    }

    fn expand_corridor(
        region: &Region,
        direction: &Direction,
        branch_spacing: i32,
        coords: (i32, i32, i32),
    ) -> (Vec<SimpleBlock>, u32, u32) {
        let mut results = (Vec::new(), 0, 0);
        // The sides of the corridor that coincide with a branch along with the sides in front and behind. For these we don't consider the side blocks that would be exposed.
        for y in -1..3 {
            // Slice that coincides
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1 + y, coords.2),
                get_block(region, (coords.0, coords.1 + y, coords.2)),
            ));
            results.1 += 2;
            results.2 += 4;
            // Slice after
            results.0.push(SimpleBlock::new(
                shift_coords(direction, (coords.0, coords.1 + y, coords.2), 1),
                get_block(
                    region,
                    shift_coords(direction, (coords.0, coords.1 + y, coords.2), 1),
                ),
            ));
            results.1 += 2;
            results.2 += 4;
            // Slice before
            results.0.push(SimpleBlock::new(
                shift_coords(direction, (coords.0, coords.1 + y, coords.2), branch_spacing),
                get_block(
                    region,
                    shift_coords(direction, (coords.0, coords.1 + y, coords.2), branch_spacing),
                ),
            ));
            results.1 += 2;
            results.2 += 4;
        }

        let mut res = two_by_one_length(
            region,
            direction,
            shift_coords(direction, coords, 2),
            branch_spacing - 3,
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        return results;
    }

    fn branch(
        region: &Region,
        branch_length: i32,
        direction: &Direction,
        coords: (i32, i32, i32),
    ) -> (Vec<SimpleBlock>, u32, u32) {
        let mut results = (Vec::new(), 0, 0);

        let mut res = two_by_one_length(region, direction, coords, branch_length);
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;

        let mut res = two_by_one_end(
            region,
            direction,
            shift_coords(direction, coords, branch_length - 1),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;

        return results;
    }

    let mut results = (Vec::new(), 0, 0);
    let (branch_direction1, branch_direction2) =
        if base_direction == &Direction::East || base_direction == &Direction::West {
            (&Direction::North, &Direction::South)
        } else {
            (&Direction::East, &Direction::West)
        };
        sender.send(ProgramStatus::UpdateSim(id, format!("Simulating Branch 1/{}", branch_pair_count * 2), 0, 0, 0, 0)).unwrap();
    let mut res = branch(
        region,
        branch_length,
        branch_direction1,
        shift_coords(branch_direction1, starting_coords, 1),
    );
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    sender.send(ProgramStatus::UpdateSim(id, format!("Simulating Branch 2/{}", branch_pair_count * 2), results.1, results.2, 0, 0)).unwrap();
    let mut res = branch(
        region,
        branch_length,
        branch_direction2,
        shift_coords(branch_direction2, starting_coords, 1),
    );
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    for n in 0..branch_pair_count - 1 {
        sender.send(ProgramStatus::UpdateSim(id, format!("Simulating Corridor"), results.1, results.2, 0, 0)).unwrap();
        let mut res = expand_corridor(
            region,
            base_direction,
            branch_spacing,
            shift_coords(base_direction, starting_coords, n * branch_spacing),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        sender.send(ProgramStatus::UpdateSim(id, format!("Simulating Branch {}/{}", 2 + (n * 2), branch_pair_count * 2), results.1, results.2, 0, 0)).unwrap();
        let mut res = branch(
            region,
            branch_length,
            branch_direction1,
            shift_coords(
                base_direction,
                shift_coords(branch_direction1, starting_coords, 1),
                (n + 1) * branch_spacing,
            ),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        sender.send(ProgramStatus::UpdateSim(id, format!("Simulating Branch {}/{}", 2 + (n * 2) + 1, branch_pair_count * 2), results.1, results.2, 0, 0)).unwrap();
        let mut res = branch(
            region,
            branch_length,
            branch_direction2,
            shift_coords(
                base_direction,
                shift_coords(branch_direction2, starting_coords, 1),
                (n + 1) * branch_spacing,
            ),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
    }

    return results;
}

pub fn branch_mining_with_poke_holes(
    region: &Region,
    base_direction: &Direction,
    starting_coords: (i32, i32, i32),
    branch_pair_count: i32,
    pokes_per_branch: i32,
    poke_spacing: i32,
    branch_spacing: i32,
    id: u32,
    sender: Sender<ProgramStatus>,
) -> (Vec<SimpleBlock>, u32, u32) {
    fn expand_corridor(
        region: &Region,
        direction: &Direction,
        branch_spacing: i32,
        coords: (i32, i32, i32),
    ) -> (Vec<SimpleBlock>, u32, u32) {
        let mut results = (Vec::new(), 0, 0);
        // The sides of the corridor that coincide with a branch along with the sides in front and behind. For these we don't consider the side blocks that would be exposed.
        for y in -1..3 {
            // Slice that coincides
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1 + y, coords.2),
                get_block(region, (coords.0, coords.1 + y, coords.2)),
            ));
            results.1 += 2;
            results.2 += 4;
            // Slice after
            results.0.push(SimpleBlock::new(
                shift_coords(direction, (coords.0, coords.1 + y, coords.2), 1),
                get_block(
                    region,
                    shift_coords(direction, (coords.0, coords.1 + y, coords.2), 1),
                ),
            ));
            results.1 += 2;
            results.2 += 4;
            // Slice before
            results.0.push(SimpleBlock::new(
                shift_coords(direction, (coords.0, coords.1 + y, coords.2), branch_spacing),
                get_block(
                    region,
                    shift_coords(direction, (coords.0, coords.1 + y, coords.2), branch_spacing),
                ),
            ));
            results.1 += 2;
            results.2 += 4;
        }

        let mut res = two_by_one_length(
            region,
            direction,
            shift_coords(direction, coords, 2),
            branch_spacing - 3,
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        return results;
    }

    fn branch(
        region: &Region,
        pokes_per_branch: i32,
        poke_spacing: i32,
        direction: &Direction,
        coords: (i32, i32, i32),
    ) -> (Vec<SimpleBlock>, u32, u32) {
        let mut results = (Vec::new(), 0, 0);
        for n in 0..pokes_per_branch {
            let (poke_direction1, poke_direction2) =
                if direction == &Direction::East || direction == &Direction::West {
                    (&Direction::North, &Direction::South)
                } else {
                    (&Direction::East, &Direction::West)
                };
            let coords = shift_coords(direction, coords, n * poke_spacing - 1);
            let mut res = poke(
                region,
                poke_direction1,
                shift_coords(poke_direction1, coords, 1),
                5,
            );
            results.0.append(&mut res.0);
            results.1 += res.1;
            results.2 += res.2;
            let mut res = poke(
                region,
                poke_direction2,
                shift_coords(poke_direction2, coords, 1),
                5,
            );
            results.0.append(&mut res.0);
            results.1 += res.1;
            results.2 += res.2;
        }
        let mut res = two_by_one_length(region, direction, coords, poke_spacing * pokes_per_branch);
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        let mut res = two_by_one_end(
            region,
            direction,
            shift_coords(direction, coords, poke_spacing * pokes_per_branch),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        return results;
    }

    let mut results = (Vec::new(), 0, 0);
    let (branch_direction1, branch_direction2) =
        if base_direction == &Direction::East || base_direction == &Direction::West {
            (&Direction::North, &Direction::South)
        } else {
            (&Direction::East, &Direction::West)
        };
    sender.send(ProgramStatus::UpdateSim(id, format!("Simulating Branch 1/{}", branch_pair_count * 2), 0, 0, 0, 0)).unwrap();
    let mut res = branch(
        region,
        pokes_per_branch,
        poke_spacing,
        branch_direction1,
        shift_coords(branch_direction1, starting_coords, 1),
    );
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    sender.send(ProgramStatus::UpdateSim(id, format!("Simulating Branch 2/{}", branch_pair_count * 2), results.1, results.2, 0, 0)).unwrap();
    let mut res = branch(
        region,
        pokes_per_branch,
        poke_spacing,
        branch_direction2,
        shift_coords(branch_direction2, starting_coords, 1),
    );
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    for n in 0..branch_pair_count - 1 {
        sender.send(ProgramStatus::UpdateSim(id, format!("Simulating Corridor"), results.1, results.2, 0, 0)).unwrap();
        let mut res = expand_corridor(
            region,
            base_direction,
            branch_spacing,
            shift_coords(base_direction, starting_coords, n * branch_spacing),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        sender.send(ProgramStatus::UpdateSim(id, format!("Simulating Branch {}/{}", 2 + (n * 2), branch_pair_count * 2), results.1, results.2, 0, 0)).unwrap();
        let mut res = branch(
            region,
            pokes_per_branch,
            poke_spacing,
            branch_direction1,
            shift_coords(
                base_direction,
                shift_coords(branch_direction1, starting_coords, 1),
                (n + 1) * branch_spacing,
            ),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        sender.send(ProgramStatus::UpdateSim(id, format!("Simulating Branch {}/{}", 2 + (n * 2) + 1, branch_pair_count * 2), results.1, results.2, 0, 0)).unwrap();
        let mut res = branch(
            region,
            pokes_per_branch,
            poke_spacing,
            branch_direction2,
            shift_coords(
                base_direction,
                shift_coords(branch_direction2, starting_coords, 1),
                (n + 1) * branch_spacing,
            ),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
    }

    return results;
}
