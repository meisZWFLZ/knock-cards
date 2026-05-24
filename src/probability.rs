use std::collections::HashMap;

/// A discrete probability distribution represented as a map of outcomes to their probabilities.
/// Probabilities are represented as fractions (numerator, denominator) to maintain precision.
#[derive(Debug, Clone)]
pub struct DiscreteDistribution {
    /// Map of outcomes to (count, total) representing the probability
    outcomes: HashMap<String, (u64, u64)>,
    total_outcomes: u64,
}

impl DiscreteDistribution {
    /// Create a new empty distribution
    pub fn new() -> Self {
        Self {
            outcomes: HashMap::new(),
            total_outcomes: 0,
        }
    }

    /// Add an outcome with its count
    pub fn add_outcome(&mut self, outcome: String, count: u64) {
        self.total_outcomes += count;
        self.outcomes
            .entry(outcome)
            .and_modify(|(c, _)| *c += count)
            .or_insert((count, 0));
    }

    /// Finalize the distribution by setting denominators
    pub fn finalize(&mut self) {
        for (_, (_, denom)) in self.outcomes.iter_mut() {
            *denom = self.total_outcomes;
        }
    }

    /// Get the probability of an outcome as (numerator, denominator)
    pub fn get_probability(&self, outcome: &str) -> Option<(u64, u64)> {
        self.outcomes.get(outcome).copied()
    }

    /// Get the probability of an outcome as a float
    pub fn get_probability_float(&self, outcome: &str) -> Option<f64> {
        self.outcomes
            .get(outcome)
            .map(|(num, denom)| *num as f64 / *denom as f64)
    }

    /// Print the distribution
    pub fn print_distribution(&self) {
        println!("\nDiscrete Distribution:");
        println!("Total outcomes: {}", self.total_outcomes);
        for (outcome, (num, denom)) in self.outcomes.iter() {
            let simplified = simplify_fraction(*num, *denom);
            let percentage = (*num as f64 / *denom as f64) * 100.0;
            println!(
                "  {}: {}/{} ({:.4}%)",
                outcome, simplified.0, simplified.1, percentage
            );
        }
    }
}

impl Default for DiscreteDistribution {
    fn default() -> Self {
        Self::new()
    }
}

/// Simplify a fraction to lowest terms
fn simplify_fraction(num: u64, denom: u64) -> (u64, u64) {
    let gcd = gcd(num, denom);
    (num / gcd, denom / gcd)
}

/// Calculate greatest common divisor
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Represents a hand of cards with their faces
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hand {
    faces: Vec<u8>, // Face values 1-13
}

impl Hand {
    /// Create a hand from face values
    pub fn new(faces: Vec<u8>) -> Self {
        let mut sorted = faces;
        sorted.sort_unstable();
        Self { faces: sorted }
    }

    /// Check if the hand contains a triple (3 of a kind)
    pub fn has_triple(&self) -> bool {
        let mut counts = HashMap::new();
        for &face in &self.faces {
            *counts.entry(face).or_insert(0) += 1;
            if counts[&face] >= 3 {
                return true;
            }
        }
        false
    }

    /// Check if the hand contains a run of 3 (3 consecutive faces)
    pub fn has_run_of_3(&self) -> bool {
        if self.faces.len() < 3 {
            return false;
        }

        for i in 0..self.faces.len() - 2 {
            // Check for consecutive sequence
            if self.faces[i + 1] == self.faces[i] + 1 && self.faces[i + 2] == self.faces[i] + 2 {
                return true;
            }
        }
        false
    }

    /// Check if the hand has either a triple or a run of 3
    pub fn has_triple_or_run(&self) -> bool {
        self.has_triple() || self.has_run_of_3()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triple_detection() {
        let hand = Hand::new(vec![1, 1, 1, 2, 3]);
        assert!(hand.has_triple());
    }

    #[test]
    fn test_run_detection() {
        let hand = Hand::new(vec![1, 2, 3, 5, 7]);
        assert!(hand.has_run_of_3());
    }

    #[test]
    fn test_no_triple_or_run() {
        let hand = Hand::new(vec![1, 3, 5, 7, 9]);
        assert!(!hand.has_triple_or_run());
    }
}
