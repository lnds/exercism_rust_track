use itertools::Itertools;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    if hands.is_empty() {
        None
    } else if hands.len() == 1 {
        Some(Vec::from(hands))
    } else {
        let sorted_hands = sort_hands(hands);
        println!("hands        = {:?}", hands);
        println!("sorted_hands = {:?}", sorted_hands);
        Some(sorted_hands)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum PokerHand<'a> {
    Invalid(&'a str),
    HighCard(u8, &'a str),
    OnePair(u8, u8, &'a str),
    TwoPair(u8, u8, u8, &'a str),
    ThreeOfAKind(Vec<Card>, &'a str),
    Straight(Vec<Card>, &'a str),
    Flush(Vec<Card>, &'a str),
    FullHouse(Vec<Card>, &'a str),
    FourOfAKind(Vec<Card>, &'a str),
    StraightFlush(Vec<Card>, &'a str),
}

impl<'a> PokerHand<'a> {
    fn extract_str(self) -> &'a str {
        match self {
            PokerHand::Invalid(ref s) => s,
            PokerHand::StraightFlush(_, s) => s,
            PokerHand::FourOfAKind(_, s) => s,
            PokerHand::FullHouse(_, s) => s,
            PokerHand::Flush(_, s) => s,
            PokerHand::Straight(_, s) => s,
            PokerHand::ThreeOfAKind(_, s) => s,
            PokerHand::TwoPair(_,_,_, s) => s,
            PokerHand::OnePair(_, _, s) => s,
            PokerHand::HighCard(_, s) => s,
        }
    }
}

fn sort_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let pre_result  = hands
        .iter()
        .map(|h| clasify(h))
        .sorted()
        .group_by(|x| x.clone())
        .into_iter()
        .map(|(x,g)| (x, g.collect()))
        .collect::<Vec<(PokerHand<'a>, Vec<_>)>>();
    println!("pre result = {:?}", pre_result);
    let result = pre_result.into_iter()
        .last().unwrap().1.into_iter().map(|s| s.extract_str()).collect::<Vec<_>>();
    println!("hands: {:?}, result = {:?}", hands, result.clone());
    result
}

fn clasify<'a>(hand: &'a str) -> PokerHand<'a> {
    println!("clasify({})", hand);
    let mut cards: Vec<Card> = hand.split_whitespace().flat_map(|a| to_card(a)).sorted().collect();
    cards.reverse();
    println!("cards = {:?}", cards);    
    let valid_hand = cards.len() == 5;
    if !valid_hand {
        println!("->Is invalid");
        return PokerHand::Invalid(hand);
    }
    let suites: Vec<Suite> = cards.iter().map(|c| &c.suite).sorted().cloned().collect();
    println!("suites = {:?}", suites);
    let same_suite = cards.iter().map(|c| &c.suite).sorted().dedup().count() == 1;
    let min_value : u8 = cards.iter().map(|c| c.value % 13).sorted().min().unwrap();
    let straight = cards.iter().map(|c| (c.value % 13) - min_value).sum::<u8>() == 10;
    if straight && same_suite {
        println!("-> is StraightFlush");
        return PokerHand::StraightFlush(cards, hand);
    } 

    if straight {
        println!("-> is straight");
        return PokerHand::Straight(cards, hand);
    }

    if same_suite {
        println!("-> is flush");
        return PokerHand::Flush(cards, hand);
    }

    let groups = cards
    .iter()
    .sorted_by_key(|c| c.value)
    .group_by(|&x| x.value);

    let clasi =groups
        .into_iter()
        .map(|(x, g)| (g.count(), x))
        .sorted_by_key(|c| c.0)
        .group_by(|c| c.0)
        .into_iter()
        .map(|(x, g)| (g.count(), x))
        .collect::<Vec<(usize, usize)>>();
        println!("hand = {}, cards  = {:?}", hand, cards);
    match &clasi[..] {
        [_, (1,4)] => {
            println!("-> Is Four of a Kind");
            PokerHand::FourOfAKind(cards, hand)
        }
        [(1, 2),(1,3)] =>{
            println!("-> Is Full House");
            PokerHand::FullHouse(cards, hand)
        }
        [_, (1, 3)] => {
            println!("-> Three of a kind");
            PokerHand::ThreeOfAKind(cards, hand)
        }
        [_, (1, 2)] => {
            let result = PokerHand::OnePair(
                cards.get(0).unwrap().value, cards.get(4).unwrap().value, hand);
            println!("-> Is a Pair {:?}", result);
            result
        }
        [_, (2, 2)] => {
            let result = PokerHand::TwoPair(
                cards.get(0).unwrap().value, cards.get(2).unwrap().value,
                cards.get(4).unwrap().value, hand);
            println!("-> Is Two Pair {:?}", result);
            result
        },
        _ => {
            println!("-> Is High Card");
            PokerHand::HighCard(cards.get(0).unwrap().value, hand)
        }
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
