  * The main function is special: it is always the first code that runs in every executable Rust program. 
* Using a `!` means that you’re calling a macro instead of a normal function, and that macros don’t always follow the same rules as functions.
* Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. 
* `Cargo` is Rust’s build system and package manager. 
* In Rust, packages of code are referred to as crates. 
* Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code.
* Cargo.lock. This file keeps track of the exact versions of dependencies in your project. 
* Cargo Args:
  * New Project : `cargo new`
  * Build a Project: `cargo build`
  * Build with and executable: `cargo run`
  * Check for Complilation: `cargo check`

* Crates are of two types: 
  * Binary: Independent on their own and excute via the binary file.
  * Library: Provide specific functionalities and can't execute on their own.