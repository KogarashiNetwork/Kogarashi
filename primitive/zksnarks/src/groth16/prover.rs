use super::constraint::Constraint;
use super::wire::Wire;
use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::error::Error;
use crate::groth16::Groth16;
use hashbrown::HashMap;
use poly_commit::{Coefficients, Fft, PointsValue};
use zkstd::common::{vec, CurveGroup, Group, Pairing, Vec};

#[derive(Debug)]
pub struct Prover<P: Pairing> {
    /// The set of rank-1 constraints which define the R1CS instance.
    pub constraints: Vec<Constraint<<P::JubjubAffine as CurveGroup>::Range>>,
    pub(crate) instance: HashMap<Wire, <P::JubjubAffine as CurveGroup>::Range>,
    pub(crate) witness: HashMap<Wire, <P::JubjubAffine as CurveGroup>::Range>,
}

impl<P: Pairing> Prover<P> {
    /// Execute the gadget, and return whether all constraints were satisfied.
    pub fn create_proof<C>(&mut self, circuit: C) -> Result<bool, Error>
    where
        C: Circuit<P::JubjubAffine, ConstraintSystem = Groth16<P::JubjubAffine>>,
    {
        let mut prover = Groth16::<P::JubjubAffine>::initialize();
        circuit.synthesize(&mut prover)?;

        let k = prover.m().trailing_zeros();
        let fft = Fft::<P::ScalarField>::new(k as usize);

        let mut a_vals = vec![
            vec![<P::JubjubAffine as CurveGroup>::Range::zero(); prover.m()];
            prover.instance_len() + prover.witness_len()
        ];
        let mut b_vals = vec![
            vec![<P::JubjubAffine as CurveGroup>::Range::zero(); prover.m()];
            prover.instance_len() + prover.witness_len()
        ];
        let mut c_vals = vec![
            vec![<P::JubjubAffine as CurveGroup>::Range::zero(); prover.m()];
            prover.instance_len() + prover.witness_len()
        ];
        for (i, w) in self.instance.keys().chain(self.witness.keys()).enumerate() {
            for (j, constr) in prover.constraints.iter().enumerate() {
                a_vals[i][j] = *constr
                    .a
                    .coefficients()
                    .get(w)
                    .unwrap_or(P::ScalarField::zero());
                b_vals[i][j] = *constr
                    .b
                    .coefficients()
                    .get(w)
                    .unwrap_or(P::ScalarField::zero());
                c_vals[i][j] = *constr
                    .c
                    .coefficients()
                    .get(w)
                    .unwrap_or(P::ScalarField::zero());
            }
        }

        let a_polys = a_vals
            .into_iter()
            .map(|col| fft.idft(PointsValue(col)))
            .collect::<Vec<_>>();
        let b_polys = b_vals
            .into_iter()
            .map(|col| fft.idft(PointsValue(col)))
            .collect::<Vec<_>>();
        let c_polys = c_vals
            .into_iter()
            .map(|col| fft.idft(PointsValue(col)))
            .collect::<Vec<_>>();

        for (a, (b, c)) in a_polys.iter().zip(b_polys.iter().zip(c_polys.iter())) {
            if !(a * b == *c) {
                return Ok(false);
            };
        }
        Ok(true)

        // Ok(prover
        //     .constraints
        //     .iter()
        //     .all(|constraint| constraint.evaluate(&prover.instance, &prover.witness)))
    }
}
