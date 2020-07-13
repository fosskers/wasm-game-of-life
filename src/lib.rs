mod utils;

use fixedbitset::FixedBitSet;
use js_sys::Math;
use wasm_bindgen::prelude::*;
// use web_sys;

// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        // To enable the output of Rust panics within a browser console.
        // utils::set_panic_hook();

        let width = 256;
        let height = 256;
        let mut cells = FixedBitSet::with_capacity(width * height);

        for i in 0..width * height {
            let rand = Math::random();
            cells.set(i, rand < 0.5);
        }

        Universe {
            width: width as u32,
            height: height as u32,
            cells,
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = width * self.height;
        let cells = FixedBitSet::with_capacity(size as usize);
        self.cells = cells;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = self.width * height;
        let cells = FixedBitSet::with_capacity(size as usize);
        self.cells = cells;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                next.set(
                    idx,
                    match (cell, live_neighbours) {
                        (true, x) if x < 2 => {
                            // log!("({}, {}) died of starvation.", row, col);
                            false
                        }
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => {
                            // log!("({}, {}) died of overpopulation.", row, col);
                            false
                        }
                        (false, 3) => {
                            // log!("({}, {}) has been born!", row, col);
                            true
                        }
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
        // (row * self.width * column) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if col == 0 { self.width - 1 } else { col - 1 };
        let east = if col == self.width - 1 { 0 } else { col + 1 };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, col);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, col);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    /// Flip the state of the cell at the given coordinates.
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.set(idx, !self.cells[idx]);
    }
}

impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells {
            let idx = self.get_index(*row, *col);
            self.cells.set(idx, true);
        }
    }
}
