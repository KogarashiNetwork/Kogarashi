# Nova Pallet Sample
Sample implementation to test Nova pallet functionality by importing on your pallet. You can find full version of tutorial [here](https://kogarashinetwork.github.io/tutorial/nova_pallet/).

## Dependency

```yml
pallet-nova = { git = "https://github.com/KogarashiNetwork/Kogarashi", branch = "master", default-features = false }
rand_core = {version="0.6", default-features = false }
```

## Test

```shell
$ cargo test --release
```

## Usage

1. Couple Nova pallet with your pallet

```rs
/// Coupling configuration trait with pallet_nova.
#[pallet::config]
pub trait Config: frame_system::Config + pallet_nova::Config {
    /// The overarching event type.
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
}
```

2. Define your custom circuit

```rs
#[derive(Debug, Clone, Default, PartialEq, Eq, Encode, Decode)]
pub struct ExampleFunction<Field: PrimeField> {
    mark: PhantomData<Field>,
}

impl<F: PrimeField> FunctionCircuit<F> for ExampleFunction<F> {
    fn invoke(z: &DenseVectors<F>) -> DenseVectors<F> {
        let next_z = z[0] * z[0] * z[0] + z[0] + F::from(5);
        DenseVectors::new(vec![next_z])
    }

    fn invoke_cs<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        z_i: Vec<FieldAssignment<F>>,
    ) -> Vec<FieldAssignment<F>> {
        let five = FieldAssignment::constant(&F::from(5));
        let z_i_square = FieldAssignment::mul(cs, &z_i[0], &z_i[0]);
        let z_i_cube = FieldAssignment::mul(cs, &z_i_square, &z_i[0]);

        vec![&(&z_i_cube + &z_i[0]) + &five]
    }
}
```

3. Use Nova pallet function in your pallet

```rs
// The module's dispatchable functions.
#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Sets the first simple storage value
    #[pallet::weight(10_000)]
    pub fn set_thing_1(
        origin: OriginFor<T>,
        val: u32,
        proof: RecursiveProof<T::E1, T::E2, T::FC1, T::FC2>,
        pp: PublicParams<T::E1, T::E2, T::FC1, T::FC2>,
    ) -> DispatchResultWithPostInfo {
        // Define the proof verification
        pallet_nova::Pallet::<T>::verify(origin, proof, pp)?;

        Thing1::<T>::put(val);

        Self::deposit_event(Event::ValueSet(1, val));
        Ok(().into())
    }
}
```

4. Execute function through library

```rs
let mut rng = get_rng();

let pp = PublicParams::<
        Bn254Driver,
        GrumpkinDriver,
        ExampleFunction<Fr>,
        ExampleFunction<Fq>,
    >::setup(&mut rng);

let z0_primary = DenseVectors::new(vec![Fr::from(0)]);
let z0_secondary = DenseVectors::new(vec![Fq::from(0)]);
let mut ivc =
    Ivc::<Bn254Driver, GrumpkinDriver, ExampleFunction<Fr>, ExampleFunction<Fq>>::init(
        &pp,
        z0_primary,
        z0_secondary,
    );

(0..2).for_each(|_| {
    ivc.prove_step(&pp);
});
let proof = ivc.prove_step(&pp);

new_test_ext().execute_with(|| {
    assert_ok!(SumStorage::set_thing_1(Origin::signed(1), 42, proof, pp));
    assert_eq!(SumStorage::get_sum(), 42);
});
```
