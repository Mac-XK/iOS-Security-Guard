use std::ptr;

// ARM64 Instructions
// RET: 0xC0035FD6 (Little Endian: D6 5F 03 C0)
const ARM64_RET: u32 = 0xD65F03C0;

// Unconditional Branch (B) opcode mask: 0x14000000 -> 0xFC000000
// We check if the instruction matches the B opcode pattern.
const ARM64_B_MASK: u32 = 0xFC000000;
const ARM64_B_OPCODE: u32 = 0x14000000;

use crate::start_protection;

pub fn check_integrity() -> bool {
    // Check if critical functions have been patched
    // We pass the address of the function to check
    if !check_function_integrity(start_protection as *const ()) {
        return false;
    }
    
    true
}

fn check_function_integrity(func_ptr: *const ()) -> bool {
    if func_ptr.is_null() {
        return false;
    }

    unsafe {
        let instruction_ptr = func_ptr as *const u32;
        let first_instruction = ptr::read_volatile(instruction_ptr);

        // Check for immediate RET (function disabled)
        if first_instruction == ARM64_RET {
            return false;
        }

        // Check for unconditional Branch (inline hook)
        // This is a very basic check. Sophisticated hooks might use other instructions (BR, BLR).
        if (first_instruction & ARM64_B_MASK) == ARM64_B_OPCODE {
            return false;
        }
    }

    true
}
