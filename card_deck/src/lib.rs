use rand::Rng;

#[derive(Debug, PartialEq)]

pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let ransm = rand::thread_rng().gen_range(1..5);
        Suit::translate(ransm)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let ransm = rand::thread_rng().gen_range(1..14);
        Rank::translate(ransm)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            _ => Rank::King,
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    // *card::suit::Ace == Card::Suit::Ace
    card.suit == crate::Suit::Spade && card.rank == crate::Rank::Ace
}
