use super::utils::compute_pado_term;
use crate::{Blake3_192, Blake3_256, Example, ExampleOptions, HashFunction, Sha3_256};
use core::marker::PhantomData;
use log::debug;
use std::time::Instant;
use winterfell::{
    crypto::{DefaultRandomCoin, ElementHasher},
    math::{fields::f128::BaseElement, FieldElement},
    DefaultTraceLde, ProofOptions, Prover, StarkProof, Trace, TraceTable, VerifierError,
};

mod air;
use air::PadoAir;

mod prover;
use prover::PadoProver;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

const TRACE_WIDTH: usize = 3;

// PADOVAN EXAMPLE
// ================================================================================================

pub fn get_example(
    options: &ExampleOptions,
    sequence_length: usize,
) -> Result<Box<dyn Example>, String> {
    let (options, hash_fn) = options.to_proof_options(28, 8);

    match hash_fn {
        HashFunction::Blake3_192 => Ok(Box::new(PadoExample::<Blake3_192>::new(
            sequence_length,
            options,
        ))),
        HashFunction::Blake3_256 => Ok(Box::new(PadoExample::<Blake3_256>::new(
            sequence_length,
            options,
        ))),
        HashFunction::Sha3_256 => Ok(Box::new(PadoExample::<Sha3_256>::new(
            sequence_length,
            options,
        ))),
        _ => Err("The specified hash function cannot be used with this example.".to_string()),
    }
}

pub struct PadoExample<H: ElementHasher> {
    options: ProofOptions,
    sequence_length: usize,
    result: BaseElement,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> PadoExample<H> {
    pub fn new(sequence_length: usize, options: ProofOptions) -> Self {
        assert!(
            (sequence_length % 3) == 0,
            "sequence length must multiple of 3"
        );
        assert!(
            (sequence_length / 3).is_power_of_two(),
            "sequence length / 3 must be a power of 2"
        );

        // compute Padovan sequence
        let now = Instant::now();
        let result = compute_pado_term(sequence_length);
        debug!(
            "Computed Padovan sequence up to {}th term in {} ms",
            sequence_length,
            now.elapsed().as_millis()
        );

        PadoExample {
            options,
            sequence_length,
            result,
            _hasher: PhantomData,
        }
    }
}

// EXAMPLE IMPLEMENTATION
// ================================================================================================

impl<H: ElementHasher> Example for PadoExample<H>
where
    H: ElementHasher<BaseField = BaseElement>,
{
    fn prove(&self) -> StarkProof {
        debug!(
            "Generating proof for computing Padovan sequence (3 terms per step) up to {}th term\n\
            ---------------------",
            self.sequence_length
        );

        // create a prover
        let prover = PadoProver::<H>::new(self.options.clone());

        // generate execution trace
        let now = Instant::now();
        let trace = prover.build_trace(self.sequence_length);

        let trace_width = trace.width();
        let trace_length = trace.length();
        debug!(
            "Generated execution trace of {} registers and 2^{} steps in {} ms",
            trace_width,
            trace_length.ilog2(),
            now.elapsed().as_millis()
        );

        // generate the proof
        prover.prove(trace).unwrap()
    }

    fn verify(&self, proof: StarkProof) -> Result<(), VerifierError> {
        winterfell::verify::<PadoAir, H, DefaultRandomCoin<H>>(proof, self.result)
    }

    fn verify_with_wrong_inputs(&self, proof: StarkProof) -> Result<(), VerifierError> {
        winterfell::verify::<PadoAir, H, DefaultRandomCoin<H>>(proof, self.result + BaseElement::ONE)
    }
}
