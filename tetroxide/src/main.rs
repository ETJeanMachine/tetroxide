use std::process;
use tetroxide::tetroxide::Game;

fn main() -> Result<(), std::io::Error> {
    let mut tet = Game::new();
    match tet.run() {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Unexpected error occurred: {}", err);
            process::exit(1);
        }
    }
}
