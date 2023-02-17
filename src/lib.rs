/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Ordering;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // unimplemented!("Out of {hands:?}, which hand wins?")
    hands.to_vec()
}

enum PokerHands{
    FiveOfAKind{value: char},
    StraightFlush{high_value: char, suit: char},
    FourOfAKind{value: char, other_card:char},
    FullHouse{triplet_value: char, pair_value: char},
    Flush{values: Vec<char>},
    Stright{high_value: char},
    ThreeOfAKind{value: char, other_cards: Vec<char>},
    TwoPair{value1: char, value2: char, other_card: char},
    OnePair{value: char, other: Vec<char>},
    HighCard{value: Vec<char>}
}

struct Hand<'a> {
    hand: &'a str,
    evaluation: PokerHands,
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}