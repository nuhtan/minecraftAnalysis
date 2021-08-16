use mvp_anvil::region::Region;

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
}

pub fn branch_mining(
    region: Region,
    base_direction: Direction,
    starting_coords: (i32, i32, i32),
    branch_pair_count: i32,
    branch_length: i32,
    branch_spacing: i32,
) -> (Vec<SimpleBlock>, u32, u32) {
    if branch_spacing < 2 {
        panic!("Branch spacing should be at least two to avoid duplicates")
    }

    fn expand_corridor(
        region: Region,
        direction: Direction,
        branch_spacing: i32,
        coords: (i32, i32, i32),
    ) -> (Vec<SimpleBlock>, u32, u32) {
        let mut results = (Vec::new(), 0, 0);
        let loop_dir = direction.clone();
        // The sides of the corridor that coincide with a branch along with the sides in front and behind. For these we don't consider the side blocks that would be exposed.
        for y in -1..3 {
            let region = region.clone();
            let direction = direction.clone();
            // Slice that coincides
            let co_reg = region.clone();
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1 + y, coords.2),
                get_block(co_reg, (coords.0, coords.1 + y, coords.2)),
            ));
            results.1 += 2;
            results.2 += 4;
            // Slice after
            let aft_dir1 = direction.clone();
            let aft_dir2 = direction.clone();
            let aft_reg = region.clone();
            results.0.push(SimpleBlock::new(
                shift_coords(aft_dir1, (coords.0, coords.1 + y, coords.2), 1),
                get_block(
                    aft_reg,
                    shift_coords(aft_dir2, (coords.0, coords.1 + y, coords.2), 1),
                ),
            ));
            results.1 += 2;
            results.2 += 4;
            // Slice before
            let bef_dir1 = direction.clone();
            let bef_dir2 = direction.clone();
            let bef_reg = region.clone();
            results.0.push(SimpleBlock::new(
                shift_coords(bef_dir1, (coords.0, coords.1 + y, coords.2), branch_spacing),
                get_block(
                    bef_reg,
                    shift_coords(bef_dir2, (coords.0, coords.1 + y, coords.2), branch_spacing),
                ),
            ));
            results.1 += 2;
            results.2 += 4;
        }

        let mut res = two_by_one_length(
            region,
            direction,
            shift_coords(loop_dir, coords, 2),
            branch_spacing - 3,
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        return results;
    }

    fn branch(
        region: Region,
        branch_length: i32,
        direction: Direction,
        coords: (i32, i32, i32),
    ) -> (Vec<SimpleBlock>, u32, u32) {
        let mut results = (Vec::new(), 0, 0);
        let region2 = region.clone();
        let direction2a = direction.clone();
        let direction2b = direction.clone();

        let mut res = two_by_one_length(region, direction, coords, branch_length);
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;

        let mut res = two_by_one_end(
            region2,
            direction2a,
            shift_coords(direction2b, coords, branch_length - 1),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;

        return results;
    }

    let mut results = (Vec::new(), 0, 0);
    let (branch_direction1, branch_direction2) =
        if base_direction == Direction::East || base_direction == Direction::West {
            (Direction::North, Direction::South)
        } else {
            (Direction::East, Direction::West)
        };
    let region_starting1 = region.clone();
    let region_starting2 = region.clone();
    let b_dir1_1 = branch_direction1.clone();
    let b_dir2_1 = branch_direction2.clone();
    let b_dir1_2 = branch_direction1.clone();
    let b_dir2_2 = branch_direction2.clone();
    let mut res = branch(
        region_starting1,
        branch_length,
        b_dir1_1,
        shift_coords(b_dir1_2, starting_coords, 1),
    );
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    let mut res = branch(
        region_starting2,
        branch_length,
        b_dir2_1,
        shift_coords(b_dir2_2, starting_coords, 1),
    );
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    for n in 0..branch_pair_count - 1 {
        let region = region.clone();
        let base_direction = base_direction.clone();
        let base_dir_exp = base_direction.clone();
        let dir1_reg = region.clone();
        let dir2_reg = region.clone();
        let dir1_bd = base_direction.clone();
        let dir2_bd = base_direction.clone();
        let dir1_bd1_1 = branch_direction1.clone();
        let dir1_bd1_2 = branch_direction1.clone();
        let dir1_bd2_1 = branch_direction2.clone();
        let dir1_bd2_2 = branch_direction2.clone();
        let mut res = expand_corridor(
            region,
            base_direction,
            branch_spacing,
            shift_coords(base_dir_exp, starting_coords, n * branch_spacing),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        let mut res = branch(
            dir1_reg,
            branch_length,
            dir1_bd1_1,
            shift_coords(
                dir1_bd,
                shift_coords(dir1_bd1_2, starting_coords, 1),
                n * branch_spacing,
            ),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        let mut res = branch(
            dir2_reg,
            branch_length,
            dir1_bd2_1,
            shift_coords(
                dir2_bd,
                shift_coords(dir1_bd2_2, starting_coords, 1),
                n * branch_spacing,
            ),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
    }

    return results;
}

pub fn branch_mining_with_poke_holes(
    region: Region,
    base_direction: Direction,
    starting_coords: (i32, i32, i32),
    branch_pair_count: i32,
    pokes_per_branch: i32,
    poke_spacing: i32,
    branch_spacing: i32,
) -> (Vec<SimpleBlock>, u32, u32) {
    fn expand_corridor(
        region: Region,
        direction: Direction,
        branch_spacing: i32,
        coords: (i32, i32, i32),
    ) -> (Vec<SimpleBlock>, u32, u32) {
        let mut results = (Vec::new(), 0, 0);
        let loop_dir = direction.clone();
        // The sides of the corridor that coincide with a branch along with the sides in front and behind. For these we don't consider the side blocks that would be exposed.
        for y in -1..3 {
            let region = region.clone();
            let direction = direction.clone();
            // Slice that coincides
            let co_reg = region.clone();
            results.0.push(SimpleBlock::new(
                (coords.0, coords.1 + y, coords.2),
                get_block(co_reg, (coords.0, coords.1 + y, coords.2)),
            ));
            results.1 += 2;
            results.2 += 4;
            // Slice after
            let aft_dir1 = direction.clone();
            let aft_dir2 = direction.clone();
            let aft_reg = region.clone();
            results.0.push(SimpleBlock::new(
                shift_coords(aft_dir1, (coords.0, coords.1 + y, coords.2), 1),
                get_block(
                    aft_reg,
                    shift_coords(aft_dir2, (coords.0, coords.1 + y, coords.2), 1),
                ),
            ));
            results.1 += 2;
            results.2 += 4;
            // Slice before
            let bef_dir1 = direction.clone();
            let bef_dir2 = direction.clone();
            let bef_reg = region.clone();
            results.0.push(SimpleBlock::new(
                shift_coords(bef_dir1, (coords.0, coords.1 + y, coords.2), branch_spacing),
                get_block(
                    bef_reg,
                    shift_coords(bef_dir2, (coords.0, coords.1 + y, coords.2), branch_spacing),
                ),
            ));
            results.1 += 2;
            results.2 += 4;
        }

        let mut res = two_by_one_length(
            region,
            direction,
            shift_coords(loop_dir, coords, 2),
            branch_spacing - 3,
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        return results;
    }

    fn branch(
        region: Region,
        pokes_per_branch: i32,
        poke_spacing: i32,
        direction: Direction,
        coords: (i32, i32, i32),
    ) -> (Vec<SimpleBlock>, u32, u32) {
        let mut results = (Vec::new(), 0, 0);
        for n in 0..pokes_per_branch {
            let loop_region1 = region.clone();
            let loop_region2 = region.clone();
            let loop_direction = direction.clone();
            let (poke_direction1, poke_direction2) =
                if loop_direction == Direction::East || loop_direction == Direction::West {
                    (Direction::North, Direction::South)
                } else {
                    (Direction::East, Direction::West)
                };
            let poke_direction12 = poke_direction1.clone();
            let poke_direction22 = poke_direction2.clone();
            let coords = shift_coords(loop_direction, coords, n * poke_spacing - 1);
            let mut res = poke(
                loop_region1,
                poke_direction1,
                shift_coords(poke_direction12, coords, 1),
                5,
            );
            results.0.append(&mut res.0);
            results.1 += res.1;
            results.2 += res.2;
            let mut res = poke(
                loop_region2,
                poke_direction2,
                shift_coords(poke_direction22, coords, 1),
                5,
            );
            results.0.append(&mut res.0);
            results.1 += res.1;
            results.2 += res.2;
        }
        let end_region = region.clone();
        let end_direction = direction.clone();
        let end_direction2 = direction.clone();
        let mut res = two_by_one_length(region, direction, coords, poke_spacing * pokes_per_branch);
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        let mut res = two_by_one_end(
            end_region,
            end_direction,
            shift_coords(end_direction2, coords, poke_spacing * pokes_per_branch),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        return results;
    }

    let mut results = (Vec::new(), 0, 0);
    let (branch_direction1, branch_direction2) =
        if base_direction == Direction::East || base_direction == Direction::West {
            (Direction::North, Direction::South)
        } else {
            (Direction::East, Direction::West)
        };
    let region_starting1 = region.clone();
    let region_starting2 = region.clone();
    let b_dir1_1 = branch_direction1.clone();
    let b_dir2_1 = branch_direction2.clone();
    let b_dir1_2 = branch_direction1.clone();
    let b_dir2_2 = branch_direction2.clone();
    let mut res = branch(
        region_starting1,
        pokes_per_branch,
        poke_spacing,
        b_dir1_1,
        shift_coords(b_dir1_2, starting_coords, 1),
    );
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    let mut res = branch(
        region_starting2,
        pokes_per_branch,
        poke_spacing,
        b_dir2_1,
        shift_coords(b_dir2_2, starting_coords, 1),
    );
    results.0.append(&mut res.0);
    results.1 += res.1;
    results.2 += res.2;
    for n in 0..branch_pair_count - 1 {
        let region = region.clone();
        let base_direction = base_direction.clone();
        let base_dir_exp = base_direction.clone();
        let dir1_reg = region.clone();
        let dir2_reg = region.clone();
        let dir1_bd = base_direction.clone();
        let dir2_bd = base_direction.clone();
        let dir1_bd1_1 = branch_direction1.clone();
        let dir1_bd1_2 = branch_direction1.clone();
        let dir1_bd2_1 = branch_direction2.clone();
        let dir1_bd2_2 = branch_direction2.clone();
        let mut res = expand_corridor(
            region,
            base_direction,
            branch_spacing,
            shift_coords(base_dir_exp, starting_coords, n * branch_spacing),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        let mut res = branch(
            dir1_reg,
            pokes_per_branch,
            poke_spacing,
            dir1_bd1_1,
            shift_coords(
                dir1_bd,
                shift_coords(dir1_bd1_2, starting_coords, 1),
                n * branch_spacing,
            ),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
        let mut res = branch(
            dir2_reg,
            pokes_per_branch,
            poke_spacing,
            dir1_bd2_1,
            shift_coords(
                dir2_bd,
                shift_coords(dir1_bd2_2, starting_coords, 1),
                n * branch_spacing,
            ),
        );
        results.0.append(&mut res.0);
        results.1 += res.1;
        results.2 += res.2;
    }

    return results;
}
