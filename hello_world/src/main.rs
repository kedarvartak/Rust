fn main() {
    println!("Hello World"); // println! is not an in built function but a macro
}

// First, println! Calls a Rust macro. If it had called a function instead,
// it would be entered as println (without !). Rust macros are a way to write code that
// generates code to extend Rust syntax


// Before running a Rust program, you must compile it using the Rust compiler by
// entering the rustc command and passing it the name of your source file
// -> rustc main.rs

// After compiling successfully, Rust outputs a binary executable.
// There are 2 extra files that are generated after compilation
// 1. main.exe -> executable
// 2. main.rdb -> debug file

// Rust is an ahead-of-time compiled language, meaning you can compile a program and
// give the executable to someone else, and they can run it even without having Rust
// installed