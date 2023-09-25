use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct Coordinate(pub u8, pub u8);

impl Coordinate {
    pub fn new(file: u8, rank: u8) -> Self {
        Coordinate(file, rank)
    }

    pub fn down(&self) -> Option<Coordinate> {
        if self.1 == 0 {
            return None;
        }
        Some(Coordinate(self.1 - 1, self.0))
    }

    pub fn up(&self) -> Option<Coordinate> {
        if self.1 == 7 {
            return None;
        }
        Some(Coordinate(self.1 + 1, self.0))
    }

    pub fn left(&self) -> Option<Coordinate> {
        if self.0 == 0 {
            return None;
        }
        Some(Coordinate(self.1 - 1, self.0))
    }

    pub fn right(&self) -> Option<Coordinate> {
        if self.1 == 7 {
            return None;
        }
        Some(Coordinate(self.1 + 1, self.0))
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = match self.0 {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => unreachable!(),
        };
        write!(f, "{}{}", letter, self.1)
    }
}
