extern crate rand;

use crate::lib::structs::{Population, PopulationConfig};
pub mod lib {
    pub mod consts;
    pub mod structs;
}

fn main() {
    let target: Vec<u32> = vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1];
    let population_config = PopulationConfig {
        target: &target,
        chromosomes_length: target.len(),
        gens_length: 10,
    };
    
    let mut population = Population::new(population_config);
    population.populate();
    println!("population: {:?}\n", population);
    // TODO: result fitness in %
    let result = population.evolve();
    println!("Target: {:?}", target);
    println!("Result: {:?}", result.gens);
}
