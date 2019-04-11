extern crate alcubierre as alc;

use std::error::Error;

#[alc::get]
fn greet(name: String) -> String {
    format!("Hello, {}", name)
}

fn main() -> Result<(), Box<dyn Error>> {
    let routes = alc::routes();
    println!("Routes: {:#?}", routes);

    Ok(())
}