use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn palindrome_checker(word: *const c_char) -> bool {
    #[no_mangle]
pub extern "C" fn palindrome_checker(word_ptr: *const std::os::raw::c_char) -> bool {
    if word_ptr.is_null() {
        return false;
    }

    // --- UNSAFE BOUNDARY (keep it tiny) ---
    let word = unsafe {
        let c_str = std::ffi::CStr::from_ptr(word_ptr);
        match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        }
    };
    // -------------------------------------

    // Back to safe Rust! No more unsafe blocks needed.
    let reversed: String = word.chars().rev().collect();
    reversed == word
}
} 