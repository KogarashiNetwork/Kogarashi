use super::{
    algebra::Field,
    comp::{Basic, ParityCmp},
    curve::Affine,
    field::PrimeField,
    Projective,
};

/// extension field
pub trait ExtensionField: Field + Basic + ParityCmp {
    fn mul_by_nonresidue(self) -> Self;
}

/// pairing field
pub trait PairingField: PrimeField + ParityCmp {}

/// pairing function range field
pub trait PairingRange: ExtensionField {
    type G1Affine: Affine;
    type G2Coeff: ParityCmp;
    type QuadraticField: ExtensionField;

    fn mul_by_014(
        self,
        c0: Self::QuadraticField,
        c1: Self::QuadraticField,
        c4: Self::QuadraticField,
    ) -> Self;

    fn untwist(self, coeffs: Self::G2Coeff, g1: Self::G1Affine) -> Self;

    fn final_exp(self) -> Option<Self>;
}

/// G2 group pairing interface
pub trait G2Pairing: Projective {
    type PairingRange: PairingRange;
    type PairingCoeff: ParityCmp;
    type PairingRepr: ParityCmp;
    type G2Affine: Affine;

    fn double_eval(&mut self) -> Self::PairingCoeff;

    fn add_eval(&mut self, rhs: Self::G2Affine) -> Self::PairingCoeff;
}

/// pairing abstraction
pub trait Pairing {
    // g1 group affine point
    type G1Affine: Affine;
    // g2 group affine point
    type G2Affine: Affine;
    // g1 group projective point
    type G1Projective: Projective;
    // g2 group projective point
    type G2Projective: G2Pairing;
    type G2PairngRepr: G2Pairing;
    // range of pairing function
    type PairingRange: PairingRange;

    const X: u64;
    const X_IS_NEGATIVE: bool;

    fn pairing(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::PairingRange;

    fn miller_loop(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::PairingRange;
}
