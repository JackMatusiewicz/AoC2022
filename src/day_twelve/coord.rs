#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Coord {
    pub row: i32,
    pub col: i32,
}

impl Coord {
    pub fn make(row: i32, col: i32) -> Coord {
        Coord { row, col }
    }
}
