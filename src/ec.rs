use super::ff::FiniteField;

// elliptic curve trait
pub(crate) trait EllipticCurve {
    // elliptic curve order
    type Order: FiniteField;

    // new elliptic curve with a and b
    fn new(a: impl FiniteField, b: impl FiniteField) -> Self;

    // return its order
    fn order(&self) -> Self::Order;
}
