use crate::genetic_algorithm::chromosome::Chromosome;

pub trait Crossover<TGene> {
    /// Run crossover, yielding (possibly) multiple offsprings.
    fn run(
        &self,
        chromosome1: &Chromosome<TGene>,
        chromosome2: &Chromosome<TGene>,
    ) -> Vec<Chromosome<TGene>>;
}
