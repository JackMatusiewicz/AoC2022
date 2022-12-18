use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub enum Folder {
    Dir(Rc<String>, HashMap<Rc<String>, Rc<RefCell<Folder>>>),
    File(Rc<String>, i32),
}

impl Folder {
    pub fn from_file(name: &Rc<String>, size: i32) -> Folder {
        Folder::File(Rc::clone(name), size)
    }

    pub fn from_dir(name: &Rc<String>) -> Folder {
        Folder::Dir(Rc::clone(name), HashMap::new())
    }

    pub fn calculate_size(&self) -> i32 {
        match self {
            &Folder::File(_, i) => i,
            Folder::Dir(_, contents) => contents
                .iter()
                .map(|(_, v)| v.borrow().calculate_size())
                .sum(),
        }
    }

    pub fn get_all_dir_sizes(&self, sizes: &mut Vec<i32>) -> () {
        match self {
            Folder::File(_, _) => {}
            Folder::Dir(_, contents) => {
                sizes.push(self.calculate_size());
                contents
                    .iter()
                    .for_each(|(_, v)| v.borrow().get_all_dir_sizes(sizes));
            }
        }
    }

    pub fn get_name(&self) -> Rc<String> {
        match self {
            Folder::File(name, _) => name.clone(),
            Folder::Dir(name, _) => name.clone(),
        }
    }

    pub fn insert(&mut self, v: Rc<RefCell<Folder>>) -> () {
        match self {
            Folder::Dir(_, contents) => {
                let v_name = v.borrow().get_name();
                contents.insert(v_name, v.clone());
            }
            _ => panic!("We should never be calling insert on a file, this makes no sense"),
        }
    }

    pub fn get_dir(&self, dir_name: Rc<String>) -> Rc<RefCell<Folder>> {
        match self {
            Folder::Dir(_, contents) => contents.get(&dir_name).unwrap().clone(),
            _ => panic!(""),
        }
    }
}
