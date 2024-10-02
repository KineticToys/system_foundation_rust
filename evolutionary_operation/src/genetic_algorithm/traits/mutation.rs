use crate::genetic_algorithm::chromosome::Chromosome;

pub trait Mutation<TGene> {
    fn run(&self, chromosome: &Chromosome<TGene>) -> Vec<Chromosome<TGene>>;
}