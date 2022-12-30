use super::coord::Coord;
use super::grid_cell::GridCell;
use std::collections::{HashSet, VecDeque};
use std::vec::Vec;
use super::linked_list::LinkedListNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Grid {
    cells: Vec<Vec<GridCell>>,
}

impl Grid {
    pub fn print(&self) -> () {
        for row in self.cells.iter() {
            for v in row.iter() {
                match v {
                    GridCell::Start => print!("S"),
                    GridCell::End => print!("E"),
                    GridCell::Cell(v) => {
                        print!("{}", char::from_u32((*v as u32) + ('a' as u32)).unwrap())
                    }
                }
            }
            println!("");
        }
    }

    pub fn from_strings(data: &Vec<String>) -> Grid {
        let grid_cells: Vec<Vec<GridCell>> = data
            .iter()
            .map(|row| row.chars().map(GridCell::from_char).collect())
            .collect();
        Grid { cells: grid_cells }
    }

    fn find_start(&self) -> Option<Coord> {
        for r in 0..self.cells.len() {
            let row = &self.cells[r];
            for c in 0..row.len() {
                match &row[c] {
                    GridCell::Start => {
                        return Option::Some(Coord::make(r as i32, c as i32));
                    }
                    _ => (),
                }
            }
        }
        Option::None
    }

    fn get_value(&self, pos: &Coord) -> Option<&GridCell> {
        if pos.row < 0 || pos.col < 0 {
            return Option::None;
        }
        let row = self.cells.get(pos.row as usize)?;
        let cell = row.get(pos.col as usize)?;
        Option::Some(&cell)
    }

    fn get_movement_deltas() -> Vec<(i32, i32)> {
        vec![(-1, 0), (0, 1), (1, 0), (0, -1) ]
    }



    fn search_for_path(
        &self,
        start_pos: Coord,
    ) -> Option<i32> {
        let mut visited_cells = HashSet::<Coord>::new();
        let mut to_visit = VecDeque::<(Coord, i32)>::new();
        to_visit.push_back((start_pos.clone(), 0));

        while !to_visit.is_empty() {
            let (pos, l) = to_visit.pop_front().unwrap();
            let cell = self.get_value(&pos).unwrap();
            if cell.eq(&GridCell::End) {
                return Option::Some(l);
            }

            Grid::get_movement_deltas()
                .iter()
                .map(|(dr, dc)| Coord { row: pos.row + dr, col: pos.col + dc })
                .filter_map(|p| {
                    self.get_value(&p).map(|_| p)
                })
                .filter(|p| {
                    if let Some(p_cell) = self.get_value(p) {
                        return cell.can_move_to(p_cell); 
                    }
                    return false;
                })
                .for_each(|p| {
                    if visited_cells.insert(p.clone()) {
                        to_visit.push_back((p.clone(), l + 1))
                    }
                });
        }

        Option::None
    }

    pub fn find_shortest_path(&self) -> Option<i32> {
        let start_position = self.find_start()?;
        println!(
            "Start position is row={}, col={}",
            start_position.row, start_position.col
        );

        let length = self.search_for_path( start_position)?;
        
        Option::Some(length)
    }
}
