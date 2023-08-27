use super::{super::utils::build_proof_options, Blake3_256};

#[test]
fn pado2_test_basic_proof_verification() {
    let pado = Box::new(super::PadoExample::<Blake3_256>::new(
        48,
        build_proof_options(false),
    ));
    crate::tests::test_basic_proof_verification(pado);
}

#[test]
fn pado2_test_basic_proof_verification_extension() {
    let pado = Box::new(super::PadoExample::<Blake3_256>::new(
        48,
        build_proof_options(true),
    ));
    crate::tests::test_basic_proof_verification(pado);
}

#[test]
fn pado2_test_basic_proof_verification_fail() {
    let pado = Box::new(super::PadoExample::<Blake3_256>::new(
        48,
        build_proof_options(false),
    ));
    crate::tests::test_basic_proof_verification_fail(pado);
}
