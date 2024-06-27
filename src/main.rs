extern crate plotters;
extern crate plotters_backend;
extern crate rand;

mod map;
mod utils;

use map::{Cell, Map, Pos, WaySide, WayVariants};
use utils::print_map;

use rand::rngs::ThreadRng;
use rand::Rng;

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

fn generate_laberynth(map: &mut Map, r: &mut ThreadRng) {
    let n = map.len();

    map.set_unchecked((1, 1), Cell::Bot);
    map.set_unchecked((n - 2, n - 2), Cell::Bot);

    let start_exit = r.gen_bool(0.5);
    let end_exit = r.gen_bool(0.5);

    let start_pos = if start_exit {
        map.set_unchecked((1, 2), Cell::Bot);
        map.set_unchecked((2, 1), Cell::Wall);

        (WaySide::South, (1, 2))
    } else {
        map.set_unchecked((1, 2), Cell::Wall);
        map.set_unchecked((2, 1), Cell::Bot);

        (WaySide::East, (2, 1))
    };

    if end_exit {
        map.set_unchecked((n - 2, n - 3), Cell::Bot);
        map.set_unchecked((n - 3, n - 2), Cell::Wall);
    } else {
        map.set_unchecked((n - 2, n - 3), Cell::Wall);
        map.set_unchecked((n - 3, n - 2), Cell::Bot);
    }

    let mut start_ways: Vec<(WaySide, Pos)> = vec![start_pos];

    for _ in 0..5 {
        let ways = start_ways.clone();
        start_ways = vec![];

        for (side, way_pos) in ways {
            let variant = WayVariants::get_random(r);

            println!("{variant:#?}: {start_ways:?}");

            for new_way in map.set_way(way_pos, side, variant).into_iter() {
                start_ways.push(new_way);
            }

            print_map(map);
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    // for x in 1..n - 1 {
    //     for y in 1..n - 1 {
    //         if x == 1 && y == 1 {
    //             continue;
    //         }
    //
    //         let cell = if r.gen_bool(1.0 / 2.0) {
    //             Cell::Wall
    //         } else {
    //             Cell::Path
    //         };
    //
    //         map.set_unchecked((x, y), cell);
    //     }
    // }
}

fn init_map(n: usize) -> Map {
    let mut map = Map::new(n);

    for y in 0..n {
        let row = map.get_unchecked_row_mut(y);

        row[0] = Cell::Wall;
        row[n - 1] = Cell::Wall;
    }

    for x in 0..n {
        map.set_unchecked((0, x), Cell::Wall);
        map.set_unchecked((n - 1, x), Cell::Wall);
    }

    map
}
