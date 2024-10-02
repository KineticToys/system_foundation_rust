use crate::genetic_algorithm::chromosome::Chromosome;

use super::{
    crossover::Crossover, evaluation::Evaluation, mutation::Mutation, selection::Selection,
};

pub struct GeneticAlgorithm<
    TGene,
    TCrossover: Crossover<TGene>,
    TMutation: Mutation<TGene>,
    TEvaluation: Evaluation<TGene>,
    TSelection: Selection<TGene>,
> {
    crossover: TCrossover,
    mutation: TMutation,
    evaluation: TEvaluation,
    selection: TSelection,

    population: Vec<(Chromosome<TGene>, f64)>,
}

impl<
        TGene,
        TCrossover: Crossover<TGene>,
        TMutation: Mutation<TGene>,
        TEvaluation: Evaluation<TGene>,
        TSelection: Selection<TGene>,
    > GeneticAlgorithm<TGene, TCrossover, TMutation, TEvaluation, TSelection>
{
    pub fn new(
        crossover: TCrossover,
        mutation: TMutation,
        evaluation: TEvaluation,
        selection: TSelection,
    ) -> Self {
        return Self {
            crossover: crossover,
            mutation: mutation,
            evaluation: evaluation,
            selection: selection,

            population: Vec::new(),
        };
    }
}
