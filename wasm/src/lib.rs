extern crate pure_evm;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn exec(code: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    pure_evm::exec(code, data).unwrap()
}
