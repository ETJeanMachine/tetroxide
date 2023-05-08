use clap::Parser;
use std::io::{self, BufRead};
use tetris::tetris::Tetris;
use tetroxide::tetroxide::Game;
use futures::executor::block_on;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    if args.debug {
        print!("\x1B[2J\x1B[1;1H");
        let mut tet = Tetris::default();
        tet.set_level(13);
        // We can rotate and not drop for a limited amount of time in
        // debug mode.
        println!("{}", tet);
        while !tet.is_game_over {
            println!("Input: (w - hold | q & e - rotate | a & d - shift | s - soft drop | z - hard drop)");
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            handle.read_line(&mut buffer)?;
            print!("\x1B[2J\x1B[1;1H");
            let lower = buffer.to_lowercase();
            let trimmed = lower.trim();
            match trimmed {
                "w" => tet.hold(),
                "q" => tet.rotate(true),
                "e" => tet.rotate(false),
                "a" => tet.shift(true),
                "d" => tet.shift(false),
                "s" => tet.soft_drop(),
                "z" => tet.hard_drop(),
                _ => {}
            }
            tet.frame_advance();
            println!("{}", tet);
        }
    } else {
        let mut game = Game::new();
        block_on(game.run())?;
    }
    Ok(())
}
