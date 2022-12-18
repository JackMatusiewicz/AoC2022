use std::{
    collections::{HashMap, HashSet},
    mem::take,
};

fn get_mapping() -> HashMap<char, i32> {
    HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ])
}

fn get_duplicate(v: &String) -> i32 {
    let mapping = get_mapping();
    let half_len = v.len() / 2;
    let mut ctr = 0;
    let mut first_half = HashSet::new();
    let mut second_half = HashSet::new();
    for c in v.chars() {
        if ctr < half_len {
            first_half.insert(c);
        } else {
            second_half.insert(c);
        }
        ctr = ctr + 1;
    }

    let mut sum = 0;

    for c in first_half.intersection(&second_half) {
        sum += mapping[c];
    }

    return sum;
}

pub fn calculate_part_one(data: Vec<String>) -> i32 {
    data.iter().map(get_duplicate).sum()
}

fn add_to_dict(d: &mut HashMap<char, i32>, c: &HashSet<char>) -> () {
    for i in c {
        let ent = d.entry(*i);
        let mut v = ent.or_insert(0);
        *v = *v + 1;
    }
}

pub fn find_shared_item(data: &[String]) -> i32 {
    let m = get_mapping();
    let mut d = HashMap::<char, i32>::new();
    let v: HashMap<char, i32> = data
        .iter()
        .map(|c| {
            let v: HashSet<char> = c.chars().collect();
            return v;
        })
        .fold(HashMap::<char, i32>::new(), |mut c, s| {
            add_to_dict(&mut c, &s);
            return c;
        });

    v.into_iter()
        .filter(|&(_, count)| count == 3)
        .map(|(c, _)| m[&c])
        .sum()
}

pub fn calculate_part_two(data: Vec<String>) -> i32 {
    data.chunks(3).map(find_shared_item).sum()
}
