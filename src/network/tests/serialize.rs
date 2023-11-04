#[cfg(test)]
mod test {
    use crate::state::{coordinate::Coord, square::*};

    #[test]
    fn coord_into_u8() {
        for (coord, expected) in [(A1, 0), (A2, 8), (H8, 63)] {
            let actual: u8 = coord.into();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn u8_into_coord() {
        for (expected, index) in [(A1, 0), (A2, 8), (H8, 63)] {
            let actual: Coord = index.into();
            assert_eq!(actual, expected);
        }
    }

    // #[test]
    // fn serialize_move() {
    //     let mv = Move::Piece(A5, C5);
    //     let command: Command = mv.into();
    //     println!("{:?}", command);
    //     assert_eq!(command, Command([0, 0b0000_0100, 0b0000_0000, 0]));
    // }
}
