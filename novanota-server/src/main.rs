use rustigo::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rustigo = Rustigo::default();

    Ok(rustigo.listen("0.0.0.0:2500", 4)?)
}
