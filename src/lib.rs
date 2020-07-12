mod utils;

use fixedbitset::FixedBitSet;
use js_sys::Math;
use wasm_bindgen::prelude::*;

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
        let width = 96;
        let height = 96;
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
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for delta_row in vec![self.height - 1, 0, 1] {
            for delta_col in vec![self.width - 1, 0, 1] {
                if !(delta_row == 0 && delta_col == 0) {
                    let neighbour_row = (row + delta_row) % self.height;
                    let neighbour_col = (column + delta_col) % self.width;
                    let idx = self.get_index(neighbour_row, neighbour_col);
                    count += self.cells[idx] as u8;
                }
            }
        }

        count
    }
}
