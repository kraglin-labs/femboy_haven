#[link(name = "math", kind = "dylib")]
unsafe extern "C" {
    fn power(base: i32, exponent: i32) -> i32;
    fn multiply(a: i32, b: i32) -> i32;
}

fn main() {
    let multiplication = unsafe { multiply(6, 9) };
    let exponential = unsafe { power(7, 14) };
    println!(" 6 * 9 = {}", multiplication);
    println!(" 7 ** 14 = {}", exponential);
}