use libc::{c_int, c_void, c_long};

// PT_DENY_ATTACH is 31 on macOS/iOS
const PT_DENY_ATTACH: c_int = 31;

extern "C" {
    fn ptrace(request: c_int, pid: c_int, addr: *mut c_char, data: c_int) -> c_int;
}

use libc::c_char;

extern "C" {
    fn mach_absolute_time() -> u64;
}

pub fn check_timing() -> bool {
    unsafe {
        let start = mach_absolute_time();
        
        // Perform some work that should be fast
        // But not optimized away entirely
        let mut sum = 0;
        for i in 0..1000 {
            sum += i;
        }
        std::hint::black_box(sum);

        let end = mach_absolute_time();
        
        // If the difference is too large, it might be single-stepping
        // Threshold needs tuning, but for 1000 iterations it should be very fast (< 1ms)
        // mach_absolute_time units are CPU dependent but roughly nanoseconds on recent Apple chips
        // Let's say if it takes more than 100ms (very conservative), something is wrong.
        // In a real debugger step, it would take seconds.
        if (end - start) > 100_000_000 { 
            return true;
        }
    }
    false
}

pub fn am_i_debugged() -> bool {
    unsafe {
        // ptrace(PT_DENY_ATTACH, 0, 0, 0) will cause the process to exit with ENOTSUP
        // if a debugger is currently attached.
        // If a debugger attaches LATER, it will also fail/exit.
        // This function doesn't return "true" if debugged, it actively prevents it.
        // However, for the sake of the API signature, we can return false here,
        // but the side effect is what we want.
        
        let ret = ptrace(PT_DENY_ATTACH, 0, std::ptr::null_mut(), 0);
        
        // If we are here, ptrace returned.
        // If it failed, it might mean we are already debugged (or other error).
        // But usually PT_DENY_ATTACH kills the app if debugged.
        if ret != 0 {
            return true;
        }
    }
    false
}
