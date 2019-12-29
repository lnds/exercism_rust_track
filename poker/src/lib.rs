use itertools::Itertools;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let hands = hands
        .iter()
        .map(|hand| {
            let pokerhand = PokerHand::new(
                hand.split_whitespace()
                    .flat_map(|card| Card::new(card))
                    .collect::<Vec<Card>>()
                    .as_mut_slice(),
            );

            (pokerhand, *hand)
        })
        .sorted_by(|x, y| y.cmp(x))
        .collect::<Vec<(PokerHand, &'a str)>>();

    let winner = &hands[0].0;
    Some(
        hands
            .iter()
            .take_while(|hand| hand.0 == *winner) // for ties
            .map(|h| h.1)
            .collect(),
    )
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum PokerHand {
    HighCard(u8, u8, u8, u8, u8),
    OnePair(u8, u8, u8, u8),
    TwoPair(u8, u8, u8),
    ThreeOfAKind(u8, u8, u8),
    Straight(u8),
    Flush(u8),
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush(u8),
}

impl PokerHand {
    fn new(cards: &[Card]) -> Self {
        let cards: Vec<Card> = cards.iter().sorted_by(|x, y| y.cmp(x)).cloned().collect();
        let straight = cards
            .windows(2)
            .all(|w| w[0].0 == w[1].0 + 1 || w[0].0 == 14 && w[1].0 == 5);
        let flush = cards.iter().all(|c| c.1 == cards[0].1);

        let max = cards
            .iter()
            .map(|c| if c.0 == 14 { 1 } else { c.0 })
            .max()
            .unwrap();
        if straight && flush {
            return PokerHand::StraightFlush(max);
        }
        if straight {
            return PokerHand::Straight(max);
        }

        if flush {
            return PokerHand::Flush(max);
        }
        
        let grouped_cards = cards
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
        let cl = &grouped_cards.iter().map(|v| v.len()).sorted().collect::<Vec<usize>>()[..] ;
        println!("cl = {:?}", cl);
        match &grouped_cards.iter().map(|v| v.len()).collect::<Vec<usize>>()[..] {
            [_, 4] => PokerHand::FourOfAKind(
                grouped_cards[1][0].0,
                grouped_cards[0][0].0,
            ),
            [2, 3] => PokerHand::FullHouse(
                grouped_cards[1][0].0,
                grouped_cards[0][0].0,
            ),
            [_, _, 3] => PokerHand::ThreeOfAKind(
                grouped_cards[2][0].0,
                grouped_cards[1][0].0,
                grouped_cards[0][0].0,
            ),
            [_, 2, 2] => PokerHand::TwoPair(
                grouped_cards[2][0].0,
                grouped_cards[1][0].0,
                grouped_cards[0][0].0,
            ),
            [_, _, _, 2] => PokerHand::OnePair(
                grouped_cards[3][0].0,
                grouped_cards[0][0].0,
                grouped_cards[0][0].0,
                grouped_cards[0][0].0,
            ),
            _ => PokerHand::HighCard(cards[0].0, cards[1].0, cards[2].0, cards[3].0, cards[4].0),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct Card(u8, char);

impl Card {
    fn new(card: &str) -> Option<Card> {
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
