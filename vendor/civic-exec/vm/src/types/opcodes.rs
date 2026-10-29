//! Operation codes of the CiviC VM

use libcivexec::operation::Operation;

/// Print current operation (instruction + operands) prefixed by the current
/// program counter
pub fn print_instr(pc: usize, op: &Operation<i16>) -> () {
    eprintln!("{:3>}: {}", pc, op.to_string());
}
