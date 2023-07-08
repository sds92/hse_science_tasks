use crate::lib::consts::MUTATION_RATE;
use rand::Rng;

use super::consts::MAX_GENERATIONS;

#[derive(Clone, Debug)]
pub struct Chromosome {
    pub gens: Vec<u32>,
    pub fitness: u32,
}

impl Chromosome {
    pub fn new(gens: Vec<u32>) -> Chromosome {
        Chromosome { gens, fitness: 0 }
    }

    pub fn calculate_fitness(&mut self, target: &[u32]) {
        // ChatGPT output
        // TODO: check if it is the best way
        self.fitness = self
            .gens
            .iter()
            .zip(target.iter())
            .filter(|(&gene, &target_gene)| gene == target_gene)
            .count() as u32;
    }

    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        for gene in &mut self.gens {
            // TODO: put mutation rate in Population config
            if rng.gen::<f64>() < MUTATION_RATE {
                *gene = rng.gen_range(0..=1);
            }
        }
    }

    pub fn select_parents(&mut self) {}
}

#[derive(Clone, Debug)]
pub struct PopulationConfig<'a> {
    pub target: &'a Vec<u32>,
    pub gens_length: usize,
    pub chromosomes_length: usize,
}

#[derive(Clone, Debug)]
pub struct Population<'a> {
    pub config: PopulationConfig<'a>,
    pub chromosomes: Vec<Chromosome>,
}

impl Population<'_> {
    pub fn new(config: PopulationConfig) -> Population {
        Population {
            chromosomes: [].to_vec(),
            config,
        }
    }

    pub fn populate(&mut self) {
        // generate random arrays (chromosomes)
        // push them to instance
        let mut rng = rand::thread_rng();
        for _ in 0..self.config.chromosomes_length {
            let gens: Vec<u32> = (0..self.config.target.len())
                .map(|_| rng.gen_range(0..=1))
                .collect();
            let mut chromosome = Chromosome::new(gens);
            chromosome.calculate_fitness(&self.config.target);
            self.chromosomes.push(chromosome)
        }
    }

    fn select_parents(&mut self) -> (Chromosome, Chromosome) {
        // randomly select parents
        // TODO: check if parent1 === parent2
        // TODO: select parents by fitness
        // TODO: put selection method in config
        let mut rng = rand::thread_rng();
        let parent1 = &self.chromosomes[rng.gen_range(0..self.chromosomes.len())];
        let parent2 = &self.chromosomes[rng.gen_range(0..self.chromosomes.len())];

        (parent1.clone(), parent2.clone())
    }

    fn crossover(&mut self, parent1: &Chromosome, parent2: &Chromosome) -> Chromosome {
        // mixes parents by crossover point
        // TODO: check methods: averaging, blending ???
        let mut rng = rand::thread_rng();
        // TODO: maybe not random?
        let crossover_point = rng.gen_range(1..=self.config.gens_length);

        let mut child_chromosome = Vec::new();
        child_chromosome.extend_from_slice(&parent1.gens[0..crossover_point]);
        child_chromosome.extend_from_slice(&parent2.gens[crossover_point..]);

        // TODO: check if child already exists in Population / take into account perfomance issues
        let mut child = Chromosome::new(child_chromosome);
        // TODO: do we need to mutate every child?
        child.mutate();

        child
    }

    pub fn evolve(&mut self) -> Chromosome {
        // replace the weakest chromosome with newly generated child
        // TODO: put max_generations into config
        // TODO: check if we can somehow calculate max_generations
        for _ in 0..MAX_GENERATIONS {
            // TODO: stop it when required fittness is achieved
            let (parent1, parent2) = self.select_parents();
            let mut child = self.crossover(&parent1, &parent2);
            child.calculate_fitness(self.config.target);

            // ChatGPT output
            // seems to be not bad
            let weakest_index = self
                .chromosomes
                .iter()
                .enumerate()
                .min_by_key(|&(_, individual)| individual.fitness)
                .map(|(index, _)| index)
                .unwrap();

            self.chromosomes[weakest_index] = child;
        }

        // return the chromosome with max fittness
        return self.chromosomes
            .iter()
            .max_by_key(|&individual| individual.fitness)
            .unwrap()
            .clone();
    }
}
