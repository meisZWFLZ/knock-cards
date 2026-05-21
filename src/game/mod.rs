mod cards;
mod pile;
mod player;
mod turn;

pub use cards::{Face, PhysicalCard, Suit, VirtualCard};
pub use pile::{DiscardPile, Pile, StockPile};
pub use player::{Brain, Player};
pub use turn::{PileType, PublicTurn};

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

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    piles: GamePiles,
    config: GameConfig,
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
        let players = grids
            .into_iter()
            .zip(brains.into_iter())
            .map(|(grid, brain)| Player::new(grid, brain, config))
            .collect();
        Self {
            players,
            piles,
            config,
        }
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
        self.players
            .iter()
            .map(|player| player.score())
            .collect()
    }
}
