use super::direction::Direction;

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Pos {
    row: i32,
    col: i32,
}

#[derive(Eq, PartialEq, Clone)]
pub struct Rope {
    pub head: Pos,
    pub tail: Vec<Pos>,
}

impl Rope {
    pub fn make_part_one_rope() -> Rope {
        Rope {
            head: Pos { row: 0, col: 0 },
            tail: vec![Pos { row: 0, col: 0 }],
        }
    }

    pub fn make_part_two_rope() -> Rope {
        Rope {
            head: Pos { row: 0, col: 0 },
            tail: vec![
                Pos { row: 0, col: 0 },
                Pos { row: 0, col: 0 },
                Pos { row: 0, col: 0 },
                Pos { row: 0, col: 0 },
                Pos { row: 0, col: 0 },
                Pos { row: 0, col: 0 },
                Pos { row: 0, col: 0 },
                Pos { row: 0, col: 0 },
                Pos { row: 0, col: 0 },
            ],
        }
    }

    fn y_delta(head: &Pos, tail: &Pos) -> i32 {
        (head.row - tail.row).abs()
    }

    fn x_delta(head: &Pos, tail: &Pos) -> i32 {
        (head.col - tail.col).abs()
    }

    fn apply_movement_to_head(head: &Pos, dir: &Direction) -> Pos {
        match dir {
            Direction::Up => Pos {
                row: head.row + 1,
                col: head.col,
            },
            Direction::Down => Pos {
                row: head.row - 1,
                col: head.col,
            },
            Direction::Left => Pos {
                row: head.row,
                col: head.col - 1,
            },
            Direction::Right => Pos {
                row: head.row,
                col: head.col + 1,
            },
        }
    }

    // Update the tail position based on the head.
    fn move_tail_for_constraints(head: &Pos, tail: &Pos) -> Pos {
        let x_delta = Rope::x_delta(head, tail);
        let y_delta = Rope::y_delta(head, tail);
        if x_delta <= 1 && y_delta <= 1 {
            return tail.clone();
        }
        if x_delta == 0 {
            if head.row > tail.row {
                return Pos {
                    row: head.row - 1,
                    col: tail.col,
                };
            } else {
                return Pos {
                    row: head.row + 1,
                    col: tail.col,
                };
            }
        } else if y_delta == 0 {
            if head.col > tail.col {
                return Pos {
                    row: tail.row,
                    col: head.col - 1,
                };
            } else {
                return Pos {
                    row: tail.row,
                    col: head.col + 1,
                };
            }
        } else {
            let row = if head.row > tail.row {
                tail.row + 1
            } else {
                tail.row - 1
            };
            let col = if head.col > tail.col {
                tail.col + 1
            } else {
                tail.col - 1
            };

            return Pos { row, col };
        }
    }

    pub fn apply_movements(&self, movements: &Vec<Direction>) -> Vec<Rope> {
        let mut rope_positions = vec![self.clone()];

        let mut last_added_index: usize = 0;
        for movement in movements.iter() {
            let last_pos = rope_positions.get(last_added_index).unwrap();
            let next_pos_head = Rope::apply_movement_to_head(&last_pos.head, movement);
            let mut next_in_chain = next_pos_head.clone();
            let mut tail = vec![];
            for next_tail in last_pos.tail.iter() {
                let updated_tail = Rope::move_tail_for_constraints(&next_in_chain, &next_tail);
                tail.push(updated_tail.clone());
                next_in_chain = updated_tail;
            }
            rope_positions.push(Rope {
                head: next_pos_head,
                tail,
            });
            last_added_index += 1;
        }
        rope_positions
    }
}
