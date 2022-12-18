use regex::Regex;

fn get_regex() -> Regex {
    Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap()
}

pub fn calculate_part_one(data: Vec<String>) -> i32 {
    let r = get_regex();

    let mut enclosing = 0;
    for l in data.into_iter() {
        for cap in r.captures_iter(&l[..]) {
            let l1: i32 = cap[1].parse().unwrap();
            let r1: i32 = cap[2].parse().unwrap();
            let l2: i32 = cap[3].parse().unwrap();
            let r2: i32 = cap[4].parse().unwrap();

            if (l1 >= l2 && r1 <= r2) || (l2 >= l1 && r2 <= r1) {
                enclosing += 1;
            }
        }
    }
    enclosing
}

pub fn calculate_part_two(data: Vec<String>) -> i32 {
    let r = get_regex();

    let mut enclosing = 0;
    for l in data.into_iter() {
        for cap in r.captures_iter(&l[..]) {
            let l1: i32 = cap[1].parse().unwrap();
            let r1: i32 = cap[2].parse().unwrap();
            let l2: i32 = cap[3].parse().unwrap();
            let r2: i32 = cap[4].parse().unwrap();

            let separate_sums = (r1 - l1 + 1) + (r2 - l2 + 1);

            let min_left = std::cmp::min(l1, l2);
            let max_right = std::cmp::max(r1, r2);
            let sum = max_right - min_left + 1;

            if sum < separate_sums {
                enclosing += 1;
            }
        }
    }
    enclosing
}
