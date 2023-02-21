/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Ordering;

// CODE TO ESTABLISH RANK BASED ON CARD VALUES ====================================================
const CARD_VALUES: [&'static str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

fn rank_of_value<>(value: & str) -> Option<usize> {
    CARD_VALUES.iter().position(|&r| r == value)
}

fn compare_rank_of_values(a: &Option<usize>, b: &Option<usize>) -> Ordering {
    a.cmp(b)
}

// FUNCTION FOR WINNING HANDS EVALUATION ==========================================================
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // unimplemented!("Out of {hands:?}, which hand wins?")
    for h in hands {
        println!("{:?}", &h);
        evaluate_hand(&h);
    }
    hands.to_vec()
}

// Enumeration of possible poker hands == ==========================================================
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

// Structure for a hand and its evaluation =========================================================
struct Hand<'a> {
    hand: &'a str,
    evaluation: PokerHands,
}

// UTILITY FUNCTIONS TO FIND PATTERNS ==============================================================

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

// EVALUATION OF HAND =============================================================================
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
    let evaluation: PokerHands;

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

// PartialEq partial_cmp function ==================================================================
impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match &self.evaluation {
            PokerHands::HighCard {value: value1} =>
                match &other.evaluation {
                    PokerHands::HighCard {value: value2} => {
                        let mut result = Ordering::Equal;
                        for (i, j) in value1.iter().zip( value2.iter()) {
                            result =i.cmp(j);
                            if result != Ordering::Equal {break;}
                        }
                        Some(result)
                    }
                    _ => Some(Ordering::Less)
                }
            PokerHands::OnePair {value: value1, other: other1} =>
                match &other.evaluation {
                    PokerHands::HighCard {value: _} => Some(Ordering::Greater),
                    PokerHands::OnePair {value: value2, other: other2} => {
                        if value1 < value2 {Some(Ordering::Less)}
                        else if value1 > value2 {Some(Ordering::Greater)}
                        else {
                            let mut result = Ordering::Equal;
                            for (i, j) in other1.iter().zip( other2.iter()) {
                                result = i.cmp(j);
                                if result != Ordering::Equal {break;}
                            }
                            Some(result)
                        }
                    }
                    _ => Some(Ordering::Less)
                }
            PokerHands::TwoPair {value1: high1, value2: low1, other_card: other1} =>
                match &other.evaluation {
                    PokerHands::HighCard {value: _} |
                    PokerHands::OnePair {value: _, other: _} => Some(Ordering::Greater),
                    PokerHands::TwoPair {value1: high2, value2: low2, other_card: other2} => {
                        let mut result = Some(high1.cmp(high2));
                        if result.unwrap() == Ordering::Equal {result = Some(low1.cmp(low2));}
                        if result.unwrap() == Ordering::Equal {result = Some(other1.cmp(other2));}
                        result
                    }
                    _ => Some(Ordering::Less)
                }
            PokerHands::ThreeOfAKind {value: value1, other_cards: other1} =>
                match &other.evaluation {
                    PokerHands::FiveOfAKind {value: _} |
                    PokerHands::StraightFlush {high_value: _} |
                    PokerHands::FourOfAKind {value: _, other_card: _} |
                    PokerHands::Flush {values: _} |
                    PokerHands::Straight {high_value: _} => Some(Ordering::Less),
                    PokerHands::ThreeOfAKind {value: value2, other_cards: other2} => {
                        let mut result = value1.cmp(value2);
                        if result == Ordering::Equal {
                            for (i, j) in other1.iter().zip(other2.iter()) {
                                result = i.cmp(j);
                                if result != Ordering::Equal { break; }
                            }
                        }
                        Some(result)
                    }
                    _ => Some(Ordering::Greater)
                }
            PokerHands::Straight {high_value: high1} =>
                match &other.evaluation {
                    PokerHands::FiveOfAKind {value: _} |
                    PokerHands::StraightFlush {high_value: _} |
                    PokerHands::FourOfAKind {value: _, other_card: _} |
                    PokerHands::Flush {values: _} => Some(Ordering::Less),
                    PokerHands::Straight {high_value: high2} => Some(high1.cmp(high2)),
                    _ => Some(Ordering::Greater)
                }
            PokerHands::Flush { values: values1 } =>
                match &other.evaluation {
                    PokerHands::FiveOfAKind { value:_ } |
                    PokerHands::StraightFlush { high_value:_ } |
                    PokerHands::FourOfAKind { value:_, other_card:_ } => Some(Ordering::Less),
                    PokerHands::Flush { values: values2 } => {
                        let mut result = Ordering::Equal;
                        for (i, j) in values1.iter().zip( values2.iter()) {
                            result = i.cmp(j);
                            if result != Ordering::Equal {break;}
                        }
                        Some(result)
                    }
                    _ => Some(Ordering::Greater)
                }
            PokerHands::FourOfAKind {value: value1, other_card: other1} =>
                match &other.evaluation {
                    PokerHands::FiveOfAKind {value:_} | PokerHands::StraightFlush {high_value:_} => Some(Ordering::Less),
                    PokerHands::FourOfAKind {value: value2, other_card: other2} => {
                        let mut result = value1.cmp(value2);
                        if result == Ordering::Equal {
                            result = other1.cmp(other2);
                        }
                        Some(result)
                    }
                    _ => Some(Ordering::Greater)
            }
            PokerHands::StraightFlush {high_value: high1} =>
                match &other.evaluation {
                    PokerHands::FiveOfAKind{value:_} => Some(Ordering::Less),
                    PokerHands::StraightFlush {high_value: high2} => Some(high1.cmp(high2)),
                    _ => Some(Ordering::Greater)
            }
            PokerHands::FiveOfAKind{value: value1} =>
                match &other.evaluation {
                    PokerHands::FiveOfAKind {value: value2} => Some(value1.cmp(value2)),
                    _ => Some(Ordering::Greater)
                }
            _ => Option::None
        }
    }
}

// PartialEq eq function ===========================================================================
impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.evaluation, &other.evaluation) {
            (PokerHands::FiveOfAKind{value: value1}, PokerHands::FiveOfAKind {value: value2}) =>
                if value1 == value2 {true} else {false}
            (PokerHands::StraightFlush {high_value: value1}, PokerHands::StraightFlush {high_value: value2}) =>
                if value1 == value2 {true} else {false}
            (PokerHands::FourOfAKind {value: value1, other_card: other1}, PokerHands::FourOfAKind {value:value2, other_card:other2}) =>
                if value1 == value2 && other1 == other2 {true} else {false}
            (PokerHands::FullHouse {triplet_value: triple1, pair_value:pair1}, PokerHands::FullHouse {triplet_value:triple2, pair_value:pair2}) =>
                if triple1 == triple2 && pair1 == pair2 {true} else {false}
            (PokerHands::Flush {values:values1}, PokerHands::Flush{values: values2}) =>
                if values1 == values2 {true} else {false}
            (PokerHands::Straight {high_value: high1}, PokerHands::Straight {high_value:high2}) =>
                if high1 == high2 {true} else {false}
            (PokerHands::ThreeOfAKind {value: value1, other_cards: other1}, PokerHands::ThreeOfAKind {value:value2, other_cards:other2}) =>
                if value1==value2 && other1 == other2 {true} else {false}
            (PokerHands::TwoPair {value1:high1, value2:low1, other_card: other1}, PokerHands::TwoPair{value1:high2, value2:low2, other_card:other2}) =>
                if high1==high2 && low1==low2 && other1==other2 {true} else {false}
            (PokerHands::OnePair{value: value1, other:other1}, PokerHands::OnePair{value:value2, other:other2}) =>
                if value1==value2 && other1==other2 {true} else {false}
            (PokerHands::HighCard {value:value1}, PokerHands::HighCard {value:value2}) =>
                if value1==value2 {true} else {false}
            _ => false
        }
    }
}