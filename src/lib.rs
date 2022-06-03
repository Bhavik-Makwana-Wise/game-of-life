mod utils;

extern crate js_sys;
extern crate fixedbitset;
extern crate web_sys;

use web_sys::console;
use fixedbitset::FixedBitSet;
use std::fmt;
use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt)* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}


impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '⬜' } else {'⬛'};
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for cell in  0..size {
            cells.set(cell, js_sys::Math::random() < 0.5);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn reset(&mut self) {
        log!("reset");
        let size = (self.width * self.height) as usize;

        for cell in 0..size {
            self.cells.set(cell, js_sys::Math::random() < 0.5);
        }
    }

    pub fn clear(&mut self) {
        log!("clear");
        let size = (self.width * self.height) as usize;

        for cell in 0..size {
            self.cells.set(cell, false);
        }
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.set(idx, !self.cells[idx]);
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity((width*self.height) as usize);
        for cell in 0..width*self.height {
            self.cells.set(cell as usize, false);
        }
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity((self.width*height) as usize);
        for cell in 0..self.width*height {
            self.cells.set(cell as usize, false);
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Univers::tick");
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);
                // log!(
                //    "cell[{}, {}] is initially {:?} and has {} live neighbours",
                //    row,
                //    col,
                //    cell,
                //    live_neighbours
                //);

                next.set(idx, match (cell, live_neighbours) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                });
                // log!("     it becomes {:?}", next[idx]);
            }
        }
        self.cells = next;
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

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

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

        //let nw = self.get_index(north, west);
        //count += self.cells[nw] as u8;

        //let n = self.get_index(north, column);
        //count += self.cells[n] as u8;
        for d_row in [north, south, row].iter() {
            for d_col in [east, west, column].iter() {
                if *d_row == row && *d_col == column {
                    continue;
                }
                let idx = self.get_index(*d_row, *d_col);
                count += self.cells[idx] as u8;
            }
        }

        count
    }
}

impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}
