#[cfg(test)]
use tetris::tetris::{Tetris, Tetromino};
use std::fs;

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
    todo!();
}

mod test_new {
    use super::*;
    #[test]
    fn test_new() -> () {
        let  t = Tetris::new(None, Some(Tetromino::I));
        let mut s = standard_strip(&mut t.to_string());
        s.retain(|c| c != '\n');
        s.retain(|c| c != '\r');

      
        let mut f  = fs::read_to_string("tests\\assets\\testnew.txt").unwrap();
        f.retain(|c| c != '\n');
        f.retain(|c| c != '\r');
     
        assert_eq!(f, s);
    }
}

mod test_scoring{

    use super::*;
    #[test]
    fn test_t_spin_score() -> (){

        // load the game state, ignore any possibly generated score at this point
        let mut board_as_string  = fs::read_to_string("tests\\assets\\testnew.txt").unwrap();

       
      


    }
}

// TODO: make unit tests for scoring
// use frame advance

