/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Ordering;

const CARD_VALUES: [&'static str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

fn rank_of_value<'a>(value: &'a str) -> Option<usize> {
    CARD_VALUES.iter().position(|&r| r == value)
}

fn compare_rank_of_values<'a, 'b>(a: &'a Option<usize>, b: &'b Option<usize>) -> Ordering {
    a.cmp(b)
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // unimplemented!("Out of {hands:?}, which hand wins?")
    hands.to_vec()
}

enum PokerHands{
    FiveOfAKind{value: Option<usize>},
    StraightFlush{high_value: Option<usize>},
    FourOfAKind{value: Option<usize>, other_card: Option<usize>},
    FullHouse{triplet_value: Option<usize>, pair_value: Option<usize>},
    Flush{values: Vec<Option<usize>>},
    Stright{high_value: Option<usize>},
    ThreeOfAKind{value: Option<usize>, other_cards: Vec<Option<usize>>},
    TwoPair{value1: Option<usize>, value2: Option<usize>, other_card: Option<usize>},
    OnePair{value: Option<usize>, other: Vec<Option<usize>>},
    HighCard{value: Vec<Option<usize>>}
}

struct Hand<'a> {
    hand: &'a str,
    evaluation: PokerHands,
}

fn EvaluateHand<'a>(hand: &'a str) -> PokerHands {
    let split = hand.split(' ').collect::<Vec<_>>();
    let mut values: Vec<Option<usize>> = Vec::with_capacity(split.capacity());
    let mut suits: Vec<char> = Vec::with_capacity(split.capacity());
    for s in &split {
        // let mut s = String::new();
        // v[0..v.len()-1].clone_into(&mut s);
        values.push(rank_of_value(&s[0..s.len()-1]));
        values.sort_unstable_by(compare_rank_of_values);
        suits.push(s.chars().last().unwrap());
    }
    // Check for a flush
    let flush = suits.iter().all(|&item| item == suits[0]);

    PokerHands::HighCard{value: values}
}
impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        match self.evaluation {
            PokerHands::FiveOfAKind{value: value1} => {
                match other.evaluation {
                    PokerHands::FiveOfAKind{value: value2} => {
                        if Some(&value1) == Some(&value2) {
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