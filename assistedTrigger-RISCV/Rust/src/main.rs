use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::field::types::PrimeField64;

use riscv_trace_reader::{run_qemu, parse_trace, convert_trace_to_rows, prove_addition_constraint, InstructionRow};
use riscv_trace_reader::verifier::verify_addition_proof;

fn main() {
    let bin = "./test.bin";
    let trace = "./traces/sample_trace.log";

    // run_qemu(bin, trace);
    let parsed = parse_trace(trace);
    let rows = convert_trace_to_rows(&parsed);
    
    for (i, row) in rows.iter().enumerate() {
        println!("Row {}: opcode = {}", i, row.opcode.to_canonical_u64());
    }

    let add_opcode = GoldilocksField::from_canonical_u64(1);

    if let Some(first_add) = rows.iter().find(|row| row.opcode == add_opcode) {
        let a = first_add.rs1_val.to_canonical_u64();
        let b = first_add.rs2_val.to_canonical_u64();
        let c = first_add.rd_val.to_canonical_u64();

        let (proof, circuit) = prove_addition_constraint(a, b, c);
        println!("Proof generated: {:?}", proof.public_inputs);
        
        if verify_addition_proof(proof, &circuit) {
            println!("✅ Proof verification succeeded.");
        } else {
            println!("❌ Proof verification failed.");
        }
    } else {
        println!("No instructions found.");
    }

}
