mod security;

use security::{anti_debug, anti_injection, integrity};

#[no_mangle]
pub extern "C" fn start_protection() -> i32 {
    // 1. Anti-Debug (ptrace)
    if anti_debug::am_i_debugged() {
        crash_app();
    }

    // 2. Timing Check (Anti-Stepping)
    if anti_debug::check_timing() {
        crash_app();
    }

    // 3. Anti-Injection (Dyld checks)
    if anti_injection::check_suspicious_dylibs() {
        crash_app();
    }

    // 4. Integrity Check (Anti-Patching)
    if !integrity::check_integrity() {
        crash_app();
    }

    0 // All clear
}

#[inline(always)]
fn crash_app() {
    unsafe {
        // Trigger a hardware breakpoint/trap
        // This causes an immediate crash (SIGTRAP/SIGBUS) that is hard to ignore/hook
        #[cfg(target_arch = "aarch64")]
        std::arch::asm!("brk #1");

        #[cfg(not(target_arch = "aarch64"))]
        std::process::abort();
    }
}
