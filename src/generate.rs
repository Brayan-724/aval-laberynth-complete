use map::{Cell, Map};
use plotters::style::full_palette::BLUE_500;
use plotters::style::Color;
use pos::Pos;
use utils::print_map;

use rand::rngs::ThreadRng;
use rand::Rng;

use crate::drawing::DrawingContext;
use crate::utils::draw_line;

pub fn generate_laberynth(
    map: &mut Map,
    r: &mut ThreadRng,
    ctx: &mut DrawingContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let n = map.len();
    let mesh_size = (n - 2).div_ceil(2);
    let mut mesh: Vec<Vec<(u8, Pos)>> = Vec::with_capacity(mesh_size);

    for y in 0..mesh_size {
        let mut row = Vec::with_capacity(mesh_size);

        for x in 0..mesh_size {
            let weight = r.gen_range(1..255);
            let pos = Pos(x * 2 + 1, y * 2 + 1);

            let node = (weight, pos);

            row.push(node);
            map.set_unchecked(pos, Cell::Path)
        }

        mesh.push(row);
    }

    for y in 0..mesh_size {
        for x in 0..mesh_size {
            let pos = Pos(x, y);
            let self_node = &mesh[pos.1][pos.0].1;

            let mut farest_nb: Option<(u8, (isize, isize))> = None;

            let x_diff = if x > mesh_size / 2 { (-1, 0) } else { (1, 0) };
            let y_diff = if y > mesh_size / 2 { (0, -1) } else { (0, 1) };

            print_map(map, ctx)?;

            if let Some(nb) = get_neighbor(&mesh, pos, x_diff) {
                draw_line(
                    ctx,
                    (self_node.0 as f32 + 0.5, self_node.1 as f32 + 0.5),
                    true,
                    x_diff.0 as f32 * (nb.0 as f32 / 128.),
                    0.3,
                    BLUE_500.filled(),
                )?;

                farest_nb = Some(nb)
            }

            if let Some(nb) = get_neighbor(&mesh, pos, y_diff) {
                draw_line(
                    ctx,
                    (self_node.0 as f32 + 0.5, self_node.1 as f32 + 0.5),
                    false,
                    y_diff.1 as f32 * (nb.0 as f32 / 128.),
                    0.3,
                    BLUE_500.filled(),
                )?;

                if let Some(nearby_nb) = &farest_nb {
                    if nearby_nb.0 < nb.0 {
                        farest_nb = Some(nb)
                    }
                } else {
                    farest_nb = Some(nb)
                }
            }

            if x % 5 == 0 {
                ctx.present()?;
            }

            if let Some((_, rel_pos)) = farest_nb {
                if let Some(new_pos) = &self_node.checked_add(rel_pos) {
                    map.set_unchecked(*new_pos, Cell::Path);

                    if x % 3 == 0 {
                        print_map(map, ctx)?;
                        ctx.present()?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn get_neighbor(
    mesh: &Vec<Vec<(u8, Pos)>>,
    pos: Pos,
    rel_pos: (isize, isize),
) -> Option<(u8, (isize, isize))> {
    let mesh_size = mesh.len();

    let mesh_pos = pos.checked_add(rel_pos)?;
    if mesh_pos.0 >= mesh_size || mesh_pos.1 >= mesh_size {
        return None;
    }

    let w = mesh[mesh_pos.1][mesh_pos.0].0;

    Some((w, rel_pos))
}
