extern crate wasm_specs;

fn main() {
    wasm_specs::run();
}

#[test]
fn test_run() {
    // Just run the full application and make sure it doesn't panic
    wasm_specs::run();
}
