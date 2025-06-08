use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::iop::witness::PartialWitness;
use plonky2::plonk::prover::prove;
use plonky2::util::timing::TimingTree;
use log::Level;
use plonky2::plonk::proof::ProofWithPublicInputs;

pub fn prove_addition_constraint(
    a: u64,
    b: u64,
    c: u64,
) -> ProofWithPublicInputs<GoldilocksField, PoseidonGoldilocksConfig, 2> {
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<GoldilocksField, 2>::new(config);

    let a_target = builder.constant(GoldilocksField::from_canonical_u64(a));
    let b_target = builder.constant(GoldilocksField::from_canonical_u64(b));
    let c_target = builder.constant(GoldilocksField::from_canonical_u64(c));

    let sum = builder.add(a_target, b_target);
    builder.is_equal(sum, c_target);

    let data = builder.build::<PoseidonGoldilocksConfig>();
    let mut pw = PartialWitness::new();
    let mut timing = TimingTree::new("prove", Level::Info);

    prove(&data.prover_only, &data.common, pw, &mut timing)
        .expect("Proof should succeed")
}
