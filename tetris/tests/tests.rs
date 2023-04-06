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
