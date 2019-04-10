extern crate alcubierre as alc;

use std::error::Error;

#[alc::get("/name")]
fn name() -> &'static str {
    "Foo Bar"
}

fn main() -> Result<(), Box<dyn Error>> {
    let routes = alc::routes();
    println!("Routes: {:#?}", routes);

    let fun: &(fn() -> &'static str) = routes[0].func.downcast_ref().ok_or("none")?;

    println!("Hello {}!", fun());

    Ok(())
}