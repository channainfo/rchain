# Getting started

## Rust
To install Rust, the recommended way is to use rustup
```
# https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Now check the version of rustc, cargo, ...

```
rustc --version
cargo --version
```

### rustup basic

Set default Rust version number
```
rustup default 1.60.0
```

### Cargo package management

Generate a Rust project

```
cargo new hello_world
```

Add dependencies
```
# cargo edit to add package dependencies to be able to run sub command
cargo install cargo-edit

# add a package
cargo add num

```

## IDE
### VSCode
Prefer VSCode

### Extension
- rust-analyzer extension: Language server
- CodeLLDB extension: Debugging Rust and C++
- Better Toml extension: Syntax Highlight toml file
- crates: Dependencies management
- Error Lens: Improve errors visibility in the editor
- Tabnine: Code faster with AI code completion

