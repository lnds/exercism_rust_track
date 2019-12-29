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
    HighCard(Vec<u8>, &'a str),
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
            PokerHand::Invalid(s) => s,
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
    println!("hands = {:?}", hands);
    let pre_result = hands
        .iter()
        .map(|h| clasify(h))
        .sorted()
        .collect::<Vec<PokerHand>>();
    if pre_result.iter().all(|x| match x {
        PokerHand::HighCard(_, _) => true,
        _ => false,
    }) {
        println!("tie!");
        let max = pre_result
            .iter()
            .map(|c| match c {
                PokerHand::HighCard(v, _) => v.clone(),
                _ => vec![],
            })
            .max()
            .unwrap();
        println!("max = {:?}", max);
        pre_result
            .iter()
            .filter(|&c| {
                let result = match c {
                    PokerHand::HighCard(v, _) => *v == max,
                    _ => false,
                };
                println!("filtrando c = {:?} => {}", *c, result);
                result
            })
            .map(|c| c.clone().extract_str())
            .collect()
    } else {
        vec![hands
            .iter()
            .map(|h| clasify(h))
            .sorted()
            .map(|s| s.extract_str())
            .last()
            .unwrap()]
    }
}

fn clasify(hand: &'_ str) -> PokerHand<'_> {
    let mut cards: Vec<Card> = hand
        .split_whitespace()
        .flat_map(|a| Card::new(a))
        .sorted()
        .collect();
    cards.reverse();
    if cards.len() != 5 {
        return PokerHand::Invalid(hand);
    }
    let same_suite = cards.iter().map(|c| &c.1).sorted().dedup().count() == 1;
    let straight = is_straight(&cards);

    let max = cards
        .iter()
        .map(|c| if c.0 == 14 { 1 } else { c.0 })
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

    let clasi = cards
        .iter()
        .sorted_by_key(|c| c.0)
        .group_by(|&c| c.0)
        .into_iter()
        .map(|(x, g)| (g.count(), x))
        .sorted_by_key(|c| c.0)
        .group_by(|c| c.0)
        .into_iter()
        .map(|(x, g)| (g.count(), x))
        .collect::<Vec<(usize, usize)>>();
    let cards2 = cards
        .iter()
        .sorted_by_key(|c| c.0)
        .group_by(|&c| c.0)
        .into_iter()
        .map(|(_, g)| {
            let mut arr: Vec<_> = g.sorted_by_key(|c| c.0).collect();
            arr.reverse();
            arr
        })
        .sorted_by_key(|v| v.len())
        .collect::<Vec<Vec<&Card>>>();
    match &clasi[..] {
        [_, (1, 4)] => PokerHand::FourOfAKind(
            cards2.get(1).unwrap().get(0).unwrap().0,
            cards2.get(0).unwrap().get(0).unwrap().0,
            hand,
        ),
        [(1, 2), (1, 3)] => PokerHand::FullHouse(
            cards2.get(1).unwrap().get(0).unwrap().0,
            cards2.get(0).unwrap().get(0).unwrap().0,
            hand,
        ),
        [_, (1, 3)] => PokerHand::ThreeOfAKind(
            cards2.get(2).unwrap().get(0).unwrap().0,
            cards2.get(1).unwrap().get(0).unwrap().0,
            cards2.get(0).unwrap().get(0).unwrap().0,
            hand,
        ),
        [_, (1, 2)] => PokerHand::OnePair(
            cards2.get(3).unwrap().get(0).unwrap().0,
            cards2.get(0).unwrap().get(0).unwrap().0,
            cards2.get(1).unwrap().get(0).unwrap().0,
            cards2.get(2).unwrap().get(0).unwrap().0,
            hand,
        ),
        [_, (2, 2)] => PokerHand::TwoPair(
            cards2.get(2).unwrap().get(0).unwrap().0,
            cards2.get(1).unwrap().get(0).unwrap().0,
            cards2.get(0).unwrap().get(0).unwrap().0,
            hand,
        ),
        _ => PokerHand::HighCard(cards.iter().map(|c| c.0   ).collect(), hand),
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct Card(u8, char);

impl Card {
    fn new (card: &str) -> Option<Card> {
        let value = match &card[0..card.len() - 1] {
            "A" => 14,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            s => s.parse().ok().filter(|&x| x >= 2 && x <= 10)?,
        };
        let suite = card.chars().last().unwrap();
        Some(Card(value, suite))
    }
}

fn is_straight(cards: &[Card]) -> bool {
    let mut previous: Option<&Card> = None;
    for c in cards.iter().rev() {
        if previous.is_some() {
            if let Some(pre) = previous {
                if !(pre.0 == 14 && c.0 == 2
                    || c.0 - pre.0 == 1
                    || c.0 == 14 && pre.0 == 5)
                {
                    return false;
                }
            }
        }
        previous = Some(c);
    }
    true
}
