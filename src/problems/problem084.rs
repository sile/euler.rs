//! [84] Monopoly odds
//! ------------------
//!
extern crate rand;

use self::rand::Rng;

struct Dice {
    side: usize,
}

impl Dice {
    fn roll(&mut self) -> (usize, usize) {
        (rand::random::<usize>() % self.side + 1,
         rand::random::<usize>() % self.side + 1)
    }
}

struct Square {
    name: &'static str,
    kind: SquareKind,
}

impl Square {
    fn normal(name: &'static str) -> Self {
        Square {
            name: name,
            kind: SquareKind::Normal,
        }
    }
    fn cc(name: &'static str) -> Self {
        Square {
            name: name,
            kind: SquareKind::CommunityChest,
        }
    }
    fn ch(name: &'static str) -> Self {
        Square {
            name: name,
            kind: SquareKind::Chance,
        }
    }
    fn g2j(name: &'static str) -> Self {
        Square {
            name: name,
            kind: SquareKind::GoToJail,
        }
    }
}

#[derive(Clone,Copy)]
enum SquareKind {
    CommunityChest,
    Chance,
    GoToJail,
    Normal,
}

const BOARD_LEN: usize = 40;
fn board() -> [Square; BOARD_LEN] {
    [Square::normal("GO"),
     Square::normal("A1"),
     Square::cc("CC1"),
     Square::normal("A2"),
     Square::normal("T1"),
     Square::normal("R1"),
     Square::normal("B1"),
     Square::ch("CH1"),
     Square::normal("B2"),
     Square::normal("B3"),
     Square::normal("JAIL"),
     Square::normal("C1"),
     Square::normal("U1"),
     Square::normal("C2"),
     Square::normal("C3"),
     Square::normal("R2"),
     Square::normal("D1"),
     Square::cc("CC2"),
     Square::normal("D2"),
     Square::normal("D3"),
     Square::normal("FP"),
     Square::normal("E1"),
     Square::ch("CH2"),
     Square::normal("E2"),
     Square::normal("E3"),
     Square::normal("R3"),
     Square::normal("F1"),
     Square::normal("F2"),
     Square::normal("U2"),
     Square::normal("F3"),
     Square::g2j("G2J"),
     Square::normal("G1"),
     Square::normal("G2"),
     Square::cc("CC3"),
     Square::normal("G3"),
     Square::normal("R4"),
     Square::ch("CH3"),
     Square::normal("H1"),
     Square::normal("T2"),
     Square::normal("H2")]
}

#[derive(Clone)]
enum Card {
    GoTo(&'static str),
    GoNext(&'static str),
    GoBack(usize),
    Noop,
}

struct Pile {
    index: usize,
    cards: [Card; 16],
}

impl Pile {
    fn cc() -> Self {
        let mut cards = [Card::GoTo("GO"),
                         Card::GoTo("JAIL"),
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop];
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut cards);
        Pile {
            index: 0,
            cards: cards,
        }
    }
    fn ch() -> Self {
        let mut cards = [Card::GoTo("GO"),
                         Card::GoTo("JAIL"),
                         Card::GoTo("C1"),
                         Card::GoTo("E3"),
                         Card::GoTo("H2"),
                         Card::GoTo("R1"),
                         Card::GoNext("R"),
                         Card::GoNext("R"),
                         Card::GoNext("U"),
                         Card::GoBack(3),
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop,
                         Card::Noop];
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut cards);
        Pile {
            index: 0,
            cards: cards,
        }
    }
    fn take(&mut self) -> Card {
        let card = self.cards[self.index].clone();
        self.index = (self.index + 1) % 16;
        card
    }
}

struct Player {
    position: usize,
    consecutive_doubles: usize,
    board: [Square; BOARD_LEN],
    cc_pile: Pile,
    ch_pile: Pile,
}

impl Player {
    fn new() -> Self {
        Player {
            position: 0,
            consecutive_doubles: 0,
            board: board(),
            cc_pile: Pile::cc(),
            ch_pile: Pile::ch(),
        }
    }
    fn mov(&mut self, roll: (usize, usize)) {
        if roll.0 == roll.1 {
            self.consecutive_doubles += 1;
            if self.consecutive_doubles == 3 {
                self.position = self.board.iter().position(|x| x.name == "JAIL").unwrap();
                return;
            }
        } else {
            self.consecutive_doubles = 0;
        }

        let distance = roll.0 + roll.1;
        let next = (self.position + distance) % self.board.len();
        let kind = self.board[next].kind;
        self.position = next;
        match kind {
            SquareKind::Normal => {}
            SquareKind::GoToJail => {
                self.position = self.board.iter().position(|x| x.name == "JAIL").unwrap();
            }
            SquareKind::CommunityChest => {
                let card = self.cc_pile.take();
                self.follow_instruction(card);
            }
            SquareKind::Chance => {
                let card = self.ch_pile.take();
                self.follow_instruction(card);
            }
        }
    }
    fn follow_instruction(&mut self, card: Card) {
        match card {
            Card::GoTo(name) => {
                self.position = self.board.iter().position(|x| x.name == name).unwrap();
            }
            Card::GoNext(prefix) => {
                let start = (self.position + 1) % self.board.len();
                let position = start +
                               self.board
                                   .iter()
                                   .skip(start)
                                   .chain(self.board.iter())
                                   .position(|x| x.name.starts_with(prefix))
                                   .unwrap();
                self.position = position % self.board.len();
            }
            Card::GoBack(n) => {
                if self.position >= n {
                    self.position -= n;
                } else {
                    self.position = self.board.len() - (n - self.position);
                }
            }
            Card::Noop => {}
        }
    }
}

pub fn solve() -> usize {
    let mut dice = Dice { side: 4 };
    let samples = 10_000_000;
    let mut counts: [usize; BOARD_LEN] = [0; BOARD_LEN];
    let mut player = Player::new();
    for _ in 0..samples {
        player.mov(dice.roll());
        counts[player.position] += 1;
    }

    {
        use std::str::FromStr;
        let mut result = counts.iter().enumerate().collect::<Vec<_>>();
        result.sort_by_key(|x| x.1);
        result.reverse();
        usize::from_str(&format!("{:0>2}{:0>2}{:0>2}", result[0].0, result[1].0, result[2].0))
            .unwrap()
    }
}
