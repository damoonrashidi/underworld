#[derive(Debug, PartialEq, Clone)]
pub struct Coord(pub usize, pub usize);

impl Coord {
    #[must_use]
    pub fn up(&self) -> Coord {
        Coord(self.0, self.1.saturating_sub(1))
    }

    #[must_use]
    pub fn down(&self) -> Coord {
        Coord(self.0, self.1.saturating_add(1))
    }

    #[must_use]
    pub fn left(&self) -> Coord {
        Coord(self.0.saturating_sub(1), self.1)
    }

    #[must_use]
    pub fn right(&self) -> Coord {
        Coord(self.0.saturating_add(1), self.1)
    }
}
