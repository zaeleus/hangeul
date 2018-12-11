use hangeul;

use std::env;

fn main() {
    let input = env::args().skip(1).next().expect("Usage: hangeul <input>");
    println!("{}", hangeul::romanize(&input));
}
