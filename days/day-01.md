# Setting up Rust

You can [install Rust](https://www.rust-lang.org/tools/install) on UNIX systems with this shell command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will look different if you want to [install Rust on Windows](https://forge.rust-lang.org/infra/other-installation-methods.html).

## CLI

This bash script installed two tools: `rustup` and `cargo`.

The first, `rustup`, is your toolchain installer. You can use it to update your Rust version and install different compile targets. You can also install components with it, like the Rust formatter:
```bash
rustup component add rustfmt
```

Now you can format your code with `cargo fmt`.

The second, `cargo`, is Rust's package manager. You create projects with it, install third party depdencies, build and run Rust programs with.

# Tooling

## Rust Language Server

You need to install the [Rust Analyzer](https://rust-analyzer.github.io/manual.html#installation) to get auto-completion etc. in your IDE or code editor. You can install this via `rustup`:
```bash
rustup component add rust-analyzer
```

Or manually:
```bash
$ mkdir -p ~/.local/bin
$ curl -L https://github.com/rust-lang/rust-analyzer/releases/latest/download/rust-analyzer-x86_64-unknown-linux-gnu.gz | gunzip -c - > ~/.local/bin/rust-analyzer
$ chmod +x ~/.local/bin/rust-analyzer
```


## IDEs
* For larger projects, I recommend using [RustRover](https://www.jetbrains.com/rust/). It helps navigating large and complex code bases, and gives enough help to work with Rust for the first time.
* For smaller projects, [Zed](https://zed.dev/) is a perfect IDE. It gives enough help, and also supports [VIM mode](https://zed.dev/docs/vim).
* If you want to stay on the terminal, [helix](https://helix-editor.com/) is a modern, batteries-included editor for the command line.

# A first Rust project

You already have everything you need to create and run Rust programs now.

Create a new project with `cargo new mozilla-rust-workshop`.

```bash
> cargo new rust-in-20-days
    Creating binary (application) `rust-in-20-days` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

You can `cd` into the folder and see what was created:

```bash
> cd rust-in-20-days
> tree .
.
├── Cargo.toml
└── src
    └── main.rs

2 directories, 2 files
```

The `Cargo.toml` file is your so-called `manifest`. It contains the `name` of the project, and all the `dependencies` the program needs to compile. You can [check out the docs](https://doc.rust-lang.org/cargo/reference/manifest.html) to get a detailed view.

```toml
[package]
name = "rust-in-20-days"
version = "0.1.0"
edition = "2021"

[dependencies]
```

The `cargo new` command also created our first hello world program (in `src/main.rs`)

```rust
fn main() {
    println!("Hello, world!");
}
```

You can run it via `cargo run`:

```bash
> cargo run
   Compiling mozilla-rust-workshop v0.1.0 (/rust-in-20-days)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.01s
     Running `target/debug/rust-in-20-days`
Hello, world!
```

This command tells you multiple things:

* Rust programs have to be compiled before they can run.
* The created binary lives in the `target` folder.
* The profile it is using to compile is called `dev`.

If you create binaries for production, you would use the `cargo build --release` command. You can also use `cargo build` and then run the binary directly:

```bash
> cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
> ./target/debug/rust-in-20-days
Hello, world!
```

The command `cargo run` combines these two into one. Running `cargo build --release` creates an optimized binary in the `release` folder:

```bash
> cargo build --release
   Compiling mozilla-rust-workshop v0.1.0 (/rust-in-20-days)
    Finished `release` profile [optimized] target(s) in 0.36s
> ./target/release/mozilla-rust-workshop
Hello, world!
```

Go ahead and change the text in the `prinln!` call to ""Welcome to Firemarks!" and run it again.
