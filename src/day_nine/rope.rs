use super::direction::Direction;

#[derive(Eq, PartialEq, Clone)]
pub struct Rope {
    pub head: (i32, i32),
    pub tail: (i32, i32),
}

impl Rope {
    fn y_delta(&self) -> i32 {
        (self.head.0 - self.tail.0).abs()
    }

    fn x_delta(&self) -> i32 {
        (self.head.1 - self.tail.1).abs()
    }

    fn apply_movement_to_head(&self, dir: &Direction) -> Rope {
        match dir {
            Direction::Up => Rope {
                head: (self.head.0 + 1, self.head.1),
                tail: self.tail,
            },
            Direction::Down => Rope {
                head: (self.head.0 - 1, self.head.1),
                tail: self.tail,
            },
            Direction::Left => Rope {
                head: (self.head.0, self.head.1 - 1),
                tail: self.tail,
            },
            Direction::Right => Rope {
                head: (self.head.0, self.head.1 + 1),
                tail: self.tail,
            },
        }
    }

    fn move_tail_for_constraints(&self) -> Option<Rope> {
        let x_delta = self.x_delta();
        let y_delta = self.y_delta();
        if x_delta <= 1 && y_delta <= 1 {
            return Option::None;
        }
        if x_delta == 0 {
            if self.head.0 > self.tail.0 {
                return Option::Some(Rope { head: self.head.clone(), tail: (self.head.0-1, self.tail.1)});
            } else {
                return Option::Some(Rope { head: self.head.clone(), tail: (self.head.0+1, self.tail.1)});
            }
        } else if y_delta == 0 {
            if self.head.1 > self.tail.1 {
                return Option::Some(Rope { head: self.head.clone(), tail: (self.tail.0, self.head.1 - 1)});
            } else {
                return Option::Some(Rope { head: self.head.clone(), tail: (self.tail.0, self.head.1 + 1)});
            }
        } else {
            let row = if self.head.0 > self.tail.0 { self.tail.0 + 1 } else {self.tail.0 - 1};
            let col = if self.head.1 > self.tail.1 { self.tail.1 + 1 } else {self.tail.1 - 1};

            return Option::Some(Rope { head: self.head.clone(), tail: (row, col)});
        }
    }

    pub fn apply_movements(&self, movements: &Vec<Direction>) -> Vec<Rope> {
        let mut rope_positions = vec![self.clone()];

        let mut last_added_index: usize = 0;
        for movement in movements.iter() {
            let last_pos = rope_positions.get(last_added_index).unwrap();
            let next_pos = last_pos.apply_movement_to_head(movement);
            rope_positions.push(next_pos.clone());
            last_added_index += 1;

            let updated_next = next_pos.move_tail_for_constraints();
            updated_next.map(|v| {
                last_added_index += 1;
                rope_positions.push(v);
            });
        }
        rope_positions
    }
}
