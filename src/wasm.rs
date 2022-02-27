
use wasm_bindgen::prelude::*;

use crate::interpreter::{Builder, Handle};
use crate::reader::Reader;
use crate::list::List;

#[wasm_bindgen]
pub fn run_some_code(code: &str) {
    let mut interp = Builder::default().eval(List::from(Vec::from_iter(Reader::from(code))));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_test() {
        run_some_code("testo");
    }
}

