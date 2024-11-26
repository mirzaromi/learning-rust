# Cargo

Cargo is a package manager and build system for Rust.

## Creating a Project With Cargo

 to create rust project using cargo use `cargo new {project_name}` . Cargo will create a directory which name is equal to `project_name` 

There are 2 items inside that project, the first is the `src` directory that contain `main.rs`  file that cargo generated. the second is file  `Cargo.toml` . the `edition`  in `Cargo.toml`  indicates with version/edition of rust we use now. for example if the `edition` is 2021 then we use 2021 rust version.

## Building and Running Cargo Project

to build rust project use `cargo build` . this command will generate executable file in `target/debug/project_name` . You can run the executable file using this command `./target/debug/project_name` 

Running cargo for the first time also generate `Cargo.lock`. `Cargo.lock`  will keep track the dependencies inside your project and do not change that file manually!.

Using `cargo run`  for both compiling and running the project at the same time. if the code doesn’t change then `cargo run`  will only run the project otherwise it will compile first then run the project.

`cargo check`  is command used for checking if the code is executable or not. `cargo check`  is faster than `cargo build`  because it doesn’t build any executable file. this command is useful for checking whether the code is executable or not before compiling into executable file.

If you ready to ship your program, run `cargo build --release` . this command will create executable file inside `/target/relase` . it can take a little bit long since this process optimizes your code to run faster.