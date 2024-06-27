extern crate plotters;
extern crate plotters_backend;
extern crate rand;

mod utils;
mod worker_bot;

use utils::print_map;

use rand::rngs::ThreadRng;
use rand::Rng;
use worker_bot::Bot;

#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Path,
    Wall,
    Bot,
}

enum Diff {
    XPositive,
    XNegative,
    YPositive,
    YNegative,
}

// const OUT_FILE_NAME: &str = "output.gif";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut r = rand::thread_rng();

    #[allow(non_snake_case)]
    let N: usize = r.gen_range(14..16);

    let mut map = init_map(N);
    print_map(&map);

    generate_laberynth(&mut map, &mut r);
    print_map(&map);

    Ok(())
}

fn generate_laberynth(map: &mut Vec<Vec<Cell>>, r: &mut ThreadRng) {
    let n = map.len();

    let mut bot_start = Bot::new((1, 1));
    let mut bot_end = Bot::new((n - 3, n - 3));

    print_map(map);

    loop {
        let a_ended = bot_start.update(map, r, &bot_end);
        if a_ended {
            println!("Ended from start");
        } else {
            print_map(map);
        }

        let b_ended = bot_end.update(map, r, &bot_start);
        if b_ended {
            println!("Ended from end");
        } else {
            print_map(map);
        }

        if a_ended && b_ended {
            break;
        }
    }
}

fn init_map(n: usize) -> Vec<Vec<Cell>> {
    let mut map = Vec::with_capacity(n);

    for _ in 0..n {
        let mut row = Vec::with_capacity(n);

        for _ in 0..n {
            row.push(Cell::Path)
        }

        map.push(row);
    }

    unsafe {
        for y in 0..n {
            let row = map.get_unchecked_mut(y);

            row[0] = Cell::Wall;
            row[n - 1] = Cell::Wall;
        }

        for x in 0..n {
            map.get_unchecked_mut(0)[x] = Cell::Wall;
            map.get_unchecked_mut(n - 1)[x] = Cell::Wall;
        }
    }

    map
}
