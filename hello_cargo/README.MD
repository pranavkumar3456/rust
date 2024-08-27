#Creating a project in cargo

#cargo --version

$ cargo new hello_cargo

$ cd hello_cargo

#it creates a two file one i.e., Cargo.toml file and other src directory with main.rs file

#Using a Cargo to build and run a project

$cargo build

it builds a executable in ./target/debug/hello_cargo.exe or hello_cargo folder.

$cargo run

$cargo check --> compiling the code without building it everytime and producing executable.
                 and when product finally ready cargo build --release

