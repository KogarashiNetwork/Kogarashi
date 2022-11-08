#[macro_export]
macro_rules! coordinate_operation {
    ($curve:ident) => {
        pub struct ProjectiveCoordinate<E: $curve> {
            pub(crate) x: E::ScalarField,
            pub(crate) y: E::ScalarField,
            pub(crate) z: E::ScalarField,
        }
    }
}

pub use coordinate_operation;
