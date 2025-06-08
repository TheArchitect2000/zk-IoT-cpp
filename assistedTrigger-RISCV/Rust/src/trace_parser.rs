use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct TraceEntry {
    pub pc: u64,
    pub opcode: String,
    pub rd: Option<String>,
    pub rs1: Option<String>,
    pub rs2: Option<String>,
}

pub fn run_qemu(binary_path: &str, trace_path: &str) {
    Command::new("qemu-riscv64")
        .arg("-d")
        .arg("in_asm,cpu")
        .arg("-D")
        .arg(trace_path)
        .arg(binary_path)
        .status()
        .expect("Failed to run QEMU");
}

pub fn parse_trace(trace_path: &str) -> Vec<TraceEntry> {
    let file = File::open(trace_path).expect("Cannot open trace log");
    let reader = BufReader::new(file);
    let re_instr = Regex::new(r"^\s*([0-9a-f]+):\s+([a-z0-9._]+)").unwrap();

    reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let caps = re_instr.captures(&line)?;
            Some(TraceEntry {
                pc: u64::from_str_radix(&caps[1], 16).ok()?,
                opcode: caps[2].to_string(),
                rd: None,
                rs1: None,
                rs2: None,
            })
        })
        .collect()
}
