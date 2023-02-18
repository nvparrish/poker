/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Ordering;

const CARD_VALUES: [&'static str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // unimplemented!("Out of {hands:?}, which hand wins?")
    hands.to_vec()
}

enum PokerHands<'a>{
    FiveOfAKind{value: &'a str},
    StraightFlush{high_value: &'a str},
    FourOfAKind{value: &'a str, other_card:&'a str},
    FullHouse{triplet_value: &'a str, pair_value: &'a str},
    Flush{values: Vec<&'a str>},
    Stright{high_value: &'a str},
    ThreeOfAKind{value: &'a str, other_cards: Vec<&'a str>},
    TwoPair{value1: &'a str, value2: &'a str, other_card: &'a str},
    OnePair{value: &'a str, other: Vec<&'a str>},
    HighCard{value: Vec<&'a str>}
}

struct Hand<'a> {
    hand: &'a str,
    evaluation: PokerHands<'a>,
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}


fn RankOfValue(value: &str) -> Option<usize> {
    CARD_VALUES.iter().position(|&r| r == value)
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        match self.evaluation {
            PokerHands::FiveOfAKind{value: value1} => {
                match other.evaluation {
                    PokerHands::FiveOfAKind{value: value2} => {
                        if Some(RankOfValue(value1)) == Some(RankOfValue(value2)) {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false
                }
            }
            _ => false
        }
    }
}