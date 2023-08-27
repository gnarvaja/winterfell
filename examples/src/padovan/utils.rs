use winterfell::math::{FieldElement};

pub fn compute_pado_term<E: FieldElement>(n: usize) -> E {
    let mut t0 = E::ONE;
    let mut t1 = E::ONE;
    let mut t2 = E::ONE;
    let mut t3;

    for _ in 0..(n - 3) {
        t3 = t0 + t1;
        t0 = t1;
        t1 = t2;
        t2 = t3;
    }

    t2
}

#[cfg(test)]
pub fn build_proof_options(use_extension_field: bool) -> winterfell::ProofOptions {
    use winterfell::{FieldExtension, ProofOptions};

    let extension = if use_extension_field {
        FieldExtension::Quadratic
    } else {
        FieldExtension::None
    };
    ProofOptions::new(28, 8, 0, extension, 4, 7)
}
