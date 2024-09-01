pub trait Vector {
    /// Returns the number of dimensions of the vector.
    fn dimensions(&self) -> usize;

    /// Returns the magnitude of the vector.
    fn magnitude(&self) -> f64;
}