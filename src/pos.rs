use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(pub usize, pub usize);

impl Pos {
    pub fn checked_add(self, rhs: (isize, isize)) -> Option<Self> {
        Some(Pos(
            self.0.checked_add_signed(rhs.0)?,
            self.1.checked_add_signed(rhs.1)?,
        ))
    }
}

impl ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos(self.0.saturating_sub(rhs.0), self.1.saturating_sub(rhs.1))
    }
}
