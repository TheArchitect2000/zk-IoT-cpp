use expectrl::{spawn, Regex, Session};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut gdb = spawn("riscv64-unknown-elf-gdb test.elf")?;
    gdb.set_expect_timeout(Some(Duration::from_secs(10)));

    // Wait for GDB prompt
    let initial = gdb.expect(Regex(".*gdb.*"))?;
    println!("Initial output: {:?}", initial);

    // Send a command
    gdb.send_line("info registers")?;

    // Read response
    let response = gdb.expect(Regex(".*ra.*"))?; // Match register output like `ra`
    println!("Register output: {:?}", response);

    Ok(())
}

// Helper to parse a MI value string like: ^done,value="0x0000000000010194"
fn parse_value(line: &str) -> Result<u64> {
    let re = Regex::new(r#"value="0x([0-9a-fA-F]+)""#)?;
    let cap = re.captures(line).ok_or_else(|| anyhow::anyhow!("No match"))?;
    Ok(u64::from_str_radix(&cap[1], 16)?)
}

// Helper to parse register values
fn parse_registers(output: &str) -> Result<Vec<u64>> {
    let mut values = Vec::new();
    let re = Regex::new(r#""value":"0x([0-9a-fA-F]+)""#)?;
    for cap in re.captures_iter(output) {
        values.push(u64::from_str_radix(&cap[1], 16)?);
    }
    Ok(values)
}



// mod trace;
// use trace::parse_qemu_trace;
// use std::error::Error;

// fn main() -> Result<(), Box<dyn Error>> {
//     let entries = parse_qemu_trace("qemu.log")?;
//     for entry in entries {
//         println!("Trace {}: {}", entry.trace_num, entry.label);
//         println!("PC: {:#x}", entry.pc);
//         println!("Instructions:");
//         for instr in &entry.instructions {
//             println!("  {}", instr);
//         }
//         println!("Registers:");
//         for (i, reg) in entry.regs.iter().enumerate() {
//             println!("  x{}: {:#018x}", i, reg);
//         }
//         println!("\n-------------------------\n");
//     }
//     Ok(())
// }

