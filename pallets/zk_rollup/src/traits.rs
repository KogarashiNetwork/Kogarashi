pub trait Rollup {
    type F;
    type Transaction;
    type Batch;
    type Proof;
    type PublicKey;

    fn state_root() -> Self::F;
}
