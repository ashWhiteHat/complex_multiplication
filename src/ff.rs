// finite field trait
pub(crate) trait FiniteField {
    // return its order
    fn order(&self) -> Self;

    // addition
    fn add(&self, other: Self) -> Self;

    // multiplication
    fn mul(&self, other: Self) -> Self;

    // square
    fn square(&self) -> Self;

    // substraction
    fn sub(&self, other: Self) -> Self;
}
