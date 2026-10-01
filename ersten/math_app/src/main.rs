#[link(name = "math", kind = "dylib")]
extern "C" {
    fn multiply(a: i32, b: i32) -> i32;
}

fn main3() {
    let result = unsafe { multiply_numbers(6, 7) };
    println!("Result from C++ code: {}", result);
}