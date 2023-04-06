#[cfg(test)]

pub fn fml(s: &mut String)  ->  Vec<&str>   {
    // extracts the 'board' out from the UI. 
    // skip first row, get chars 10-34 until row 21 (22 includes ghosting line)
    let mut stripped : Vec<&str> = vec!();
    let mut l = s.lines();
 
    l.next();
    for _rows in 1..22{
        let curr_row = l.next().expect("Will never have less than 22 lines.");
        stripped.push(&curr_row[10..34]);
    }
    stripped
}
    
pub fn standard_strip(s: &mut String)  ->  String  {
    // extracts the 'board' out from the UI. 
    // skip first row, get chars 10-34 until row 21 (22 includes ghosting line)
    let mut stripped = String::new();
    let mut l = s.lines();
 
    l.next();
    for _rows in 1..22{
        let curr_row = l.next().expect("Will never have less than 22 lines.");
        stripped.push_str(format!("{:.*}\n", 10, 34).as_str());
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

        println!("{}", t.to_string());

        let s = standard_strip(&mut t.to_string());

        println!("{}", s);
    }
}
