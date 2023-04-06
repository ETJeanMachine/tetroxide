#[cfg(test)]

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

mod test_rotate {
    use tetris::tetris::{Tetris, Tetromino};

    use super::*;
    #[test]
    fn test_rotate() -> () {
        let mut t = Tetris::new(None, Some(Tetromino::I));

        t.rotate(true);

        let s = standard_strip(&mut t.to_string());

        println!("{}", s);
    }
}
