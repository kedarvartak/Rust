fn main() {
    println!("Hello, world!");
}
// We can create a new project in an existing dir using cargo init, rest is same as cargo new
// We can create a project using cargo new.
// We can build a project using cargo build.
// We can build and run a project in one step using cargo run.
// We can build a project without producing a binary to check for errors using cargo check.
// Instead of saving the result of the build in the same directory as our code, Cargo stores it
// in the target/debug directory.


// With simple projects, Cargo doesn’t provide a lot of value over just using rustc, but it will prove
// its worth as your programs become more intricate. Once programs grow to multiple files or need a
// dependency, it’s much easier to let Cargo coordinate the build.