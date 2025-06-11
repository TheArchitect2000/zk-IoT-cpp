use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::field::types::Field;
use riscv_trace_reader::{parse_trace, convert_trace_to_rows, prove_instruction_constraint};
use riscv_trace_reader::verifier::verify_instruction_proof;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;

fn main() {
    let trace = "./traces/sample_trace.log";
    let parsed = parse_trace(trace);
    let rows = convert_trace_to_rows(&parsed);

    let mut builder = CircuitBuilder::<F, C>::new(1024); // adjust size if needed
    let mut public_inputs = vec![];

    for (i, row) in rows.iter().enumerate() {
        let opcode_id = row.opcode.to_canonical_u64();
        let rs1 = row.rs1_val.to_canonical_u64();
        let rs2 = row.rs2_val.to_canonical_u64();
        let rd = row.rd_val.to_canonical_u64();

        if [1, 2, 3, 4, 7, 8].contains(&opcode_id) {
            println!("Adding constraints for row {}: opcode {}", i, opcode_id);
            // Assume this adds constraints into the builder
            prove_instruction_constraint(&mut builder, opcode_id, rs1, rs2, rd);

            // Optionally store public inputs for later proof checking
            public_inputs.extend([
                F::from_canonical_u64(opcode_id),
                F::from_canonical_u64(rs1),
                F::from_canonical_u64(rs2),
                F::from_canonical_u64(rd),
            ]);
        } else {
            println!("Skipping unsupported opcode {} at row {}", opcode_id, i);
        }
    }

    let circuit = builder.build::<C>();
    let proof = circuit.prove::<C>(public_inputs.clone()).unwrap();

    println!("✅ Aggregated proof generated with {} instructions.", rows.len());

    if verify_instruction_proof(proof.clone(), &circuit) {
        println!("✅ Aggregated proof verification succeeded.");
    } else {
        println!("❌ Aggregated proof verification failed.");
    }
}
