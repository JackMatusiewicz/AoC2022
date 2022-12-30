#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Clone)]
pub enum GridCell {
    Start,
    End,
    Cell(i32),
}

impl GridCell {
    pub fn from_char(c: char) -> GridCell {
        match c {
            'S' => GridCell::Start,
            'E' => GridCell::End,
            _ => GridCell::Cell((c as i32) - ('a' as i32)),
        }
    }

    pub fn replace(&self) -> GridCell {
        match self {
            GridCell::Start => GridCell::Cell(0),
            GridCell::End => GridCell::Cell(26),
            _ => self.clone()
        }
    }

    pub fn can_move_to(&self, other: &GridCell) -> bool {
        let a = self.replace();
        let b = other.replace();
        match (a, b) {
            (GridCell::Cell(from), GridCell::Cell(to)) => to - from <= 1,
            _ => false
        }
    }
}
