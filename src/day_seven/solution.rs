use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;

use crate::day_seven::folder::Folder;

fn make_file_system(data: Vec<String>) -> Rc<RefCell<Folder>> {
    let root_name = Rc::new("/".to_string());
    let root_folder = Rc::new(RefCell::new(Folder::from_dir(&root_name)));

    let mut path_to_root = Vec::<Rc<RefCell<Folder>>>::new();
    let mut current = root_folder.clone();

    for line in data.into_iter() {
        if !line.starts_with("$ ls") {
            if line.starts_with("dir") {
                let mut segments = line.split(' ');
                segments.next();
                match segments.next() {
                    Option::Some(n) => {
                        let dname = Rc::new(n.to_string());
                        let new_folder = Rc::new(RefCell::new(Folder::from_dir(&dname)));
                        current.borrow_mut().insert(new_folder);
                    }
                    Option::None => panic!("Invalid input)"),
                }
            } else if line.starts_with("$ cd") {
                let mut segments = line.split(' ');
                segments.next();
                segments.next();

                let dir_name = segments.next().unwrap();
                match dir_name {
                    ".." => {
                        current = path_to_root.pop().unwrap();
                    }
                    "/" => {
                        path_to_root = vec![];
                        current = root_folder.clone();
                    }
                    _ => {
                        path_to_root.push(current.clone());
                        let dname = Rc::new(dir_name.to_string());
                        let next_current = current.borrow().get_dir(dname);
                        current = next_current;
                    }
                }
            } else {
                let mut segments = line.split(' ');
                let size = segments.next().unwrap().parse::<i32>().unwrap();
                let name = Rc::new(segments.next().unwrap().to_string());
                let new_file = Rc::new(RefCell::new(Folder::from_file(&name, size)));
                current.borrow_mut().insert(new_file);
            }
        }
    }

    root_folder
}

fn calculate_sizes(v: &Folder) -> i32 {
    match &v {
        Folder::Dir(_, contents) => {
            let total = if v.calculate_size() <= 100000 {
                v.calculate_size()
            } else {
                0
            };

            let children: i32 = contents
                .iter()
                .map(|(_, v)| calculate_sizes(&v.borrow()))
                .sum();

            total + children
        }
        _ => 0,
    }
}

pub fn calculate_part_one(data: Vec<String>) -> i32 {
    let fs = make_file_system(data);

    let v = fs.borrow();
    calculate_sizes(&v)
}

pub fn calculate_part_two(data: Vec<String>) -> i32 {
    let fs = make_file_system(data);

    let root_size = fs.borrow().calculate_size();

    let mut dir_sizes = vec![];
    fs.borrow().get_all_dir_sizes(&mut dir_sizes);

    let remaining_free = 70000000 - root_size;
    let minimum_required_space = 30000000;

    let mut smallest_improvement = 70000000;
    let mut best_dir_size = 0;

    for i in dir_sizes {
        let new_free = remaining_free + i;
        if new_free > minimum_required_space {
            if new_free - minimum_required_space < smallest_improvement {
                best_dir_size = i;
                smallest_improvement = new_free - minimum_required_space;
            }
        }
    }

    best_dir_size
}
