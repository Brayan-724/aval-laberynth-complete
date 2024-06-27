use std::time::Duration;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::{utils::print_map, Cell, Diff};

pub struct Bot {
    all_points: Vec<(usize, usize)>,
    ways: Vec<(usize, usize)>,
}

impl Bot {
    pub fn new(pos: (usize, usize)) -> Self {
        Self {
            all_points: vec![pos],
            ways: vec![pos],
        }
    }

    /// Returns if is done
    pub fn update(&mut self, map: &mut Vec<Vec<Cell>>, r: &mut ThreadRng, other: &Self) -> bool {
        if self.ways.is_empty() {
            return true;
        }

        let ways = self.ways.clone();
        self.ways = vec![];

        for pos in ways {
            let mut cases = [
                Diff::XPositive,
                Diff::XNegative,
                Diff::YPositive,
                Diff::YNegative,
            ];
            cases.shuffle(r);
            let diffurcate: u8 = r.gen_range(0..3);

            let mut n = 0;
            let mut case = 0usize;
            'explore: loop {
                if case >= cases.len() {
                    break 'explore;
                }

                let diff = &cases[case];
                case += 1;

                let new_pos = get_new_pos(pos, diff);

                if unsafe { &map.get_unchecked_mut(new_pos.1)[new_pos.0] } != &Cell::Path
                    || self.ways.contains(&new_pos)
                    || self.all_points.contains(&new_pos)
                    || other.all_points.contains(&new_pos)
                    || new_pos.0 == 0
                    || new_pos.1 == 0
                    || new_pos.0 == map.len() - 2
                    || new_pos.1 == map.len() - 2
                {
                    continue 'explore;
                }

                let (wall1, wall2) = match diff {
                    Diff::XPositive | Diff::XNegative => ((pos.0, pos.1 + 1), (pos.0, pos.1 - 1)),
                    Diff::YPositive | Diff::YNegative => ((pos.0 + 1, pos.1), (pos.0 - 1, pos.1)),
                };

                unsafe {
                    let wall1 = map.get_unchecked_mut(wall1.1).get_unchecked_mut(wall1.0);
                    if *wall1 == Cell::Path {
                        *wall1 = Cell::Wall;
                    }

                    let wall2 = map.get_unchecked_mut(wall2.1).get_unchecked_mut(wall2.0);
                    if *wall2 == Cell::Path {
                        *wall2 = Cell::Wall;
                    }

                    map.get_unchecked_mut(new_pos.1)[new_pos.0] = Cell::Bot;
                }

                self.all_points.push(new_pos);
                self.ways.push(new_pos);

                n += 1;

                if n >= diffurcate {
                    println!("Ended");
                    break 'explore;
                }

                print_map(map);
                println!("{:?}", self.ways);
                std::thread::sleep(Duration::from_millis(500));
            }
        }

        self.ways.is_empty()
    }
}

fn get_new_pos(pos: (usize, usize), dir: &Diff) -> (usize, usize) {
    match dir {
        Diff::XPositive => (pos.0 + 1, pos.1),
        Diff::XNegative => (pos.0 - 1, pos.1),
        Diff::YPositive => (pos.0, pos.1 + 1),
        Diff::YNegative => (pos.0, pos.1 - 1),
    }
}
