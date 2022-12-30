use super::coord::Coord;
use std::rc::Rc;
use std::cell::RefCell;

pub struct LinkedListNode {
    pub coord: Coord,
    pub next: Option<Rc<RefCell<LinkedListNode>>>
}

impl LinkedListNode {
    pub fn base(c: Coord) -> LinkedListNode {
        LinkedListNode {
            coord: c,
            next: Option::None
        }
    }

    pub fn cons(h: Coord, next: Rc<RefCell<LinkedListNode>>) -> LinkedListNode {
        LinkedListNode {
            coord: h,
            next: Option::Some(next.clone())
        }
    }
}