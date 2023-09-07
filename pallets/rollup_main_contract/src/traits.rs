pub trait MainContract {
    type F;
    type Transaction;
    type Batch;
    type Proof;
    type PublicKey;

    fn state_root() -> Self::F;
}
