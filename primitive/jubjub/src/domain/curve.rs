macro_rules! curve_operation {
    ($curve:ident) => {
        impl Add for $curve {
            type Output = $curve;

            #[inline]
            fn add(self, rhs: $curve) -> $curve {
                add(self, rhs)
            }
        }

        impl<'a, 'b> Add<&'b $curve> for &'a $curve {
            type Output = $curve;

            #[inline]
            fn add(self, rhs: &'b $curve) -> $curve {
                add(self.clone(), rhs.clone())
            }
        }

        impl AddAssign for $curve {
            fn add_assign(&mut self, rhs: $curve) {
                self.add(rhs)
            }
        }
    };
}

pub(crate) use curve_operation;
