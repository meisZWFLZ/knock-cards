use crate::game::VirtualCard;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PileType {
    Stock,
    Discard,
}

/// Represents the data about player's turn that is obseverable by another player.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PublicTurn {
    Knock,
    Regular {
        drew_from: PileType,
        /// Cannot be `None` if `drew_from` is `Discard`.
        swapped_card_index: Option<usize>,
        discarded_card: VirtualCard,
    },
}

/// Represents the data about player's turn that is obseverable by the player who took the turn.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PrivateTurn {
    Knock,
    Regular {
        drew_from: PileType,
        drawn_card: VirtualCard,
        /// Cannot be `None` if `drew_from` is `Discard`.
        swapped_card_index: Option<usize>,
        discarded_card: VirtualCard,
    },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Turn {
    Public(PublicTurn),
    Private(PrivateTurn),
}

impl From<PrivateTurn> for PublicTurn {
    fn from(private_turn: PrivateTurn) -> Self {
        match private_turn {
            PrivateTurn::Knock => PublicTurn::Knock,
            PrivateTurn::Regular {
                drew_from,
                swapped_card_index,
                discarded_card,
                ..
            } => PublicTurn::Regular {
                drew_from,
                swapped_card_index,
                discarded_card,
            },
        }
    }
}
