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
    /// Get a reference to the underlying probability map
    pub fn get_map(&self) -> &HashMap<T, f64> {
        &self.map
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

struct CountState {
    faces_drawn: Vec<usize>,
    cards_drawn: usize,
}

/// Calculate the likelihood of getting a triple or run of 3 when drawing 7 cards from a full deck.
/// 
/// This function uses DiscreteDistribution to compute the exact probability.
pub fn calculate_triple_or_run_likelihood_7_cards() -> (f64, DiscreteDistribution<String>) {
    // First, count all hands and categorize them
    let num_faces = 13;
    let max_per_face = 4;
    let cards_to_draw = 7;

    let mut state = CountState {
        faces_drawn: vec![0; num_faces],
        cards_drawn: 0,
    };

    let mut outcomes: HashMap<String, f64> = HashMap::new();
    count_hands_recursive_collect(
        0,
        cards_to_draw,
        num_faces,
        max_per_face,
        &mut state,
        &mut outcomes,
    );

    // Create distribution from collected outcomes
    let distribution = DiscreteDistribution::new_weighted(
        outcomes.into_iter().map(|(outcome, count)| (outcome, count)),
    );

    // Get favorable outcomes
    let favorable = distribution.map.get("triple_or_run").copied().unwrap_or(0.0);
    let probability = favorable * 100.0;

    (probability, distribution)
}

fn count_hands_recursive_collect(
    face_idx: usize,
    cards_remaining: usize,
    num_faces: usize,
    max_per_face: usize,
    state: &mut CountState,
    outcomes: &mut HashMap<String, f64>,
) -> u64 {
    // Base case: all cards drawn
    if cards_remaining == 0 {
        let hand_faces: Vec<u8> = state
            .faces_drawn
            .iter()
            .enumerate()
            .flat_map(|(face_idx, count)| vec![face_idx as u8 + 1; *count])
            .collect();

        let outcome = if has_triple_or_run(&hand_faces) {
            "triple_or_run".to_string()
        } else {
            "no_triple_or_run".to_string()
        };

        *outcomes.entry(outcome).or_insert(0.0) += 1.0;
        return 1;
    }

    // Base case: no more faces to consider
    if face_idx >= num_faces {
        return 0;
    }

    let mut total = 0;

    // Try drawing 0 to min(max_per_face, cards_remaining) of this face
    for count in 0..=std::cmp::min(max_per_face, cards_remaining) {
        state.faces_drawn[face_idx] = count;
        total += count_hands_recursive_collect(
            face_idx + 1,
            cards_remaining - count,
            num_faces,
            max_per_face,
            state,
            outcomes,
        );
    }

    state.faces_drawn[face_idx] = 0;
    total
}

/// Check if a hand contains a triple (3 of a kind)
fn has_triple(faces: &[u8]) -> bool {
    let mut counts = std::collections::HashMap::new();
    for &face in faces {
        *counts.entry(face).or_insert(0) += 1;
        if counts[&face] >= 3 {
            return true;
        }
    }
    false
}

/// Check if a hand contains a run of 3 (3 consecutive faces)
fn has_run_of_3(faces: &[u8]) -> bool {
    if faces.len() < 3 {
        return false;
    }

    let mut sorted = faces.to_vec();
    sorted.sort_unstable();

    for i in 0..sorted.len() - 2 {
        if sorted[i + 1] == sorted[i] + 1 && sorted[i + 2] == sorted[i] + 2 {
            return true;
        }
    }
    false
}

/// Check if a hand has either a triple or a run of 3
fn has_triple_or_run(faces: &[u8]) -> bool {
    has_triple(faces) || has_run_of_3(faces)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_triple() {
        assert!(has_triple(&[1, 1, 1, 2, 3]));
        assert!(!has_triple(&[1, 1, 2, 3, 4]));
    }

    #[test]
    fn test_has_run() {
        assert!(has_run_of_3(&[1, 2, 3, 5, 7]));
        assert!(!has_run_of_3(&[1, 3, 5, 7, 9]));
    }
}

