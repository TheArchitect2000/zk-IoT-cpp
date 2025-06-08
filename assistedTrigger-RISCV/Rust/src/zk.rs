use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use crate::trace_parser::TraceEntry;

#[derive(Debug, Clone)]
pub struct InstructionRow<F: Field> {
    pub pc: F,
    pub opcode: F,
    pub rs1_val: F,
    pub rs2_val: F,
    pub rd_val: F,
}

pub fn opcode_to_id(op: &str) -> u64 {
    match op {
        "add" => 1,
        "sub" => 2,
        "mul" => 3,
        "addi" => 4,
        "lw" => 5,
        "sw" => 6,
        _ => 0,
    }
}

pub fn convert_trace_to_rows(trace: &[TraceEntry]) -> Vec<InstructionRow<GoldilocksField>> {
    trace.iter().map(|entry| {
        let pc = GoldilocksField::from_canonical_u64(entry.pc);
        let opcode = GoldilocksField::from_canonical_u64(opcode_to_id(&entry.opcode));

        InstructionRow {
            pc,
            opcode,
            rs1_val: GoldilocksField::ZERO,
            rs2_val: GoldilocksField::ZERO,
            rd_val: GoldilocksField::ZERO,
        }
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace_parser::TraceEntry;

    #[test]
    fn test_convert_trace_to_rows() {
        let trace = vec![
            TraceEntry { pc: 0x1000, opcode: "add".to_string(), rd: None, rs1: None, rs2: None },
            TraceEntry { pc: 0x1004, opcode: "mul".to_string(), rd: None, rs1: None, rs2: None },
        ];

        let rows = convert_trace_to_rows(&trace);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].opcode, GoldilocksField::from_canonical_u64(1));
        assert_eq!(rows[1].opcode, GoldilocksField::from_canonical_u64(3));
    }
}
