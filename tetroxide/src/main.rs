use std::io::{self, BufRead};
use std::process;
use tetris::tetris::Tetris;
use tetroxide::tetroxide::Game;

fn main() -> Result<(), std::io::Error> {
    print!("\x1B[2J\x1B[1;1H");
    let mut tet = Tetris::default();
    // We can rotate and not drop for a limited amount of time in
    // debug mode.
    let mut frame_count = 0;
    const MAX_FRAMES: usize = 3;
    println!("{}", tet);
    while !tet.is_game_over {
        println!("Input: (w - hold | q & e - rotate | a & d - shift | s - hard drop | enter - soft drop)");
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer)?;
        print!("\x1B[2J\x1B[1;1H");
        let lower = buffer.to_lowercase();
        match lower.trim() {
            "w" => tet.hold(),
            "q" | "e" => {
                if lower.trim() == "q" {
                    tet.rotate(true);
                } else {
                    tet.rotate(false)
                }
                if frame_count < MAX_FRAMES {
                    frame_count += 1;
                    println!("{}", tet);
                    continue;
                }
                frame_count = 0;
            }
            "a" => tet.shift(true),
            "d" => tet.shift(false),
            "s" | _ => {
                if lower.trim() == "s" {
                    tet.hard_drop();
                } else {
                    tet.drop();
                }
                tet.clear_lines();
                println!("{}", tet);
                continue;
            }
        }
        tet.drop();
        println!("{}", tet);
    }
    Ok(())
}
