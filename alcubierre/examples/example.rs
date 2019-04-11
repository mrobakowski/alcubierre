extern crate alcubierre as alc;

use std::error::Error;

#[alc::get]
fn greet(name: String) -> String {
    format!("Hello, {}", name)
}

#[alc::get]
fn name() -> &'static str {
    "Mikołaj Robakowski"
}

fn main() {
    println!("Server starting at http://localhost:2137/");
    println!("Press Ctrl+C to exit...");
    alc::engage(([0, 0, 0, 0], 2137));
}