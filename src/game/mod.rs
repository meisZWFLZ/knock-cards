mod cards;
mod pile;
mod player;
mod turn;

pub use cards::{Face, PhysicalCard, Suit, VirtualCard};
pub use pile::{DiscardPile, Pile, StockPile};
pub use player::{Brain, Player};
pub use turn::{PileType, PrivateTurn, PublicTurn, Turn};

use rand::SeedableRng as _;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct GameConfig {
    pub num_players: usize,
    pub grid_size: usize,
    /// How many cards each player gets to see (from their own cards) at the start of the game. Must be less than hand_size.
    pub revealed_num: usize,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct GamePiles {
    pub stock: StockPile,
    pub discard: DiscardPile,
}

impl GamePiles {
    pub fn new_shuffled(seed: u64) -> Self {
        let mut stock = StockPile::new_full_deck();
        stock.shuffle(rand::rngs::StdRng::seed_from_u64(seed));
        Self {
            stock,
            discard: DiscardPile::new_empty(),
        }
    }
}

/// A Lap is a round of turns where each player takes one turn.
/// This differentiates it from a "round" which might be a full game from shuffle to scoring.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Lap {
    /// Should have the same length as the number of players in the game, unless this is the current lap.
    pub turns: Vec<PrivateTurn>,
}
impl Lap {
    pub fn new_empty() -> Self {
        Self { turns: Vec::new() }
    }
    pub fn view_as_player<'a>(&'a self, player_index: usize) -> LapView<'a> {
        LapView::new(self, player_index)
    }
}

/// A LapView is a view of a lap from the perspective of a single player, where they can only see private info
/// about their own turn, and public info about other players' turns.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LapView<'a> {
    lap: &'a Lap,
    player_index: usize,
}
impl<'a> LapView<'a> {
    pub fn new(lap: &'a Lap, player_index: usize) -> LapView {
        LapView { lap, player_index }
    }
    pub fn iter(&'a self) -> impl Iterator<Item = Turn> + 'a {
        self.lap.turns.iter().enumerate().map(|(i, turn)| {
            if i == self.player_index {
                Turn::Private(turn.clone())
            } else {
                Turn::Public(turn.to_owned().into())
            }
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct History {
    pub rounds: Vec<Lap>,
}
impl History {
    pub fn new_empty() -> Self {
        Self { rounds: Vec::new() }
    }
    pub fn view_as_player<'a>(&'a self, player_index: usize) -> HistoryView<'a> {
        HistoryView::new(self, player_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct HistoryView<'a> {
    history: &'a History,
    player_index: usize,
}
impl<'a> HistoryView<'a> {
    pub fn new(history: &'a History, player_index: usize) -> HistoryView {
        HistoryView {
            history,
            player_index,
        }
    }
    pub fn iter(&'a self) -> impl Iterator<Item = LapView<'a>> + 'a {
        self.history
            .rounds
            .iter()
            .map(move |lap| LapView::new(lap, self.player_index))
    }
}

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    piles: GamePiles,
    config: GameConfig,
    history: History,
}

impl Game {
    fn deal_new(config: GameConfig, brains: Vec<Box<dyn Brain>>, seed: u64) -> Self {
        let mut piles = GamePiles::new_shuffled(seed);
        let mut grids: Vec<Vec<PhysicalCard>> =
            (0..config.num_players).map(|_| Vec::new()).collect();
        for _ in 0..config.grid_size {
            for grid in &mut grids {
                grid.push(
                    piles
                        .stock
                        .remove_top_card()
                        .expect("Not enough cards in the deck to deal"),
                );
            }
        }
        let mut result = Game {
            players: Vec::new(),
            piles,
            config,
            history: History::new_empty(),
        };
        result.players = grids
            .into_iter()
            .zip(brains.into_iter())
            .enumerate()
            .map(|(index, (grid, brain))| Player::new(grid, brain))
            .collect();
        result
    }
    fn play(mut self) -> Vec<u32> {
        'outer: loop {
            for player in &mut self.players {
                if todo!("Player knocked a round before") {
                    break 'outer;
                }
                let turn = player.take_turn(&mut self.piles);
                todo!("Update history");
            }
        }
        self.players.iter().map(|player| player.score()).collect()
    }
}
