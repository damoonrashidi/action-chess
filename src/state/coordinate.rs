use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coord(pub i8, pub i8);

impl Coord {
    pub fn new(file: i8, rank: i8) -> Self {
        Coord(file, rank)
    }

    pub fn down(&self) -> Option<Coord> {
        if self.1 == 0 {
            return None;
        }
        Some(Coord(self.1 - 1, self.0))
    }

    pub fn up(&self) -> Option<Coord> {
        if self.1 == 7 {
            return None;
        }
        Some(Coord(self.1 + 1, self.0))
    }

    pub fn left(&self) -> Option<Coord> {
        if self.0 == 0 {
            return None;
        }
        Some(Coord(self.1 - 1, self.0))
    }

    pub fn right(&self) -> Option<Coord> {
        if self.1 == 7 {
            return None;
        }
        Some(Coord(self.1 + 1, self.0))
    }

    pub fn is_valid(&self) -> bool {
        (0..8).contains(&self.0) && (0..8).contains(&self.1)
    }
}

impl Display for Coord {
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
            _ => panic!("tried to get file for {}", self.0),
        };
        write!(f, "{}{}", letter, self.1 + 1)
    }
}
