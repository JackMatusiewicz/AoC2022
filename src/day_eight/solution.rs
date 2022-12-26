use crate::day_eight::grid::Grid;

fn convert_to_grid(data: Vec<String>) -> Vec<Vec<i32>> {
    let mut rows = Vec::<Vec<i32>>::new();

    for line in data.iter() {
        let mut r = vec![];
        for c in line.chars() {
            r.push(c.to_digit(10).unwrap() as i32);
        }
        rows.push(r);
    }

    return rows;
}

pub fn calcuate_part_one(data: Vec<String>) -> i32 {
    let grid = convert_to_grid(data);
    let g = Grid::from_grid(grid);
    g.get_visible_count()
}

pub fn calcuate_part_two(data: Vec<String>) -> i32 {
    let grid = convert_to_grid(data);
    let g = Grid::from_grid(grid);
    g.get_best_viewing_score()
}
