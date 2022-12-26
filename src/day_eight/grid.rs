pub struct Grid {
    grid: Vec<Vec<i32>>,
    is_visible: Vec<Vec<bool>>,
}

impl Grid {
    fn update_from_left(&mut self) {
        for row in 0..self.grid.len() {
            let mut highest_seen = -1;
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] > highest_seen {
                    self.is_visible[row][col] = true;
                    highest_seen = self.grid[row][col];
                }
            }
        }
    }

    fn update_from_right(&mut self) {
        for row in 0..self.grid.len() {
            let mut highest_seen = -1;
            for c in 0..self.grid[row].len() {
                let col = self.grid[row].len() - 1 - c;
                if self.grid[row][col] > highest_seen {
                    self.is_visible[row][col] = true;
                    highest_seen = self.grid[row][col];
                }
            }
        }
    }

    fn update_from_top(&mut self) {
        for col in 0..self.grid[0].len() {
            let mut highest_seen = -1;
            for row in 0..self.grid.len() {
                if self.grid[row][col] > highest_seen {
                    self.is_visible[row][col] = true;
                    highest_seen = self.grid[row][col];
                }
            }
        }
    }

    fn update_from_bottom(&mut self) {
        for col in 0..self.grid[0].len() {
            let mut highest_seen = -1;
            for r in 0..self.grid.len() {
                let row = self.grid.len() - 1 - r;
                if self.grid[row][col] > highest_seen {
                    self.is_visible[row][col] = true;
                    highest_seen = self.grid[row][col];
                }
            }
        }
    }

    fn update_visible_grid(mut self) -> Self {
        self.update_from_left();
        self.update_from_right();
        self.update_from_top();
        self.update_from_bottom();

        self
    }

    pub fn from_grid(grid: Vec<Vec<i32>>) -> Grid {
        let mut visible_grid = vec![];
        for r in grid.iter() {
            let mut row = vec![];
            for _ in 0..r.len() {
                row.push(false);
            }
            visible_grid.push(row);
        }

        let grid = Grid {
            grid,
            is_visible: visible_grid,
        };

        grid.update_visible_grid()
    }

    fn in_range(&self, row: usize, col: usize) -> bool {
        row < self.grid.len() && col < self.grid[row].len()
    }

    fn viewing_distance_to_edge(&self, row: i32, col: i32, row_delta: i32, col_delta: i32) -> i32 {
        let mut visible_trees = 0;
        let mut next_row = row + row_delta;
        let mut next_col = col + col_delta;

        while self.in_range(next_row as usize, next_col as usize) {
            if self.grid[row as usize][col as usize]
                > self.grid[next_row as usize][next_col as usize]
            {
                visible_trees += 1;
                next_col += col_delta;
                next_row += row_delta;
            } else {
                return visible_trees + 1;
            }
        }
        return visible_trees;
    }

    fn get_scene_score(&self, row: i32, col: i32) -> i32 {
        self.viewing_distance_to_edge(row, col, -1, 0)
            * self.viewing_distance_to_edge(row, col, 1, 0)
            * self.viewing_distance_to_edge(row, col, 0, 1)
            * self.viewing_distance_to_edge(row, col, 0, -1)
    }

    pub fn get_best_viewing_score(&self) -> i32 {
        let mut highest_score = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                let score = self.get_scene_score(i as i32, j as i32);
                if score > highest_score {
                    highest_score = score;
                }
            }
        }
        highest_score
    }

    pub fn get_visible_count(&self) -> i32 {
        self.is_visible.iter().fold(0, |acc, x| {
            x.iter()
                .fold(acc, |acc2, v| if *v { acc2 + 1 } else { acc2 })
        })
    }
}
