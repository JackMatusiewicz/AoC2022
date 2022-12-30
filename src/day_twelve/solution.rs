use super::grid::Grid;

pub fn calculate_part_one(data: Vec<String>) -> i32 {
    let grid = Grid::from_strings(&data);

    grid.find_shortest_path().unwrap_or(99)
}
