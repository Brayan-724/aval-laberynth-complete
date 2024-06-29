use std::collections::HashMap;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use crate::drawing::DrawingContext;
use crate::map::{Cell, Map};
use crate::pos::Pos;
use crate::utils::print_map;

pub fn fix_it(
    mut map: &mut Map,
    r: &mut ThreadRng,
    ctx: &mut DrawingContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let n = map.len();
    let mut mesh: HashMap<Pos, u8> = HashMap::new();

    let mut island_id = 0;

    // Bridge Position, Bridge Target
    let mut islands: Vec<Vec<(Pos, Pos)>> = vec![];

    let mut last_cell = Pos(0, 1);

    loop {
        let start_cell = last_cell + Pos(1, 0);

        // Is in the last cell?
        if start_cell == Pos(n - 2, n - 2) {
            break;
        }

        // Wrap it
        if start_cell.0 == n - 2 {
            last_cell = Pos(0, start_cell.1 + 1);
            continue;
        }

        // Continue from this cell
        last_cell = start_cell;

        // It is already checked
        if mesh.contains_key(&start_cell) {
            continue;
        }

        if *map.get_unchecked(start_cell) == Cell::Wall {
            continue;
        }

        mesh.insert(start_cell, island_id);
        map.set_unchecked(start_cell, Cell::Island(island_id));

        let mut island: Vec<Pos> = vec![start_cell];
        let mut bridges: Vec<(Pos, Pos)> = vec![];
        let mut ways: Vec<Pos> = vec![start_cell];

        let mut i = 0u8;
        loop {
            let last_way = ways.pop();

            if let Some(way_pos) = last_way {
                let mut ctx = (&mut map, &mut mesh, &mut ways, island_id, &mut island);

                if process_nb(&mut ctx, way_pos + Pos(1, 0)) {
                    bridges.push((way_pos + Pos(0, 1), way_pos + Pos(0, 2)));
                }

                if process_nb(&mut ctx, way_pos - Pos(1, 0)) {
                    bridges.push((way_pos - Pos(0, 1), way_pos - Pos(0, 2)));
                }

                if process_nb(&mut ctx, way_pos + Pos(0, 1)) {
                    bridges.push((way_pos + Pos(0, 1), way_pos + Pos(0, 2)));
                }

                if process_nb(&mut ctx, way_pos - Pos(0, 1)) {
                    bridges.push((way_pos - Pos(0, 1), way_pos - Pos(0, 2)));
                }
            }

            if i % 10 == 0 {
                print_map(map, ctx)?;
                ctx.present()?;
                i = 1;
            } else {
                i += 1;
            }

            if ways.is_empty() {
                println!("{bridges:?}");
                break;
            }
        }

        print_map(map, ctx)?;
        ctx.present()?;

        islands.push(bridges);

        island_id += 1;
    }

    let mut island_bridges: Vec<Vec<(Pos, Pos)>> = vec![];

    for (island_id, bridges) in islands.iter().enumerate() {
        let island_id = island_id as u8;
        let mut actual_bridges = vec![];

        for (bridge_pos, bridge_target) in bridges {
            let Some(target_island) = mesh.get(bridge_target) else {
                continue;
            };

            if *target_island == island_id {
                continue;
            }

            actual_bridges.push((*bridge_pos, *bridge_target));
        }

        island_bridges.push(actual_bridges);
    }

    for (island_id, mut bridges) in island_bridges.into_iter().enumerate() {
        let island_id = island_id as u8;
        bridges.shuffle(r);

        loop {
            let bridge = bridges.pop();

            if let Some((bridge_pos, bridge_target)) = bridge {
                if *map.get_unchecked(bridge_pos) == Cell::Path {
                    continue;
                }

                let Some(target_island) = mesh.get(&bridge_target) else {
                    continue;
                };
                let target_island = *target_island;

                if target_island == island_id {
                    continue;
                }

                for c in mesh.iter_mut() {
                    if *c.1 == island_id {
                        *c.1 = target_island;
                        map.set_unchecked(*c.0, Cell::Island(target_island));
                    }
                }

                map.set_unchecked(bridge_pos, Cell::Path);

                print_map(map, ctx)?;
                ctx.present()?;
                ctx.present()?;

                break;
            } else {
                break;
            }
        }
    }

    print_map(map, ctx)?;
    ctx.present()?;

    for r in map.iter_rows_mut() {
        for c in r.iter_mut() {
            if matches!(c, Cell::Island(_)) {
                *c = Cell::Path;
            }
        }
    }

    Ok(())
}

fn process_nb(
    ctx: &mut (
        &mut &mut Map,
        &mut HashMap<Pos, u8>,
        &mut Vec<Pos>,
        u8,
        &mut Vec<Pos>,
    ),

    nb_pos: Pos,
) -> bool {
    let map = &mut ctx.0;
    let mesh = &mut ctx.1;
    let ways = &mut ctx.2;
    let island_id = ctx.3;
    let island = &mut ctx.4;

    if !mesh.contains_key(&nb_pos) {
        if map.set_free_island(nb_pos, island_id) {
            mesh.insert(nb_pos, island_id);
            island.push(nb_pos);
            ways.push(nb_pos);
        } else {
            return true;
        }
    }

    false
}
