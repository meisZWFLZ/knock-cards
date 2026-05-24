use std::{fmt::Debug, mem::swap};

use crate::game::{
    GameConfig, GamePiles, HistoryView, PhysicalCard, PileType, VirtualCard, turn::{PrivateTurn, Turn}
};

#[derive(Debug)]
pub struct Player {
    /// Doesn't necessarily have to be a grid, but it differentiates from terms like "hand"
    grid: Vec<PhysicalCard>,
    brain: Box<dyn Brain>,
}

impl Player {
    pub fn new(grid: Vec<PhysicalCard>, brain: Box<dyn Brain>) -> Self {
        let initial_revealed_cards = grid
            .iter()
            .take(game_config.revealed_num)
            .map(|card| card.into())
            .collect();
        Self {
            grid,
            brain,
        }
    }
    pub fn score(&self) -> u32 {
        self.grid.iter().map(|card| card.value() as u32).sum()
    }
    pub fn take_turn(&mut self, piles: &mut GamePiles) -> PrivateTurn {
        *self
            .brain
            .take_turn(
                &self.game_info,
                DrawPlayerInterface::new(piles, &mut self.grid),
            )
            .get_turn()
    }
}

/// Wrapper around PrivateTurn that can only be created by using the PlayerInterface structs.
#[derive(Eq, PartialEq, Hash)]
pub struct PlayerAction {
    turn: PrivateTurn,
}

impl PlayerAction {
    pub(self) fn new(turn: PrivateTurn) -> Self {
        Self { turn }
    }
    pub fn get_turn(&self) -> &PrivateTurn {
        &self.turn
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct DrawPlayerInterface<'a> {
    game_piles: &'a mut GamePiles,
    player_grid: &'a mut Vec<PhysicalCard>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct SwapPlayerInterface<'a, const DREW_FROM_DISCARD: bool> {
    game_piles: &'a mut GamePiles,
    player_grid: &'a mut Vec<PhysicalCard>,
    held_card: PhysicalCard,
}

impl<'a, const DREW_FROM_DISCARD: bool> SwapPlayerInterface<'a, DREW_FROM_DISCARD> {
    fn new(
        held_card: PhysicalCard,
        game_piles: &'a mut GamePiles,
        player_grid: &'a mut Vec<PhysicalCard>,
    ) -> Self {
        Self {
            game_piles,
            player_grid,
            held_card,
        }
    }
    pub fn pile_type(&self) -> PileType {
        if DREW_FROM_DISCARD {
            PileType::Discard
        } else {
            PileType::Stock
        }
    }
    pub fn swap_card(mut self, card_index: usize) -> PlayerAction {
        swap(&mut self.held_card, &mut self.player_grid[card_index]);
        let turn = PrivateTurn::Regular {
            drew_from: self.pile_type(),
            drawn_card: (&self.held_card).into(),
            swapped_card_index: Some(card_index),
            discarded_card: (&self.held_card).into(),
        };
        self.game_piles.discard.add_card_to_top(self.held_card);
        PlayerAction::new(turn)
    }
}

impl<'a> SwapPlayerInterface<'a, false> {
    pub fn skip(self) -> PlayerAction {
        let turn = PrivateTurn::Regular {
            drew_from: self.pile_type(),
            drawn_card: (&self.held_card).into(),
            swapped_card_index: None,
            discarded_card: (&self.held_card).into(),
        };
        self.game_piles.discard.add_card_to_top(self.held_card);
        PlayerAction::new(turn)
    }
}

impl<'a> DrawPlayerInterface<'a> {
    fn new(game_piles: &'a mut GamePiles, player_grid: &'a mut Vec<PhysicalCard>) -> Self {
        Self {
            game_piles,
            player_grid,
        }
    }
    pub fn peek_discard_pile(&self) -> Option<VirtualCard> {
        self.game_piles.discard.peek_top_card()
    }
    pub fn draw_from_stock_pile(self) -> SwapPlayerInterface<'a, false> {
        SwapPlayerInterface::new(
            self.game_piles
                .stock
                .remove_top_card()
                .expect("Stock pile should not be empty"),
            self.game_piles,
            self.player_grid,
        )
    }
    pub fn draw_from_discard_pile(self) -> SwapPlayerInterface<'a, true> {
        SwapPlayerInterface::new(
            self.game_piles
                .stock
                .remove_top_card()
                .expect("Discard pile should not be empty"),
            self.game_piles,
            self.player_grid,
        )
    }
    pub fn knock() -> PlayerAction {
        PlayerAction::new(PrivateTurn::Knock)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GameInfo<'a> {
    pub history: HistoryView<'a>,
    pub game_config: GameConfig,
    pub initial_revealed_cards: Vec<VirtualCard>,
}

pub trait Brain: Debug {
    fn take_turn(&mut self, state: &GameInfo, interface: DrawPlayerInterface) -> PlayerAction;
}
