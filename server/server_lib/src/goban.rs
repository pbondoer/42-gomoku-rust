use crate::types::*;

//Goban Methods
pub static ERR_OUTSIDE_BOARD: Error = "Move outside goban board";
pub static ERR_NOT_EMPTY: Error = "Intersection is not empty";

type Align = (Delta, Delta);

static ALIGNMENTS: &'static [Align; 8] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl Goban {
    pub fn new(size: GobanSize) -> Goban {
        Goban {
            size: size as Size,
            board: vec![Intersection::None; size as Size * size as Size],
        }
    }

    fn is_move_valid(&self, pos: Move) -> Result<(), Error> {
        // out of bounds check
        if pos.0 >= self.size || pos.1 >= self.size {
            return Err(ERR_OUTSIDE_BOARD);
        }

        // TODO: implement other case of invalid move detection
        //  - double-three case

        // check if it is empty
        match self.get(pos) {
            Ok(Intersection::None) => Ok(()),
            Ok(_) => Err(ERR_NOT_EMPTY),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn check_alignment(
        &self,
        pos: Move,
        align: Align,
        multiplier: Delta,
        expected: Intersection,
    ) -> Option<Move> {
        let x = pos.0 as Delta + align.0 * multiplier;
        let y = pos.1 as Delta + align.1 * multiplier;

        if x < 0 || y < 0 || multiplier < 1 {
            return None;
        }

        let new_pos = (x as Size, y as Size);

        match self.get(new_pos) {
            Ok(value) if value == expected => Some(new_pos),
            _ => None,
        }
    }

    fn check_capture(&mut self, pos: Move, player: Intersection) -> Delta {
        let enemy = if player == Intersection::Player1 {
            Intersection::Player2
        } else {
            Intersection::Player1
        };

        let mut captured = 0;

        for align in ALIGNMENTS {
            let enemy1 = self.check_alignment(pos, *align, 1, enemy);
            let enemy2 = self.check_alignment(pos, *align, 2, enemy);

            if enemy1 == None
                || enemy2 == None
                || self.check_alignment(pos, *align, 3, player) == None
            {
                continue;
            }

            // enemy1 and enemy2 are valid positions, so we can safely unwrap
            self.set(enemy1.unwrap(), Intersection::None).unwrap();
            self.set(enemy2.unwrap(), Intersection::None).unwrap();

            captured += 2;
        }

        captured
    }

    //pos Tuple => 0 = line | 1 = column
    pub fn set(&mut self, pos: Move, player: Intersection) -> Result<(), Error> {
        if pos.0 > self.size || pos.1 > self.size {
            return Err(ERR_OUTSIDE_BOARD);
        }

        self.board[pos.0 * self.size + pos.1] = player;
        Ok(())
    }

    pub fn get(&self, pos: Move) -> Result<Intersection, Error> {
        if pos.0 >= self.size || pos.1 >= self.size {
            return Err(ERR_OUTSIDE_BOARD);
        }

        // Position is always valid, so it is safe to unwrap
        Ok(self.board[pos.0 * self.size + pos.1])
    }

    pub fn play(&mut self, player: Intersection, pos: Move) -> Result<Delta, Error> {
        let mut changed = 1;

        self.is_move_valid(pos)?;
        changed -= self.check_capture(pos, player);

        self.set(pos, player)?;
        Ok(changed)
    }
}

//Tests
#[cfg(test)]
mod test {
    use super::*;
    use crate::types::Intersection::Player1;
    use crate::types::Intersection::Player2;

    static GOBAN_SIZES: &'static [GobanSize; 3] =
        &[GobanSize::Small, GobanSize::Medium, GobanSize::Large];

    // new
    #[test]
    fn new_working() {
        for val in GOBAN_SIZES {
            let goban = Goban::new(*val);

            let size = *val as Size;

            assert_eq!(goban.size, size);
            assert_eq!(Intersection::None, goban.board[size * size - 1]);
        }
    }

    // set
    #[test]
    fn set_valid() {
        let mut goban = Goban::new(GobanSize::Large);

        assert_eq!(Ok(()), goban.set((10, 10), Player1));
    }

    #[test]
    fn set_out_of_bounds() {
        let mut goban = Goban::new(GobanSize::Large);

        assert_eq!(Err(ERR_OUTSIDE_BOARD), goban.set((10, 42), Player1));
        assert_eq!(Err(ERR_OUTSIDE_BOARD), goban.set((42, 10), Player1));
    }

    // get
    #[test]
    fn get_valid() {
        let goban = Goban::new(GobanSize::Large);

        assert_eq!(Ok(Intersection::None), goban.get((10, 10)));
    }

    #[test]
    fn get_out_of_bounds() {
        let goban = Goban::new(GobanSize::Large);

        assert_eq!(Err(ERR_OUTSIDE_BOARD), goban.get((19, 10)));
        assert_eq!(Err(ERR_OUTSIDE_BOARD), goban.get((10, 19)));
    }

    // check_capture
    #[test]
    fn check_capture_valid() {
        let mut goban = Goban::new(GobanSize::Large);

        // X O O X is a capture for X
        goban.set((0, 0), Player1).unwrap();
        goban.set((0, 1), Player2).unwrap();
        goban.set((0, 2), Player2).unwrap();
        goban.set((0, 3), Player1).unwrap();

        assert_eq!(2, goban.check_capture((0, 3), Player1));

        // check that the capture has been performed
        assert_eq!(Ok(Player1), goban.get((0, 0)));
        assert_eq!(Ok(Intersection::None), goban.get((0, 1)));
        assert_eq!(Ok(Intersection::None), goban.get((0, 2)));
        assert_eq!(Ok(Player1), goban.get((0, 3)));
    }

    #[test]
    fn check_capture_double() {
        let mut goban = Goban::new(GobanSize::Large);

        goban.set((0, 0), Player1).unwrap();
        goban.set((0, 1), Player2).unwrap();
        goban.set((0, 2), Player2).unwrap();
        goban.set((0, 3), Player1).unwrap();
        goban.set((0, 4), Player2).unwrap();
        goban.set((0, 5), Player2).unwrap();
        goban.set((0, 6), Player1).unwrap();

        assert_eq!(4, goban.check_capture((0, 3), Player1));

        // check that the capture has been performed
        assert_eq!(Ok(Player1), goban.get((0, 0)));
        assert_eq!(Ok(Intersection::None), goban.get((0, 1)));
        assert_eq!(Ok(Intersection::None), goban.get((0, 2)));
        assert_eq!(Ok(Player1), goban.get((0, 3)));
        assert_eq!(Ok(Intersection::None), goban.get((0, 4)));
        assert_eq!(Ok(Intersection::None), goban.get((0, 5)));
        assert_eq!(Ok(Player1), goban.get((0, 6)));
    }

    #[test]
    fn check_capture_none() {
        let mut goban = Goban::new(GobanSize::Large);

        // X O X is not a capture
        goban.set((0, 0), Player1).unwrap();
        goban.set((0, 1), Player2).unwrap();

        assert_eq!(0, goban.check_capture((0, 2), Player1));

        // X O . X is not a capture
        assert_eq!(0, goban.check_capture((0, 3), Player1));

        // X O O O X is not a capture
        goban.set((0, 2), Player2).unwrap();
        goban.set((0, 3), Player2).unwrap();

        assert_eq!(0, goban.check_capture((0, 4), Player1));
    }

    // check_alignment
    #[test]
    fn check_alignment_none() {
        let goban = Goban::new(GobanSize::Large);

        // invalid usage
        assert_eq!(None, goban.check_alignment((0, 0), (1, 0), -1, Player1));

        // out of bounds
        assert_eq!(None, goban.check_alignment((0, 0), (-1, 0), 1, Player1));
        assert_eq!(None, goban.check_alignment((0, 0), (0, -1), 1, Player1));
        assert_eq!(None, goban.check_alignment((18, 18), (1, 0), 1, Player1));
        assert_eq!(None, goban.check_alignment((18, 18), (0, 1), 1, Player1));
    }

    #[test]
    fn check_alignment_expected() {
        let mut goban = Goban::new(GobanSize::Large);

        goban.set((0, 1), Player1).unwrap();

        // should return the position it checks for
        assert_eq!(
            Some((0, 1)),
            goban.check_alignment((0, 0), (0, 1), 1, Player1)
        );
        // should return None if the expected doesn't match
        assert_eq!(None, goban.check_alignment((0, 0), (0, 1), 1, Player2));
    }

    // is_move_valid
    #[test]
    fn is_move_valid() {
        let mut goban = Goban::new(GobanSize::Large);

        goban.set((1, 1), Player1).unwrap();

        assert_eq!(Ok(()), goban.is_move_valid((0, 0)));
        assert_eq!(Err(ERR_NOT_EMPTY), goban.is_move_valid((1, 1)));
        assert_eq!(Err(ERR_OUTSIDE_BOARD), goban.is_move_valid((100, 1)));
        assert_eq!(Err(ERR_OUTSIDE_BOARD), goban.is_move_valid((1, 100)));
    }

    // play
    #[test]
    fn play_out_of_bounds() {
        let mut goban = Goban::new(GobanSize::Medium);

        assert_eq!(Err(ERR_OUTSIDE_BOARD), goban.play(Player1, (14, 0)));
        assert_eq!(Err(ERR_OUTSIDE_BOARD), goban.play(Player1, (0, 14)));
    }

    #[test]
    fn play_valid() {
        let mut goban = Goban::new(GobanSize::Large);

        assert_eq!(Ok(Intersection::None), goban.get((10, 5)));
        assert_eq!(Ok(1), goban.play(Player1, (10, 5)));
        assert_eq!(Ok(Player1), goban.get((10, 5)));
    }
}
