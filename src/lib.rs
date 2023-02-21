/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Ordering;

const CARD_VALUES: [&'static str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

fn rank_of_value<>(value: & str) -> Option<usize> {
    CARD_VALUES.iter().position(|&r| r == value)
}

fn compare_rank_of_values(a: &Option<usize>, b: &Option<usize>) -> Ordering {
    a.cmp(b)
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // unimplemented!("Out of {hands:?}, which hand wins?")
    for h in hands {
        println!("{:?}", &h);
        evaluate_hand(&h);
    }
    hands.to_vec()
}

enum PokerHands{
    FiveOfAKind{value: Option<usize>},
    StraightFlush{high_value: Option<usize>},
    FourOfAKind{value: Option<usize>, other_card: Option<usize>},
    FullHouse{triplet_value: Option<usize>, pair_value: Option<usize>},
    Flush{values: Vec<Option<usize>>},
    Straight{high_value: Option<usize>},
    ThreeOfAKind{value: Option<usize>, other_cards: Vec<Option<usize>>},
    TwoPair{value1: Option<usize>, value2: Option<usize>, other_card: Option<usize>},
    OnePair{value: Option<usize>, other: Vec<Option<usize>>},
    HighCard{value: Vec<Option<usize>>}
}

struct Hand<'a> {
    hand: &'a str,
    evaluation: PokerHands,
}

fn is_n_matching(values: &Vec<Option<usize>>, start: usize, count: usize) -> bool {
    (&values[start..start+count]).iter().all(|&item| item.unwrap() == values[0].unwrap())
}

fn is_straight(values: &Vec<Option<usize>>) -> bool {
    values.iter().enumerate().all(|(i,&x)| x.unwrap() == values[0].unwrap()+i)
}

fn is_low_ace_straight(values: &Vec<Option<usize>>) -> bool {
    if values[values.len()-1].unwrap() == CARD_VALUES.len()-1 {
        (&values[0..values.len() - 1]).iter().enumerate().all(|(i, &x)| x.unwrap() == values[0].unwrap() + i)
    } else {
        false
    }
}

fn is_flush(suits: &Vec<char>) -> bool {
    suits.iter().all(|&item| item == suits[0])
}

fn evaluate_hand(hand: & str) -> PokerHands {
    println!("Evaluating hand {:?}", &hand);
    let split = hand.split(' ').collect::<Vec<_>>();
    let mut values: Vec<Option<usize>> = Vec::with_capacity(split.capacity());
    let mut suits: Vec<char> = Vec::with_capacity(split.capacity());
    for s in &split {
        // let mut s = String::new();
        // v[0..v.len()-1].clone_into(&mut s);
        values.push(rank_of_value(&s[0..s.len()-1]));
        suits.push(s.chars().last().unwrap());
    }
    values.sort_unstable_by(compare_rank_of_values);
    println!("{:?}", values);
    println!("{:?}", suits);
    let mut evaluation: PokerHands;

    // Check for a straight
    // let straight = values.iter().enumerate().all(|(i,&x)| x.unwrap() == values[0].unwrap()+i);
    let straight = is_straight(&values);
    let low_ace_straight = is_low_ace_straight(&values);
    let flush = is_flush(&suits);
    let mut four_of_a_kind: Option<PokerHands> = Option::None;
    if is_n_matching(&values, 0, 4) {
        four_of_a_kind = Some(PokerHands::FourOfAKind {value: values[0], other_card: values[1]});
    } else if is_n_matching(&values, 1, 4) {
        four_of_a_kind = Some(PokerHands::FourOfAKind {value: values[1], other_card: values[0]});
    }
    // Check full house and three of a kind
    let mut full_house: Option<PokerHands> = Option::None;
    let mut three_of_a_kind: Option<PokerHands> = Option::None;
    if is_n_matching(&values, 0, 3) {
        if is_n_matching(&values, 3, 2) {
            full_house = Some(PokerHands::FullHouse {triplet_value: values[0], pair_value: values[3]});
        } else {
            three_of_a_kind = Some(PokerHands::ThreeOfAKind {value: values[0], other_cards: values[3..5].to_vec()});
        }
    } else if is_n_matching(&values, 1, 3) {
        three_of_a_kind = Some(PokerHands::ThreeOfAKind {value: values[1], other_cards: vec![values[0], values[4]]});
    } else if is_n_matching(&values, 2, 3) {
        if is_n_matching(&values, 0, 2) {
            full_house = Some(PokerHands::FullHouse {triplet_value: values[2], pair_value: values[0]});
        } else {
            three_of_a_kind = Some(PokerHands::ThreeOfAKind {value: values[2], other_cards: values[0..2].to_vec()});
        }
    }

    // Set evaluation
    if is_n_matching(&values, 0, 5) {
        evaluation = PokerHands::FiveOfAKind {value: values[0]};
    } else if straight && flush {
        evaluation = PokerHands::StraightFlush{high_value: values[values.len()-1]};
        println!("Straight flush with max value {}", CARD_VALUES[values[values.len()-1].unwrap()]);
    } else if low_ace_straight && flush {
        evaluation = PokerHands::StraightFlush{high_value: values[values.len()-2]};
        println!("Straight flush with max value {}", CARD_VALUES[values[values.len()-2].unwrap()]);
    } else if four_of_a_kind.is_some(){
        println!("Four of a kind");
        evaluation = four_of_a_kind.unwrap();
    } else if full_house.is_some() {
        println!("Full House");
        evaluation = full_house.unwrap();
    } else if flush {
        println!("Flush");
        evaluation = PokerHands::Flush{values: values};
    } else if straight {
        println!("Straight");
        evaluation = PokerHands::Straight { high_value: values[values.len() - 1] };
    } else if low_ace_straight {
        println!("Straight");
        evaluation = PokerHands::Straight { high_value: values[values.len() - 2] };
    } else if three_of_a_kind.is_some() {
        println!("Three of a kind");
        evaluation = three_of_a_kind.unwrap();
    } else { // Pairs or high cards
        if is_n_matching(&values, 0, 2) {
            if is_n_matching(&values, 2, 2) {
                println!("Two pair {} over {}", CARD_VALUES[values[2].unwrap()], CARD_VALUES[values[0].unwrap()]);
                evaluation = PokerHands::TwoPair {value1: values[2], value2: values[0], other_card: values[4]};
            } else if is_n_matching(&values, 3, 2) {
                println!("Two pair {} over {}", CARD_VALUES[values[3].unwrap()], CARD_VALUES[values[0].unwrap()]);
                evaluation = PokerHands::TwoPair {value1: values[3], value2: values[0], other_card: values[2]};
            } else {
                println!("One pair of  {}", CARD_VALUES[values[0].unwrap()]);
                evaluation = PokerHands::OnePair {value: values[0], other: values[2..5].to_vec()};
            }
        } else if is_n_matching(&values, 1, 2) {
            if is_n_matching(&values, 3, 2) {
                println!("Two pair {} over {}", CARD_VALUES[values[3].unwrap()], CARD_VALUES[values[1].unwrap()]);
                evaluation = PokerHands::TwoPair {value1: values[3], value2: values[1], other_card: values[0]};
            } else {
                println!("One pair of  {}", CARD_VALUES[values[3].unwrap()]);
                evaluation = PokerHands::OnePair {value: values[3], other: vec![values[0],values[3], values[4]]};
            }
        } else if is_n_matching(&values, 2, 2) {
            println!("One pair of  {}", CARD_VALUES[values[2].unwrap()]);
            evaluation = PokerHands::OnePair {value: values[2], other: vec![values[0], values[1], values[4]]};
        } else if is_n_matching(&values, 3, 2) {
            println!("One pair of  {}", CARD_VALUES[values[3].unwrap()]);
            evaluation = PokerHands::OnePair {value: values[2], other: values[0..3].to_vec()};
        } else {
            println!("High value with max value {}", CARD_VALUES[values[values.len() - 1].unwrap()]);
            evaluation = PokerHands::HighCard { value: values }
        }
    }
    evaluation
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