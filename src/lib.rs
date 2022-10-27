mod ec;
mod ff;

use ff::FiniteField;

trait ComplexMultiplication {
    // elliptic curve param a and b
    type A: FiniteField;
    type B: FiniteField;

    // construct complex multiplication
    fn new(field_order: impl FiniteField, elliptic_curve_order: impl FiniteField) -> Self;

    // get elliptic curve params a and b
    fn get(self) -> (Self::A, Self::B);
}
