pub enum ListElement {
    IntValue(i32),
    ListValue(Vec<ListElement>)
}

impl ListElement {
    fn int_from_string(v: &str) -> ListElement {
        let v = v.parse::<i32>().unwrap();
        ListElement::IntValue(v)
    }

    pub fn from_string(v: &str) -> ListElement {
        let chars: Vec<char> = v.chars().collect();
        let mut current_list: Vec<ListElement> = vec![];
        let mut parent_lists: Vec<Vec<ListElement>> = vec![];

        let mut start_number_index = -1;
        for i in 1 .. (v.len() - 1) {
            match chars[i] {
                '[' => {
                    // We are starting a new list
                    parent_lists.push(current_list);
                    current_list = vec![];
                },
                ']' => {
                    if start_number_index != -1 {
                        // We are making a number.
                        let number_string = &v[start_number_index as usize .. i as usize];
                        println!("Making int from: {number_string}");
                        let number_element = ListElement::int_from_string(number_string);
                        current_list.push(number_element);
                        start_number_index = -1;
                    }
                    let child = ListElement::ListValue(current_list);
                    if let Some(mut parent) = parent_lists.pop() {
                        parent.push(child);
                        current_list = parent;
                    } else {
                        return child;
                    }
                },
                ',' => {
                    // We have ended an element in this current list.
                    if start_number_index != -1 {
                        // We are making a number.
                        let number_string = &v[start_number_index as usize .. i as usize];
                        println!("Making int from: {number_string}");
                        let number_element = ListElement::int_from_string(number_string);
                        current_list.push(number_element);
                        start_number_index = -1;
                    } else {
                        // We are finishing a list, we have already added it so do nothing here.
                    }
                },
                _ => {
                    // This is a digit, so change the starting index of the number if we have one.
                    if start_number_index == -1 {
                        start_number_index = i as i32;
                    }
                }
            }
        }
        if start_number_index != -1 {
            // We are making a number.
            let number_string = &v[start_number_index as usize .. (v.len()-1) as usize];
            println!("Making int from: {number_string}");
            let number_element = ListElement::int_from_string(number_string);
            current_list.push(number_element);
            start_number_index = -1;
        }
        ListElement::ListValue(current_list)
    }

    pub fn print(&self) -> () {
        match self {
            ListElement::IntValue(v) => print!("{v}"),
            ListElement::ListValue(ls) => {
                print!("[");
                for v in ls.iter() {
                    v.print();
                    print!(",");
                }
                print!("]");
            }
        }
    }
}