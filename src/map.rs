use rand::Rng;

#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Path,
    Wall,
    Bot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaySide {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WayVariants {
    /// x | x
    /// x O x
    /// x O x
    Forward,

    /// x x x
    /// - O x
    /// x O x
    Left,

    /// x x x
    /// - O -
    /// x O x
    LeftAndRight,

    /// x x x
    /// x O -
    /// x O x
    Right,

    /// x | x
    /// - O -
    /// x O x
    Cross,
}

impl WayVariants {
    pub fn get_random(r: &mut rand::rngs::ThreadRng) -> Self {
        match r.gen_range::<u8, _>(0..5) {
            0 => Self::Forward,
            1 => Self::Left,
            2 => Self::LeftAndRight,
            3 => Self::Right,
            4 => Self::Cross,
            _ => unreachable!(),
        }
    }
}

pub type Pos = (usize, usize);

pub struct Map {
    size: usize,
    content: Vec<Vec<Cell>>,
    last_diff: Option<Vec<(Pos, Cell)>>
}

impl Map {
    pub fn new(size: usize) -> Self {
        let mut map = Vec::with_capacity(size);

        for _ in 0..size {
            let mut row = Vec::with_capacity(size);

            for _ in 0..size {
                row.push(Cell::Path)
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

    pub fn set_unchecked(&mut self, pos: Pos, cell: Cell) {
        unsafe {
            self.content.get_unchecked_mut(pos.1)[pos.0] = cell;
        }
    }

    pub fn get_unchecked_row_mut(&mut self, y: usize) -> &mut Vec<Cell> {
        unsafe { self.content.get_unchecked_mut(y) }
    }

    pub fn get_unchecked(&mut self, pos: Pos) -> &Cell {
        unsafe { self.content.get_unchecked(pos.1).get_unchecked(pos.0) }
    }

    pub fn get_unchecked_mut(&mut self, pos: Pos) -> &mut Cell {
        unsafe {
            self.content
                .get_unchecked_mut(pos.1)
                .get_unchecked_mut(pos.0)
        }
    }

    pub fn set_unchecked_bot(&mut self, pos: Pos) -> bool {
        if *self.get_unchecked(pos) == Cell::Path {
            self.set_unchecked(pos, Cell::Bot);

            true
        } else {
            false
        }
    }

    pub fn set_unchecked_block(&mut self, pos: Pos) -> bool {
        if *self.get_unchecked(pos) == Cell::Path {
            self.set_unchecked(pos, Cell::Wall);

            true
        } else {
            false
        }
    }

    pub fn set_unchecked_mask(&mut self, pos: Pos, mask: [[bool; 3]; 3]) -> Vec<(WaySide, Pos)> {
        let mut outputs = vec![];

        macro_rules! test_cell {
            ($mask_x:literal, $mask_y: literal) => {{
                let new_pos = (pos.0 + $mask_x - 1, pos.1 + $mask_y - 1);

                if mask[$mask_y][$mask_x] {
                    self.set_unchecked_block(new_pos);
                } else {
                    self.set_unchecked_bot(new_pos);
                }
            }};

            ($mask_x:literal, $mask_y: literal; $side: expr) => {{
                let new_pos = (pos.0 + $mask_x - 1, pos.1 + $mask_y - 1);

                if mask[$mask_y][$mask_x] {
                    self.set_unchecked_block(new_pos);
                } else {
                    if self.set_unchecked_bot(new_pos) {
                        outputs.push(($side, new_pos));
                    }
                }
            }};
        }

        test_cell!(0, 0);
        test_cell!(1, 0; WaySide::North);
        test_cell!(2, 0);

        test_cell!(0, 1; WaySide::West);
        test_cell!(2, 1; WaySide::East);

        test_cell!(0, 2);
        test_cell!(1, 2; WaySide::South);
        test_cell!(2, 2);

        outputs
    }

    pub fn set_way(
        &mut self,
        pos: Pos,
        side: WaySide,
        variant: WayVariants,
    ) -> Vec<(WaySide, Pos)> {
        match side {
            WaySide::North | WaySide::South => match variant {
                WayVariants::Forward => self.set_unchecked_mask(
                    pos,
                    [[true, false, true], [true, true, true], [true, false, true]],
                ),
                WayVariants::LeftAndRight => self.set_unchecked_mask(
                    pos,
                    [[true, true, true], [false, true, false], [true, true, true]],
                ),
                WayVariants::Cross => self.set_unchecked_mask(
                    pos,
                    [
                        [true, false, true],
                        [false, true, false],
                        [true, false, true],
                    ],
                ),
                WayVariants::Left if side == WaySide::North => self.set_unchecked_mask(
                    pos,
                    [[true, true, true], [false, true, true], [true, true, true]],
                ),

                WayVariants::Right if side == WaySide::North => self.set_unchecked_mask(
                    pos,
                    [[true, true, true], [true, true, false], [true, true, true]],
                ),

                WayVariants::Right if side == WaySide::South => self.set_unchecked_mask(
                    pos,
                    [[true, true, true], [false, true, true], [true, true, true]],
                ),

                WayVariants::Left if side == WaySide::South => self.set_unchecked_mask(
                    pos,
                    [[true, true, true], [true, true, false], [true, true, true]],
                ),

                _ => unreachable!(),
            },

            WaySide::East | WaySide::West => match variant {
                WayVariants::Forward => self.set_unchecked_mask(
                    pos,
                    [[true, true, true], [false, true, false], [true, true, true]],
                ),
                WayVariants::LeftAndRight => self.set_unchecked_mask(
                    pos,
                    [[true, false, true], [true, true, true], [true, false, true]],
                ),
                WayVariants::Cross => self.set_unchecked_mask(
                    pos,
                    [
                        [true, false, true],
                        [false, true, false],
                        [true, false, true],
                    ],
                ),
                WayVariants::Left if side == WaySide::East => self.set_unchecked_mask(
                    pos,
                    [[true, false, true], [true, true, true], [true, true, true]],
                ),

                WayVariants::Right if side == WaySide::East => self.set_unchecked_mask(
                    pos,
                    [[true, true, true], [true, true, true], [true, false, true]],
                ),

                WayVariants::Right if side == WaySide::West => self.set_unchecked_mask(
                    pos,
                    [[true, false, true], [true, true, true], [true, true, true]],
                ),

                WayVariants::Left if side == WaySide::West => self.set_unchecked_mask(
                    pos,
                    [[true, true, true], [true, true, true], [true, false, true]],
                ),

                _ => unreachable!(),
            },
        }
    }
}
