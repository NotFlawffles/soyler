pub mod utils;
pub mod diagnostic;
pub mod text;

use std::env::args;

fn usage() -> i32 {
    eprintln!("usage: soyc <file name>");
    1
}

fn main() -> Result<(), i32> {
    let arguments = args().collect::<Vec<_>>();

    if arguments.len() < 2 {
        return Err(usage());
    };

    Ok(())
}
