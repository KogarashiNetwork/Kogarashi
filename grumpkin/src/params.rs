use bn_254::Fr;

// g1 curve parameters
pub const GENERATOR_X: Fr = Fr::one();
pub const GENERATOR_Y: Fr = Fr::new_unchecked([
    0x11b2dff1448c41d8,
    0x23d3446f21c77dc3,
    0xaa7b8cf435dfafbb,
    0x14b34cf69dc25d68,
]);

// bn curve b param
pub(crate) const PARAM_B: Fr = Fr::new_unchecked([
    0xdd7056026000005a,
    0x223fa97acb319311,
    0xcc388229877910c0,
    0x034394632b724eaa,
]);
pub const PARAM_B3: Fr = PARAM_B.add_const(PARAM_B).add_const(PARAM_B);
