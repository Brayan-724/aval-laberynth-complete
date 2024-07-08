use plotters::prelude::*;

use crate::drawing::DrawingContext;
use crate::map::Map;
use crate::pos::Pos;
use crate::Cell;

const BACKGROUND: RGBColor = RGBColor(255 / 16, 255 / 16, 255 / 8);

pub fn print_map(map: &Map, ctx: &mut DrawingContext) -> Result<(), Box<dyn std::error::Error>> {
    ctx.fill(&BACKGROUND)?;

    let n = map.len();
    let mut s = String::with_capacity(n.pow(2) * 3);

    for (y, row) in map.iter_rows().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell != &Cell::Wall {
                let t_cell = map.get(Pos(x, y - 1));
                let t_border = matches!(
                    (cell, t_cell),
                    (Cell::Bot(_) | Cell::Path, Some(Cell::Best(_)))
                        | (Cell::Best(_), Some(Cell::Bot(_) | Cell::Path))
                        | (_, Some(Cell::Wall))
                );
                let r_cell = map.get(Pos(x + 1, y));
                let r_border = matches!(
                    (cell, r_cell),
                    (Cell::Bot(_) | Cell::Path, Some(Cell::Best(_)))
                        | (Cell::Best(_), Some(Cell::Bot(_) | Cell::Path))
                        | (_, Some(Cell::Wall))
                );
                let b_cell = map.get(Pos(x, y + 1));
                let b_border = matches!(
                    (cell, b_cell),
                    (Cell::Bot(_) | Cell::Path, Some(Cell::Best(_)))
                        | (Cell::Best(_), Some(Cell::Bot(_) | Cell::Path))
                        | (_, Some(Cell::Wall))
                );
                let l_cell = map.get(Pos(x - 1, y));
                let l_border = matches!(
                    (cell, l_cell),
                    (Cell::Bot(_) | Cell::Path, Some(Cell::Best(_)))
                        | (Cell::Best(_), Some(Cell::Bot(_) | Cell::Path))
                        | (_, Some(Cell::Wall))
                );

                cell_rectangle(
                    ctx,
                    map.len(),
                    Pos(x, y),
                    cell,
                    (t_border, r_border, b_border, l_border),
                )?;
            }

            match cell {
                Cell::Wall if x == 0 && y == 0 => s.push_str(" ┌"),
                Cell::Wall if x == 0 && y == n - 1 => s.push_str(" └"),
                Cell::Wall if x == n - 1 && y == 0 => s.push_str("┐ "),
                Cell::Wall if x == n - 1 && y == n - 1 => s.push_str("┘ "),

                Cell::Wall if x == 0 => s.push_str(" │"),
                Cell::Wall if x == n - 1 => s.push_str("│ "),
                Cell::Wall if y == 0 || y == n - 1 => s.push_str("──"),

                Cell::Wall => s.push_str("--"),
                Cell::Path => s.push_str("  "),

                Cell::Bot(cost) => s.push_str(&format!("{cost:0>2x}")),
                Cell::Island(cost) => s.push_str(&format!("{cost:0>2x}")),

                Cell::Best(_) => s.push_str("XX"),
            }

            s.push(' ');
        }

        s.push('\n');
    }

    print!("{s}\n\n");
    Ok(())
}

const PLAIN_ST: f32 = 0.0;
const PLAIN_EN: f32 = 1.0;

const ROUND_ST: f32 = 0.25;
const ROUND_EN: f32 = 0.75;

fn cell_rectangle(
    chart: &mut DrawingContext,
    size: usize,
    pos: Pos,
    cell: &Cell,

    (t_border, r_border, b_border, l_border): (bool, bool, bool, bool),
) -> Result<(), Box<dyn std::error::Error>> {
    let x = pos.0 as f32;
    let y = pos.1 as f32;

    let color = match cell {
        Cell::Wall => BACKGROUND,
        Cell::Path => RGBColor(255, 255, 255),
        Cell::Bot(cost) => {
            let max = ((size.pow(2) * 2) as f32).sqrt() as f32;
            let cost = (200f32 * (*cost as f32 / max)) as u8;
            RGBColor(cost, cost, cost)
        }
        Cell::Best(_) => RGBColor(255 / 2, 255 / 3, 255 / 2),
        Cell::Island(id) => {
            let id = id + 1;
            let r_factor = ((id / 3) % 3) + 1;
            let g_factor = ((id / 2) % 3) + 1;
            let b_factor = ((id / 1) % 3) + 1;

            RGBColor(255 / r_factor, 255 / g_factor, 255 / b_factor)
        }
    };

    #[rustfmt::skip]
    let points = vec![
        if t_border { (x + ROUND_ST, y + PLAIN_ST) } else { (x + PLAIN_ST, y + PLAIN_ST) },
        if t_border { (x + ROUND_EN, y + PLAIN_ST) } else { (x + PLAIN_EN, y + PLAIN_ST) },

        if r_border { (x + PLAIN_EN, y + ROUND_ST) } else { (x + PLAIN_EN, y + PLAIN_ST) },
        if r_border { (x + PLAIN_EN, y + ROUND_EN) } else { (x + PLAIN_EN, y + PLAIN_EN) },

        if b_border { (x + ROUND_EN, y + PLAIN_EN) } else { (x + PLAIN_EN, y + PLAIN_EN) },
        if b_border { (x + ROUND_ST, y + PLAIN_EN) } else { (x + PLAIN_ST, y + PLAIN_EN) },

        if l_border { (x + PLAIN_ST, y + ROUND_EN) } else { (x + PLAIN_ST, y + PLAIN_EN) },
        if l_border { (x + PLAIN_ST, y + ROUND_ST) } else { (x + PLAIN_ST, y + PLAIN_ST) },
    ];

    chart.draw(Polygon::new(points, color.filled()))?;

    Ok(())
}

pub fn draw_line(
    chart: &mut DrawingContext,
    pos: (f32, f32),
    horizontal: bool,
    width: f32,
    stroke: f32,
    style: impl Into<ShapeStyle>,
) -> Result<(), Box<dyn std::error::Error>> {
    let w = stroke / 2.;
    let m = stroke / 4.;

    if horizontal {
        chart.draw(Rectangle::new(
            [(pos.0 - m, pos.1 - w), (pos.0 + width + m, pos.1 + w)],
            style,
        ))?;
    } else {
        chart.draw(Rectangle::new(
            [(pos.0 - w, pos.1 - m), (pos.0 + w, pos.1 + width + m)],
            style,
        ))?;
    }

    Ok(())
}
