use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct TraceEntry {
    pub trace_num: usize,
    pub label: String,
    pub pc: u64,
    pub regs: [u64; 32],
    pub instructions: Vec<String>,
}

pub fn parse_qemu_trace(path: &str) -> Result<Vec<TraceEntry>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let mut entries = Vec::new();

    let mut trace_num = 0;
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.starts_with("IN: ") {
            let label = line[4..].trim().to_string();
            i += 1;

            let mut instructions = Vec::new();
            let mut pc: Option<u64> = None;
            let mut regs = [0u64; 32];
            let mut regs_found = 0;

            // Collect instructions until we hit a line starting with 'pc' or 'x'
            while i < lines.len() {
                let l = lines[i].trim();
                // Match line like: 0x00010194:  00002197   auipc gp,2
                if let Some(colon_pos) = l.find(':') {
                    if l.starts_with("0x") {
                        instructions.push(l.to_string());

                        if pc.is_none() {
                            let addr_str = &l[2..colon_pos]; // skip "0x"
                            pc = u64::from_str_radix(addr_str, 16).ok();
                        }

                        i += 1;
                        continue;
                    }
                }

                // Stop when we reach pc or register section
                if l.starts_with("pc") || l.starts_with("x") {
                    break;
                }

                i += 1;
            }

            // Parse pc and registers
            while i < lines.len() {
                let l = lines[i].trim();

                if l.starts_with("pc") {
                    let tokens: Vec<&str> = l.split_whitespace().collect();
                    if tokens.len() >= 2 {
                        pc = u64::from_str_radix(tokens[1], 16).ok();
                    }
                    i += 1;
                    continue;
                }

                if l.starts_with('x') {
                    let tokens: Vec<&str> = l.split_whitespace().collect();
                    let mut k = 0;
                    while k + 1 < tokens.len() && regs_found < 32 {
                        if let Ok(val) = u64::from_str_radix(tokens[k + 1], 16) {
                            regs[regs_found] = val;
                            regs_found += 1;
                        }
                        k += 2;
                    }
                    i += 1;
                    continue;
                }

                if l.starts_with("----") || l.starts_with("IN: ") {
                    break;
                }

                i += 1;
            }

            if let Some(pc_val) = pc {
                entries.push(TraceEntry {
                    trace_num,
                    label,
                    pc: pc_val,
                    regs,
                    instructions,
                });
                trace_num += 1;
            }
        } else {
            i += 1;
        }
    }

    Ok(entries)
}

