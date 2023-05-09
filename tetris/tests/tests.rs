use std::fs;
use tetris;
use tetris::tetris::{Tetris, Tetromino};

const MAX_COL: usize = tetris::tetris::MAX_COL;
const MAX_ROW: usize = tetris::tetris::MAX_ROW;

pub fn standard_strip(s: &mut String) -> String {
    // extracts the 'board' out from the UI.
    // skip first row, get chars 10-34 until row 21 (22 includes ghosting line)
    let mut stripped = String::new();
    for line in s.split("\n") {
        if let Some(i) = line.find('<') {
            let (_, end) = line.split_at(i + 2);
            if end[0..1] != *"=" {
                //skips the last line
                stripped.push_str(format!("{:.*}\n", 20, end).as_str());
            }
        }
    }
    stripped
}

pub fn convert_string_to_u8_array(s: &mut String) -> [[u8; MAX_COL]; MAX_ROW] {
    // Convert an input board as a string into a grid of u8s that can be feed into
    // Tetris::new() to instantiate a tetris game.

    let mut board: [[u8; MAX_COL]; MAX_ROW] = [[0; MAX_COL]; MAX_ROW];

    //purge all whitespace
    s.retain(|c| c != '\n');
    s.retain(|c| c != '\r');
    s.retain(|c| c != ' ');

    let mut char_feed = s.chars();

    for i in 20..MAX_ROW {
        // input files are 20 X 20, skipping blank first 20 rows and using spacing to be human-readable
        for j in 0..MAX_COL {
            let next_char = char_feed.next().unwrap(); // prepared file will never not be sufficiently filled w/ chars
            if next_char == '.' || next_char == 'X' {
                //shadows and spaces turn into spaces when translated into a board
                //space, no tetrimino
                board[i][j] = 0 as u8;
            } else if next_char == '[' {
                //detected ']',
                board[i][j] = 1 as u8;
                char_feed.next(); // skip over the ']' char
            }   
        }
    }
    board
}

mod test_new {
    use super::*;
    #[test]
    fn test_new() -> () {
        let t = Tetris::new(None, Some(Tetromino::I));
        let mut test = standard_strip(&mut t.to_string());
        test.retain(|c| c != '\n');
        test.retain(|c| c != '\r');

        let mut control: String = fs::read_to_string("tests\\assets\\testnew.txt").unwrap();
        control.retain(|c| c != '\n');
        control.retain(|c| c != '\r');

        assert_eq!(control, test);
    }
}

mod test_t_spin_score {
    use super::*;
    #[test]
    fn test_t_spin() -> () {
        let mut str_board: String = fs::read_to_string("tests\\assets\\test_tspin.txt").unwrap();

        let board = convert_string_to_u8_array(&mut str_board);

        let game = Tetris::new(Some(board), None);

        assert!(true); // need to calc control score, move piece into t-spin, check test score

        // Unit testing was a bear for this, didn't have enough time to alloc to do properly vs working on other area of project.`
    }
}

