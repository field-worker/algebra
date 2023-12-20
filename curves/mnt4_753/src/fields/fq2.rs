use ark_ff::{
    fields::fp2::{Fp2, Fp2Config},
    Field, MontFp,
};

use crate::Fq;

pub type Fq2 = Fp2<Fq2Config>;

pub struct Fq2Config;

impl Fp2Config for Fq2Config {
    type Fp = Fq;

    // non_residue = 13
    const NONRESIDUE: Fq = MontFp!("13");

    // Coefficients:
    // [1, 41898490967918953402344214791240637128170709919953949071783502921025352812571106773058893763790338921418070971888253786114353726529584385201591605722013126468931404347949840543007986327743462853720628051692141265303114721689600]
    // see https://github.com/o1-labs/snarky/blob/2cf5ef3a14989e57c17518832b3c52590068fc48/src/camlsnark_c/libsnark-caml/depends/libff/libff/algebra/curves/mnt753/mnt4753/mnt4753_init.cpp
    const FROBENIUS_COEFF_FP2_C1: &'static [Self::Fp] = &[
        Fq::ONE,
        MontFp!("41898490967918953402344214791240637128170709919953949071783502921025352812571106773058893763790338921418070971888253786114353726529584385201591605722013126468931404347949840543007986327743462853720628051692141265303114721689600"),
    ];
}
