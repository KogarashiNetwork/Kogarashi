use super::field::{Field, PrimeField};

/// `Fr` and `Curve` necessary for pairing
pub trait Engine {
    type Fr: PrimeField;

    type Fq: PrimeField;

    type Fqk: Field;

    fn pairing() -> Self::Fqk;
}
