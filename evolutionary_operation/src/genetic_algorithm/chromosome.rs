pub struct Chromosome<TGene> {
    genes: Vec<TGene>,
}

impl<TGene> Chromosome<TGene> {
    pub fn new(genes: Vec<TGene>) -> Self {
        return Self { genes: genes };
    }
}
