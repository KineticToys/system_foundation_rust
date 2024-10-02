use crate::genetic_algorithm::chromosome::Chromosome;

pub trait Evaluation<TGene> {
    fn run(&self, chromosome: &Chromosome<TGene>) -> f64;
}