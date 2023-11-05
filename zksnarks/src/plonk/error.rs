#[derive(Debug)]
pub enum PlonkError {
    CoefficientsDegreeIsZero,
    CoefficientsDegreeTooLarge,
}
