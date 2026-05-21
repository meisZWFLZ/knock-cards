use crate::game::{Face, PhysicalCard, VirtualCard};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Pile<const FACE_UP: bool> {
    cards: Vec<PhysicalCard>,
}

pub type StockPile = Pile<false>;
pub type DiscardPile = Pile<true>;

impl<const F: bool> Pile<F> {
    pub fn new_empty() -> Self {
        Self { cards: Vec::new() }
    }
    pub(super) fn new_full_deck() -> Self {
        Self {
            cards: PhysicalCard::iter().collect(),
        }
    }
    pub fn shuffle(&mut self, mut rng: impl rand::Rng) {
        use rand::seq::SliceRandom;

        self.cards.shuffle(&mut rng);
    }
    pub fn remove_top_card(&mut self) -> Option<PhysicalCard> {
        self.cards.pop()
    }
    pub fn add_card_to_top(&mut self, card: PhysicalCard) {
        self.cards.push(card);
    }
}

impl DiscardPile {
    pub fn peek_top_card(&self) -> Option<VirtualCard> {
        self.cards.last().map(|card| card.into())
    }
}
