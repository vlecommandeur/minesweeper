mod minesweeper;
mod random;

use minesweeper::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
    let ms = Minesweeper::new(10, 10, 5);

    ms.to_string()
}
