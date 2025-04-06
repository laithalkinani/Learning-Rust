fn main() {
    println!("Hello, world!");
}

/*
Running cargo new hello_cargo in cmd creates this src file and Cargo.toml, which holds dependencies information
If we started a project without using cargos, we can just run cargo init to generate the Cargo.toml for that project,
and then we have to place that source file in the src folder
Cargo always expects your source files to be inside the src folder (organized!)
Q: does hello_cargo automatically produce a Hello,World! fn?
Yes, it always does.

Now run cargo build in cmd
Or cargo run and it will build / run the exe in one go (most people use this)

cargo check: compiles but doesn't run, just to check for you

for release:
cargo build --release
runs with optimizations

*/
