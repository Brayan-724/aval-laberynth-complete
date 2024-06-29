use std::collections::HashMap;

use plotters::style::full_palette::BLUE_500;
use plotters::style::Color;

use crate::drawing::DrawingContext;
use crate::map::{Cell, Map};
use crate::pos::Pos;
use crate::utils::{draw_line, print_map};

pub fn resolve_it(
    map: &mut Map,
    ctx: &mut DrawingContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let n = map.len();
    let mut mesh: HashMap<Pos, u8> = HashMap::new();

    let mut ways: Vec<(u8, Pos, Vec<Pos>)> = vec![(0, Pos(1, 1), vec![])];
    let mut cheapest_way: Option<Vec<Pos>> = None;

    loop {
        let mut best_way: Option<(u8, Vec<Pos>)> = None;

        let a = ways.pop();

        if let Some((way_cost, way_pos, mut way_tail)) = a {
            if let Some(best_way_) = best_way {
                if best_way_.0 > way_cost {
                    best_way = Some((way_cost, way_tail.clone()));
                } else {
                    best_way = Some(best_way_);
                }
            } else {
                best_way = Some((way_cost, way_tail.clone()));
            }

            if let Some(best_way) = &best_way {
                for cell in &best_way.1 {
                    map.set_best(*cell, true);
                }
            }

            print_map(map, ctx)?;

            if let Some(best_way) = &best_way {
                for cell in &best_way.1 {
                    map.set_best(*cell, false);
                }
            }

            way_tail.push(way_pos);

            if way_pos.0 == n - 2 && way_pos.1 == n - 2 {
                cheapest_way = Some(way_tail);
                ctx.present()?;

                break;
            }

            process_nb(
                ctx,
                map,
                &mut mesh,
                &mut ways,
                way_cost,
                way_pos,
                &way_tail,
                way_pos + Pos(1, 0),
            )?;

            process_nb(
                ctx,
                map,
                &mut mesh,
                &mut ways,
                way_cost,
                way_pos,
                &way_tail,
                way_pos - Pos(1, 0),
            )?;

            process_nb(
                ctx,
                map,
                &mut mesh,
                &mut ways,
                way_cost,
                way_pos,
                &way_tail,
                way_pos + Pos(0, 1),
            )?;

            process_nb(
                ctx,
                map,
                &mut mesh,
                &mut ways,
                way_cost,
                way_pos,
                &way_tail,
                way_pos - Pos(0, 1),
            )?;

            ctx.present()?;
        }

        if ways.is_empty() {
            break;
        }
    }

    println!("{cheapest_way:#?}");

    if let Some(cheapest_way) = cheapest_way {
        for cell in &cheapest_way {
            map.set_best(*cell, true);
        }
    }

    print_map(map, ctx)?;
    ctx.present()?;

    Ok(())
}

fn process_nb(
    ctx: &mut DrawingContext,
    map: &mut Map,
    mesh: &mut HashMap<Pos, u8>,
    ways: &mut Vec<(u8, Pos, Vec<Pos>)>,
    way_cost: u8,
    way_pos: Pos,
    way_tail: &Vec<Pos>,
    nb_pos: Pos,
) -> Result<(), Box<dyn std::error::Error>> {
    draw_line(
        ctx,
        (way_pos.0 as f32 + 0.6, way_pos.1 as f32 + 0.5),
        true,
        1.0,
        0.4,
        BLUE_500.filled(),
    )?;

    if let Some(nb_cost) = mesh.get(&nb_pos) {
        if *nb_cost > way_cost + 1 {
            let new_cost = way_cost + 1;

            mesh.insert(nb_pos, new_cost);
            map.set_unchecked(nb_pos, Cell::Bot(new_cost));

            match ways.binary_search_by(|w| new_cost.cmp(&w.0)) {
                Ok(idx) | Err(idx) => ways.insert(
                    idx.saturating_add_signed(-1),
                    (new_cost, nb_pos, way_tail.clone()),
                ),
            }
        }
    } else {
        let size = map.len();
        let new_distance =
            (((size - nb_pos.0).pow(2) + (size - nb_pos.1).pow(2)) as f32).sqrt() as u8;
        let new_cost = new_distance;
        // let new_cost = (way_tail.len() as u8) + new_distance;

        if map.set_unchecked_bot(nb_pos, new_cost) {
            mesh.insert(nb_pos, new_cost);

            match ways.binary_search_by(|w| new_cost.cmp(&w.0)) {
                Ok(idx) | Err(idx) => ways.insert(
                    idx.saturating_add_signed(-1),
                    (new_cost, nb_pos, way_tail.clone()),
                ),
            }
        }
    }

    Ok(())
}
