use itertools::Itertools;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    if hands.is_empty() {
        None
    } else if hands.len() == 1 {
        Some(Vec::from(hands))
    } else {
        Some(sort_hands(hands))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum PokerHand<'a> {
    Invalid(&'a str),
    HighCard(u8, &'a str),
    OnePair(u8, u8, u8, u8, &'a str),
    TwoPair(u8, u8, u8, &'a str),
    ThreeOfAKind(u8, u8, u8, &'a str),
    Straight(u8, &'a str),
    Flush(u8, &'a str),
    FullHouse(u8, u8, &'a str),
    FourOfAKind(u8, u8, &'a str),
    StraightFlush(u8, &'a str),
}

impl<'a> PokerHand<'a> {
    fn extract_str(self) -> &'a str {
        match self {
            PokerHand::Invalid(ref s) => s,
            PokerHand::StraightFlush(_, s) => s,
            PokerHand::FourOfAKind(_, _, s) => s,
            PokerHand::FullHouse(_, _, s) => s,
            PokerHand::Flush(_, s) => s,
            PokerHand::Straight(_, s) => s,
            PokerHand::ThreeOfAKind(_, _, _, s) => s,
            PokerHand::TwoPair(_, _, _, s) => s,
            PokerHand::OnePair(_, _, _, _, s) => s,
            PokerHand::HighCard(_, s) => s,
        }
    }
}

fn sort_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let pre_result = hands
        .iter()
        .map(|h| clasify(h))
        .sorted()
        .group_by(|x| x.clone())
        .into_iter()
        .map(|(x, g)| (x, g.collect()))
        .collect::<Vec<(PokerHand<'a>, Vec<_>)>>();
    println!("pre result = {:?}", pre_result);
    let result = pre_result
        .into_iter()
        .last()
        .unwrap()
        .1
        .into_iter()
        .map(|s| s.extract_str())
        .collect::<Vec<_>>();
    println!("hands: {:?}, result = {:?}", hands, result.clone());
    result
}

fn clasify(hand: &'_ str) -> PokerHand<'_> {
    println!("clasify({})", hand);
    let mut cards: Vec<Card> = hand
        .split_whitespace()
        .flat_map(|a| to_card(a))
        .sorted()
        .collect();
    cards.reverse();
    println!("cards = {:?}", cards);
    let valid_hand = cards.len() == 5;
    if !valid_hand {
        return PokerHand::Invalid(hand);
    }
    let same_suite = cards.iter().map(|c| &c.suite).sorted().dedup().count() == 1;
    let straight = is_straight(&cards);

    let max = cards
        .iter()
        .map(|c| if c.value == 14 { 1 } else { c.value })
        .max()
        .unwrap();

    if straight && same_suite {
        return PokerHand::StraightFlush(max, hand);
    }

    if straight {
        return PokerHand::Straight(max, hand);
    }

    if same_suite {
        return PokerHand::Flush(max, hand);
    }

    let groups = cards
        .iter()
        .sorted_by_key(|c| c.value)
        .group_by(|&x| x.value);

    let clasi = groups
        .into_iter()
        .map(|(x, g)| (g.count(), x))
        .sorted_by_key(|c| c.0)
        .group_by(|c| c.0)
        .into_iter()
        .map(|(x, g)| (g.count(), x))
        .collect::<Vec<(usize, usize)>>();
    let cards2 = cards
        .iter()
        .sorted_by_key(|c| c.value)
        .group_by(|&x| x.value)
        .into_iter()
        .map(|(_, g)| {
            let mut arr: Vec<_> = g.sorted_by_key(|c| c.value).collect();
            arr.reverse();
            arr
        })
        .sorted_by_key(|v| v.len())
        .collect::<Vec<Vec<&Card>>>();
    println!("cards2 = {:?}", cards2);
    match &clasi[..] {
        [_, (1, 4)] => PokerHand::FourOfAKind(
            cards2.get(1).unwrap().get(0).unwrap().value,
            cards2.get(0).unwrap().get(0).unwrap().value,
            hand,
        ),
        [(1, 2), (1, 3)] => PokerHand::FullHouse(
            cards2.get(1).unwrap().get(0).unwrap().value,
            cards2.get(0).unwrap().get(0).unwrap().value,
            hand,
        ),
        [_, (1, 3)] => PokerHand::ThreeOfAKind(
            cards2.get(2).unwrap().get(0).unwrap().value,
            cards2.get(1).unwrap().get(0).unwrap().value,
            cards2.get(0).unwrap().get(0).unwrap().value,
            hand,
        ),
        [_, (1, 2)] => PokerHand::OnePair(
            cards2.get(3).unwrap().get(0).unwrap().value,
            cards2.get(0).unwrap().get(0).unwrap().value,
            cards2.get(1).unwrap().get(0).unwrap().value,
            cards2.get(2).unwrap().get(0).unwrap().value,
            hand,
        ),
        [_, (2, 2)] => PokerHand::TwoPair(
            cards2.get(2).unwrap().get(0).unwrap().value,
            cards2.get(1).unwrap().get(0).unwrap().value,
            cards2.get(0).unwrap().get(0).unwrap().value,
            hand,
        ),
        _ => PokerHand::HighCard(cards.get(0).unwrap().value, hand),
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone)]
enum Suite {
    Spade,
    Club,
    Heart,
    Diamond,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct Card {
    value: u8,
    suite: Suite,
}

fn to_card(card: &str) -> Option<Card> {
    let value = match &card[0..card.len() - 1] {
        "A" => 14,
        "J" => 11,
        "Q" => 12,
        "K" => 13,
        s => s.parse().ok().filter(|&x| x >= 2 && x <= 10)?,
    };
    let suite = match &card[card.len() - 1..] {
        "H" => Suite::Heart,
        "D" => Suite::Diamond,
        "S" => Suite::Spade,
        "C" => Suite::Club,
        _ => return None,
    };
    Some(Card { value, suite })
}

fn is_straight(cards: &[Card]) -> bool {
    let mut previous: Option<&Card> = None;
    for c in cards.iter().rev() {
        if previous.is_some() {
            if let Some(pre) = previous {
                if !(pre.value == 14 && c.value == 2
                    || c.value - pre.value == 1
                    || c.value == 14 && pre.value == 5)
                {
                    return false;
                }
            }
        }
        previous = Some(c);
    }
    true
}
