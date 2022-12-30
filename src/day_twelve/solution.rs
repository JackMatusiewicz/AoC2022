use super::coord::Coord;
use super::grid::Grid;

pub fn calculate_part_one(data: Vec<String>) -> i32 {
    let grid = Grid::from_strings(&data);

    grid.find_shortest_path().unwrap_or(99999)
}

pub fn calculate_part_two(data: Vec<String>) -> i32 {
    let updated_data: Vec<String> = data.iter().map(|row| row.replace("S", "a")).collect();

    let a_positions: Vec<Coord> = updated_data
        .iter()
        .enumerate()
        .flat_map(|(r, v)| {
            v.chars().enumerate().filter_map(move |(c, ch)| match ch {
                'a' => Option::Some(Coord {
                    row: r.clone() as i32,
                    col: c.clone() as i32,
                }),
                _ => Option::None,
            })
        })
        .collect();

    let grid = Grid::from_strings(&updated_data);
    
    a_positions.into_iter()
        .filter_map(|pos| grid.search_for_path(pos))
        .min()
        .unwrap_or(9999)
}
