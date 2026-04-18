mod layout;

use std::fs;

fn main() {
    let svg = layout::build_svg(800, 200);
    fs::write("card.svg", &svg).expect("failed to write card.svg");
    println!("Generated → card.svg");
}
