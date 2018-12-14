use hangeul;

use std::env;

fn main() {
    let input = env::args().nth(1).expect("Usage: hangeul <input>");
    println!("{}", hangeul::romanize(&input));
}
