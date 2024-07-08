extern crate plotters;
extern crate plotters_backend;
extern crate rand;

mod drawing;
mod fixer;
mod generate;
mod map;
mod pos;
mod resolve;
mod utils;

use drawing::DrawingContext;
use fixer::fix_it;
use generate::generate_laberynth;
use map::{Cell, Map};
use resolve::resolve_it;

use utils::print_map;

use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut r = rand::thread_rng();

    #[allow(non_snake_case)]
    let N: usize = {
        // Random an odd map size
        // let n = r.gen_range(13..16);
        let n = 16 * 3;

        if n % 2 == 1 {
            n
        } else {
            n + 1
        }
    };

    let root = if let Ok(file) = std::env::var("OUTPUT_FILE") {
        Some(DrawingContext::create_root(file, 1000)?)
    } else {
        None
    };

    let mut ctx = if let Some(root) = &root {
        DrawingContext::new_gif(root, N as f32)?
    } else {
        DrawingContext::NoDraw
    };

    let mut map = Map::new(N);

    print_map(&mut map, &mut ctx)?;
    ctx.present()?;

    generate_laberynth(&mut map, &mut r, &mut ctx)?;
    print_map(&mut map, &mut ctx)?;
    ctx.present()?;

    fix_it(&mut map, &mut r, &mut ctx)?;
    print_map(&mut map, &mut ctx)?;
    ctx.present()?;

    resolve_it(&mut map, &mut ctx)?;
    print_map(&mut map, &mut ctx)?;
    ctx.present()?;

    ctx.present()?;

    Ok(())
}
