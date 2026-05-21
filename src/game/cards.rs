/// Represents the face of a card in a standard deck. Each face has an associated value used for scoring.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Face {
    ACE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
}

impl Face {
    pub fn value(&self) -> u8 {
        match self {
            Face::ACE => 1,
            Face::TWO => 2,
            Face::THREE => 3,
            Face::FOUR => 4,
            Face::FIVE => 5,
            Face::SIX => 6,
            Face::SEVEN => 7,
            Face::EIGHT => 8,
            Face::NINE => 9,
            Face::TEN | Face::JACK | Face::QUEEN | Face::KING => 10,
        }
    }
    /// Returns an iterator over all 13 faces in a standard deck.
    fn iter() -> impl Iterator<Item = Face> {
        [
            Face::ACE,
            Face::TWO,
            Face::THREE,
            Face::FOUR,
            Face::FIVE,
            Face::SIX,
            Face::SEVEN,
            Face::EIGHT,
            Face::NINE,
            Face::TEN,
            Face::JACK,
            Face::QUEEN,
            Face::KING,
        ]
        .iter()
        .copied()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Suit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES,
}

impl Suit {
    /// Returns an iterator over all four suits.
    fn iter() -> impl Iterator<Item = Suit> {
        [Suit::HEARTS, Suit::DIAMONDS, Suit::CLUBS, Suit::SPADES]
            .iter()
            .copied()
    }
}

/// Represents a card that a player has memorized.
/// These can be copied and cloned freely, and are not unique like PhysicalCards.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct VirtualCard {
    face: Face,
    suit: Suit,
}

impl VirtualCard {
    pub fn new(face: Face, suit: Suit) -> Self {
        Self { face, suit }
    }
    pub fn value(&self) -> u8 {
        self.face.value()
    }
    /// Returns an iterator over all 52 cards in a standard deck.
    pub fn iter() -> impl Iterator<Item = Self> {
        Face::iter().flat_map(|face| Suit::iter().map(move |suit| Self::new(face, suit)))
    }
}

/// Represents a physical card in the game.
/// Each card is "unique" and cannot be copied or cloned.
/// Uniqueness is not super rigourously enforced.
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct PhysicalCard {
    card: VirtualCard,
}

impl PhysicalCard {
    pub(super) fn new(face: Face, suit: Suit) -> Self {
        Self {
            card: VirtualCard::new(face, suit),
        }
    }
    pub(super) fn new_from_virtual(card: VirtualCard) -> Self {
        Self { card }
    }
    pub fn value(&self) -> u8 {
        self.card.value()
    }
    /// Returns an iterator over all 52 cards in a standard deck.
    pub(super) fn iter() -> impl Iterator<Item = Self> {
        VirtualCard::iter().map(PhysicalCard::new_from_virtual)
    }
}

impl From<&PhysicalCard> for VirtualCard {
    fn from(physical_card: &PhysicalCard) -> Self {
        physical_card.card
    }
}