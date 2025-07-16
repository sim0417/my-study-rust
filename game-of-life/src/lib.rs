mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    /// Create a new universe with specified dimensions
    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let size = (width * height) as usize;
        let mut cells = vec![Cell::Dead; size];

        // Initialize with some interesting patterns
        for i in 0..size {
            cells[i] = if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            };
        }

        log!("Universe created with dimensions: {}x{}", width, height);

        Universe {
            width,
            height,
            cells,
        }
    }

    /// Get the width of the universe
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the universe
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get pointer to the cells
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Get cells as string for display
    pub fn render(&self) -> String {
        self.to_string()
    }

    /// Get a direct pointer to the cells array for JavaScript access
    pub fn cells_ptr(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Get total number of cells
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Resize the universe
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;

        let size = (width * height) as usize;
        self.cells = vec![Cell::Dead; size];

        log!("Universe resized to: {}x{}", width, height);
    }

    /// Advance the universe by one tick
    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");

        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    /// Clear all cells (kill all)
    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = Cell::Dead;
        }
        log!("Universe cleared");
    }

    /// Toggle the state of a cell
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx] = match self.cells[idx] {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }

    /// Load a predefined pattern
    pub fn load_pattern(&mut self, pattern: &str) {
        self.clear();

        match pattern {
            "glider" => self.load_glider(),
            "pulsar" => self.load_pulsar(),
            "beacon" => self.load_beacon(),
            "toad" => self.load_toad(),
            "spaceship" => self.load_spaceship(),
            _ => {
                log!("Unknown pattern: {}", pattern);
                return;
            }
        }

        log!("Loaded pattern: {}", pattern);
    }

    /// Get current generation performance stats
    pub fn get_stats(&self) -> String {
        let live_cells = self.cells.iter().filter(|&c| *c == Cell::Alive).count();
        format!(
            "Live cells: {}, Total cells: {}",
            live_cells,
            self.cells.len()
        )
    }

    /// Create a random universe
    pub fn random(width: u32, height: u32) -> Universe {
        let size = (width * height) as usize;
        let mut cells = vec![Cell::Dead; size];

        for i in 0..size {
            cells[i] = if js_sys::Math::random() < 0.5 {
                Cell::Alive
            } else {
                Cell::Dead
            };
        }

        log!(
            "Random universe created with dimensions: {}x{}",
            width,
            height
        );

        Universe {
            width,
            height,
            cells,
        }
    }
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let neighbors = [
            (north, west),
            (north, column),
            (north, east),
            (row, west),
            (row, east),
            (south, west),
            (south, column),
            (south, east),
        ];

        for (r, c) in neighbors {
            let idx = self.get_index(r, c);
            if self.cells[idx] == Cell::Alive {
                count += 1;
            }
        }

        count
    }

    // Predefined patterns
    fn load_glider(&mut self) {
        let cells = vec![(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)];
        self.set_cells(&cells);
    }

    fn load_pulsar(&mut self) {
        let center_row = self.height / 2;
        let center_col = self.width / 2;

        let pulsar_pattern = vec![
            (-6, -4),
            (-6, -3),
            (-6, -2),
            (-6, 2),
            (-6, 3),
            (-6, 4),
            (-4, -6),
            (-4, -1),
            (-4, 1),
            (-4, 6),
            (-3, -6),
            (-3, -1),
            (-3, 1),
            (-3, 6),
            (-2, -6),
            (-2, -1),
            (-2, 1),
            (-2, 6),
            (-1, -4),
            (-1, -3),
            (-1, -2),
            (-1, 2),
            (-1, 3),
            (-1, 4),
            (1, -4),
            (1, -3),
            (1, -2),
            (1, 2),
            (1, 3),
            (1, 4),
            (2, -6),
            (2, -1),
            (2, 1),
            (2, 6),
            (3, -6),
            (3, -1),
            (3, 1),
            (3, 6),
            (4, -6),
            (4, -1),
            (4, 1),
            (4, 6),
            (6, -4),
            (6, -3),
            (6, -2),
            (6, 2),
            (6, 3),
            (6, 4),
        ];

        let cells: Vec<(u32, u32)> = pulsar_pattern
            .iter()
            .map(|(dr, dc)| {
                let row = ((center_row as i32 + dr) + self.height as i32) as u32 % self.height;
                let col = ((center_col as i32 + dc) + self.width as i32) as u32 % self.width;
                (row, col)
            })
            .collect();

        self.set_cells(&cells);
    }

    fn load_beacon(&mut self) {
        let cells = vec![
            (1, 1),
            (1, 2),
            (2, 1),
            (2, 2),
            (3, 3),
            (3, 4),
            (4, 3),
            (4, 4),
        ];
        self.set_cells(&cells);
    }

    fn load_toad(&mut self) {
        let cells = vec![(2, 2), (2, 3), (2, 4), (3, 1), (3, 2), (3, 3)];
        self.set_cells(&cells);
    }

    fn load_spaceship(&mut self) {
        let cells = vec![
            (1, 1),
            (1, 4),
            (2, 5),
            (3, 1),
            (3, 5),
            (4, 2),
            (4, 3),
            (4, 4),
            (4, 5),
        ];
        self.set_cells(&cells);
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}
