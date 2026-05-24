#![allow(dead_code)]

mod game;
mod brain;

use brain::calculate_triple_or_run_likelihood_7_cards;

fn main() {
    println!("Calculating likelihood of triple or run of 3 with 7 cards from a full deck...\n");
    
    let (probability, distribution) = calculate_triple_or_run_likelihood_7_cards();
    
    println!("Results using DiscreteDistribution:");
    println!("====================================");
    println!("Probability of having a triple or run of 3: {:.4}%", probability);
    println!("\nDetailed Distribution:");
    for (outcome, prob) in distribution.get_map().iter() {
        println!("  {}: {:.6} ({:.4}%)", outcome, prob, prob * 100.0);
    }
}
