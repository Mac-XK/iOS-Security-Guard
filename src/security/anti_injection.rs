use std::ffi::CStr;
use libc::{c_char, c_uint};
use obfstr::obfstr;

extern "C" {
    fn _dyld_image_count() -> c_uint;
    fn _dyld_get_image_name(image_index: c_uint) -> *const c_char;
}

pub fn check_suspicious_dylibs() -> bool {
    unsafe {
        let count = _dyld_image_count();
        for i in 0..count {
            let name_ptr = _dyld_get_image_name(i);
            if !name_ptr.is_null() {
                let c_str = CStr::from_ptr(name_ptr);
                if let Ok(name) = c_str.to_str() {
                    // Check for suspicious libraries using obfuscated strings
                    // We check each one individually to avoid having a static array of plain strings
                    if name.contains(obfstr!("FridaGadget")) ||
                       name.contains(obfstr!("frida")) ||
                       name.contains(obfstr!("cynject")) ||
                       name.contains(obfstr!("libcycript")) ||
                       name.contains(obfstr!("CydiaSubstrate")) ||
                       name.contains(obfstr!("MobileSubstrate")) ||
                       name.contains(obfstr!("TweakInject")) ||
                       name.contains(obfstr!("SSLKillSwitch")) ||
                       name.contains(obfstr!("SSLKillSwitch2")) {
                        return true;
                    }
                }
            }
        }
    }
    false
}
