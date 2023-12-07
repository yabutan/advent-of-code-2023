use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;
use nom::character::complete;
use nom::character::complete::{alphanumeric1, newline, space1};
use nom::multi::separated_list1;

use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct InputLine<'a> {
    pub cards: &'a str,
    pub bid: u32,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NormalRule;
#[derive(Debug, PartialEq, Eq)]
pub struct JokerRule;

#[derive(Debug)]
pub struct Hand<'a, Rule> {
    cards: &'a str,
    hand_type: HandType,
    orders: Vec<u8>,
    _rule: std::marker::PhantomData<Rule>,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<InputLine>> {
    fn input_line(input: &str) -> IResult<&str, InputLine> {
        let (input, label) = alphanumeric1(input)?;
        let (input, _) = space1(input)?;
        let (input, bid) = complete::u32(input)?;
        Ok((input, InputLine { cards: label, bid }))
    }

    let (input, hands) = separated_list1(newline, input_line)(input)?;
    Ok((input, hands))
}

impl Hand<'_, NormalRule> {
    pub fn new(cards: &str) -> Hand<NormalRule> {
        let hand_type = get_hand_type(cards, false);
        let orders = cards
            .chars()
            .map(|c| get_order(&c, &CARD_ORDER_FOR_NORMAL_RULE))
            .collect::<Vec<_>>();

        Hand {
            cards,
            orders,
            hand_type,
            _rule: std::marker::PhantomData,
        }
    }
}

impl Hand<'_, JokerRule> {
    pub fn new(cards: &str) -> Hand<JokerRule> {
        let hand_type = get_hand_type(cards, true);
        let orders = cards
            .chars()
            .map(|c| get_order(&c, &CARD_ORDER_FOR_JOKER_RULE))
            .collect::<Vec<_>>();

        Hand {
            cards,
            orders,
            hand_type,
            _rule: std::marker::PhantomData,
        }
    }
}

impl<Rule> PartialEq for Hand<'_, Rule> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl<Rule> Eq for Hand<'_, Rule> {}

impl<Rule> Ord for Hand<'_, Rule> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.orders.cmp(&other.orders),
            ordering => ordering,
        }
    }
}
impl<Rule> PartialOrd for Hand<'_, Rule> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const CARD_ORDER_FOR_NORMAL_RULE: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const CARD_ORDER_FOR_JOKER_RULE: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn get_order(card: &char, card_order: &[char]) -> u8 {
    card_order
        .iter()
        .position(|x| x == card)
        .map(|x| x as u8)
        .unwrap_or_else(|| panic!("Invalid card: {}", card))
}

fn get_counts(cards: &str, joker_rule: bool) -> HashMap<char, usize> {
    let counts = cards
        .chars()
        .sorted()
        .group_by(|&c| c)
        .into_iter()
        .map(|(k, v)| (k, v.count()))
        .collect::<HashMap<char, usize>>();

    if !joker_rule {
        return counts;
    }

    // J を一番多い札に差し替える。
    let j = *counts.get(&'J').unwrap_or(&0);
    let max = counts
        .iter()
        .filter(|(c, _)| **c != 'J')
        .max_by_key(|(_, count)| *count)
        .map(|(k, count)| (*k, *count));
    let Some(max) = max else { return counts };

    let mut new_counts = HashMap::new();
    for (card, count) in counts {
        if card == 'J' {
            continue;
        }

        if card == max.0 {
            new_counts.insert(card, max.1 + j);
        } else {
            new_counts.insert(card, count);
        }
    }
    new_counts
}

fn get_hand_type(cards: &str, joker_rule: bool) -> HandType {
    let counts = get_counts(cards, joker_rule);
    println!("{:?}", counts);

    // Five of a kind
    if counts.len() == 1 {
        return HandType::FiveOfKind;
    }

    // Four of a kind
    if counts.len() == 2 && counts.iter().any(|(_, count)| *count == 4) {
        return HandType::FourOfKind;
    }

    // Full house
    if counts.len() == 2
        && counts.iter().any(|(_, count)| *count == 3)
        && counts.iter().any(|(_, count)| *count == 2)
    {
        return HandType::FullHouse;
    }

    // Three of a kind
    if counts.len() == 3 && counts.iter().any(|(_, count)| *count == 3) {
        return HandType::ThreeOfKind;
    }

    // Two pair
    if counts.len() == 3 && counts.iter().any(|(_, count)| *count == 2) {
        return HandType::TwoPair;
    }

    // One pair
    if counts.iter().any(|(_, count)| *count == 2) {
        return HandType::OnePair;
    }

    // High card
    HandType::HighCard
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use itertools::Itertools;

    use super::*;

    const INPUT: &str = indoc! {r#"
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
    "#};

    #[test]
    fn test_parse_input() {
        #[rustfmt::skip]
        let expected = [
            InputLine { cards: "32T3K", bid: 765 },
            InputLine { cards: "T55J5", bid: 684 },
            InputLine { cards: "KK677", bid: 28 },
            InputLine { cards: "KTJJT", bid: 220 },
            InputLine { cards: "QQQJA", bid: 483 },
        ];

        let (_, hands) = parse_input(INPUT).unwrap();
        assert_eq!(hands, expected);
    }

    #[test]
    fn test_get_type() {
        #[rustfmt::skip]
        let patterns = [
            ("AAAAA", HandType::FiveOfKind),
            ("AKKKK", HandType::FourOfKind),
            ("QQQ22", HandType::FullHouse),
            ("QQQ23", HandType::ThreeOfKind),
            ("AAQQK", HandType::TwoPair),
            ("66234", HandType::OnePair),
            ("23456", HandType::HighCard),
            ("T55J5", HandType::ThreeOfKind),
            ("KTJJT", HandType::TwoPair),
        ];

        for (cards, expected) in patterns {
            assert_eq!(get_hand_type(cards, false), expected, "{:?}", cards)
        }
    }

    #[test]
    fn test_get_type_j() {
        #[rustfmt::skip]
        let patterns = [
            ("32T3K",  HandType::OnePair ),
            ("T55J5",  HandType::FourOfKind ),
            ("KK677",  HandType::TwoPair ),
            ("KTJJT",  HandType::FourOfKind ),
            ("QQQJA",  HandType::FourOfKind ),
            ("JJJJJ",  HandType::FiveOfKind ),
        ];

        for (cards, expected) in patterns {
            assert_eq!(get_hand_type(cards, true), expected, "{:?}", cards)
        }
    }

    #[test]
    fn test_hand_type_order() {
        let types = [
            HandType::FullHouse,
            HandType::ThreeOfKind,
            HandType::OnePair,
            HandType::HighCard,
            HandType::FiveOfKind,
            HandType::TwoPair,
            HandType::FourOfKind,
        ];

        // Order according
        assert_eq!(
            types.into_iter().sorted().collect::<Vec<_>>(),
            vec![
                HandType::HighCard,
                HandType::OnePair,
                HandType::TwoPair,
                HandType::ThreeOfKind,
                HandType::FullHouse,
                HandType::FourOfKind,
                HandType::FiveOfKind,
            ]
        );
    }

    #[test]
    fn test_hand_order() {
        assert!(Hand::<NormalRule>::new("AKQJT") > Hand::<NormalRule>::new("2KQJT"));
        assert!(Hand::<NormalRule>::new("33332") > Hand::<NormalRule>::new("2AAAA"));
        assert!(Hand::<NormalRule>::new("T55J5") > Hand::<NormalRule>::new("KTJJT"));
    }

    #[test]
    fn test_hand_order_j() {
        assert!(Hand::<JokerRule>::new("AKQJT") > Hand::<JokerRule>::new("2KQJT"));
        assert!(Hand::<JokerRule>::new("33332") > Hand::<JokerRule>::new("2AAAA"));
        assert!(Hand::<JokerRule>::new("T55J5") < Hand::<JokerRule>::new("KTJJT"));
    }
}
