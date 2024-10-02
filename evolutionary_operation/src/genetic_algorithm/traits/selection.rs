use crate::genetic_algorithm::chromosome::Chromosome;

pub trait Selection<TGene> {
    fn run(&self, population: &Vec<Chromosome<TGene>>) -> Vec<&Chromosome<TGene>>;
}