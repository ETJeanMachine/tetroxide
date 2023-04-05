#[cfg(test)]
mod good_get_squares {
    use super::*;
    #[test]
    fn good_get_squares() -> () {
        let board = [[0; 10]; 40];  
        let mut bag = Bag::new();
        let piece =  ActivePiece {
            tetromino: Tetromino::I,
            origin: Pos(0,1),
            rotation: State::Right,
        };

        let ans = [Pos(0,0),Pos(0,1),Pos(0,2),Pos(0,3)];
        assert!(piece.get_squares() == ans);
    }
}
