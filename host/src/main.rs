mod bindings;
mod host;
mod prettify;

use prettify::*;

pub fn main() {
    let mut prettify = Prettify::new("target/wasm32-wasip2/release/plugin.wasm").expect("initialize wasmtime");

    let pretty = prettify
        .prettify(&["We", "will", "prettify", "this", "with", "a", "plugin"])
        .expect("call Wasm")
        .expect("prettify");

    println!("{}", pretty);

    let greeting = prettify.greet("Linus", "Torvalds").expect("call Wasm").expect("greet");

    println!("{}", greeting);
}
