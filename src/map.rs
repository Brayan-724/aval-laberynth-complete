use crate::pos::Pos;

#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Path,
    Wall,
    Bot(u8),
    Best(u8),
    Island(u8),
}

pub struct Map {
    size: usize,
    content: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(size: usize) -> Self {
        let mut map = Vec::with_capacity(size);

        for _ in 0..size {
            let mut row = Vec::with_capacity(size);

            for _ in 0..size {
                row.push(Cell::Wall)
            }

            map.push(row);
        }

        Self { content: map, size }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &Vec<Cell>> {
        self.content.iter()
    }

    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut Vec<Cell>> {
        self.content.iter_mut()
    }

    pub fn set_unchecked(&mut self, pos: Pos, cell: Cell) {
        *self.content.get_mut(pos.1).unwrap().get_mut(pos.0).unwrap() = cell;
        // unsafe {
        //     self.content.get_unchecked_mut(pos.1)[pos.0] = cell;
        // }
    }

    // pub fn get_unchecked_row_mut(&mut self, y: usize) -> &mut Vec<Cell> {
    //     unsafe { self.content.get_unchecked_mut(y) }
    // }

    pub fn get(&self, pos: Pos) -> Option<&Cell> {
        self.content.get(pos.1)?.get(pos.0)
        // unsafe { self.content.get_unchecked(pos.1).get_unchecked(pos.0) }
    }

    pub fn get_unchecked(&mut self, pos: Pos) -> &Cell {
        self.content.get(pos.1).unwrap().get(pos.0).unwrap()
        // unsafe { self.content.get_unchecked(pos.1).get_unchecked(pos.0) }
    }

    // pub fn get_unchecked_mut(&mut self, pos: Pos) -> &mut Cell {
    //     unsafe {
    //         self.content
    //             .get_unchecked_mut(pos.1)
    //             .get_unchecked_mut(pos.0)
    //     }
    // }

    pub fn set_free_island(&mut self, pos: Pos, cost: u8) -> bool {
        if *self.get_unchecked(pos) == Cell::Path {
            self.set_unchecked(pos, Cell::Island(cost));

            true
        } else {
            false
        }
    }

    pub fn set_unchecked_bot(&mut self, pos: Pos, cost: u8) -> bool {
        if *self.get_unchecked(pos) == Cell::Path {
            self.set_unchecked(pos, Cell::Bot(cost));

            true
        } else {
            false
        }
    }

    pub fn set_best(&mut self, pos: Pos, flag: bool) -> bool {
        match *self.get_unchecked(pos) {
            Cell::Bot(cost) if flag => {
                self.set_unchecked(pos, Cell::Best(cost));

                true
            }
            Cell::Best(cost) if !flag => {
                self.set_unchecked(pos, Cell::Bot(cost));
                true
            }

            _ => false,
        }
    }

    // pub fn set_unchecked_block(&mut self, pos: Pos) -> bool {
    //     if *self.get_unchecked(pos) == Cell::Path {
    //         self.set_unchecked(pos, Cell::Wall);
    //
    //         true
    //     } else {
    //         false
    //     }
    // }
}
