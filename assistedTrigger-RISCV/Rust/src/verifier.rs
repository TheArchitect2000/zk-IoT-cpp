// src/verifier.rs

use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::proof::ProofWithPublicInputs;

pub fn verify_addition_proof(
    proof: ProofWithPublicInputs<GoldilocksField, PoseidonGoldilocksConfig, 2>,
    circuit: &CircuitData<GoldilocksField, PoseidonGoldilocksConfig, 2>,
) -> bool {
    circuit.verify(proof).is_ok()
}