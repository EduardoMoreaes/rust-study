//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

extern crate webasm_rust;
use webasm_rust::Universe;

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

