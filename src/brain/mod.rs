use itertools::Itertools;

use crate::game::{
    Brain, Game, GameConfig, GamePiles, HistoryView, PhysicalCard, Turn, VirtualCard,
};

use core::hash;
use std::{
    collections::{HashMap, HashSet},
    hash::{DefaultHasher, Hash},
};

pub struct DiscreteDistribution<T: Eq + Hash> {
    /// The items in the distribution, along with their probabilities.
    /// The probabilities should sum to 1.
    /// Each item is enforced to be unique.
    map: HashMap<T, f64>,
}
impl<T: Eq + Hash> DiscreteDistribution<T> {
    pub fn new_weighted(items: impl IntoIterator<Item = (T, f64)>) -> Self {
        let mut total_weight = 0f64;
        let mut map =
            items
                .into_iter()
                .fold(HashMap::<T, f64>::new(), |mut acc, (item, weight)| {
                    *acc.entry(item).or_insert(0.) += weight;
                    total_weight += weight;
                    acc
                });
        map.values_mut().for_each(|weight| *weight /= total_weight);
        Self { map }
    }
    pub fn new_uniform(items: Vec<T>) -> Self {
        Self::new_weighted(items.into_iter().map(|item| (item, 1.0)))
    }
    pub unsafe fn new_unchecked(map: HashMap<T, f64>) -> Self {
        Self { map }
    }
    pub fn map<U: Eq + Hash, F: Fn(T) -> U>(self, f: F) -> DiscreteDistribution<U> {
        DiscreteDistribution::new_weighted(
            self.map.into_iter().map(|(item, weight)| (f(item), weight)),
        )
    }
    /// Mapper function must be injective (one to one)
    pub unsafe fn map_unchecked<U: Eq + Hash, F: Fn(T) -> U>(
        self,
        f: F,
    ) -> DiscreteDistribution<U> {
        let map = self
            .map
            .into_iter()
            .map(|(item, weight)| (f(item), weight))
            .collect();
        unsafe { DiscreteDistribution::new_unchecked(map) }
    }
}
impl<T: Eq + Hash + Clone> DiscreteDistribution<T> {
    pub fn cartesian_product<U: Eq + Hash + Clone>(
        &self,
        other: &DiscreteDistribution<U>,
    ) -> DiscreteDistribution<(T, U)> {
        DiscreteDistribution::new_weighted(self.map.iter().cartesian_product(other.map.iter()).map(
            |((event1, weight1), (event2, weight2))| {
                ((event1.clone(), event2.clone()), weight1 * weight2)
            },
        ))
    }
    /// (event1, event2) and (event2, event1) are treated as the same outcome, and their probabilities are added together.
    pub fn unordered_cartesian_product(
        &self,
        other: &DiscreteDistribution<T>,
    ) -> DiscreteDistribution<(T, T)> {
        self.cartesian_product(other).map(|(a, b)| {
            let mut slice = [a, b];
            slice.sort_by_key(|event| event.hash(&mut DefaultHasher::new()));
            (slice[0].to_owned(), slice[1].to_owned())
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct GameState {
    /// Cards that have yet to been seen by the player.
    /// These could be in the stock pile, other players' grids, .
    unknowns: Vec<VirtualCard>,
}
