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

fn main() -> Result<(), Box<dyn Error>> {
    let routes = alc::routes();
    println!("Routes: {:#?}", routes);

    alc::engage(([0, 0, 0, 0], 2137));

    Ok(())
}