use plonky2::field::extension::Extendable;
use plonky2::field::types::Field;
use plonky2::field::types::RichField;
use plonky2::gadgets::arithmetic::CircuitBuilder;
use plonky2::gadgets::comparison::ComparisonGadget;
use plonky2::gadgets::boolean::Boolean;
use plonky2::plonk::config::GenericConfig;

pub fn prove_instruction_constraint<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    instr_name: &str,
    rs1: u64,
    rs2: u64,
    rd: u64,
) {
    let rs1_val = builder.constant(F::from_canonical_u64(rs1));
    let rs2_val = builder.constant(F::from_canonical_u64(rs2));
    let rd_val = builder.constant(F::from_canonical_u64(rd));

    match instr_name {
        "add" => {
            let sum = builder.add(rs1_val, rs2_val);
            let eq = builder.is_equal(sum, rd_val);
            builder.assert_bool(eq);
        }
        "mul" => {
            let prod = builder.mul(rs1_val, rs2_val);
            let eq = builder.is_equal(prod, rd_val);
            builder.assert_bool(eq);
        }
        "sub" => {
            let diff = builder.sub(rs1_val, rs2_val);
            let eq = builder.is_equal(diff, rd_val);
            builder.assert_bool(eq);
        }
        "sltu" => {
            let lt = builder.lt(rs1_val, rs2_val); // returns Boolean<F>
            let one = builder.constant(F::ONE);
            let zero = builder.zero();
            let is_one = builder.is_equal(rd_val, one);
            let is_zero = builder.is_equal(rd_val, zero);
            let or_result = builder.or(is_one, is_zero);
            builder.assert_bool(or_result);

            let lt_as_target = lt.target;
            builder.connect(rd_val, lt_as_target);
        }
        _ => panic!("Unsupported instruction: {}", instr_name),
    }
}
