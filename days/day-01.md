# Day 1: Getting started with Rust

Before we start, **there are a few, very important mindsets** to adopt before switching from, let's say Python, to Rust:

* You won't be able to *know* the whole language (and you don't need you).
* Read documentation. In Rust, knowing what types are and what they can do is the most important thing. [The documentation](https://doc.rust-lang.org) is your go-to tool. Crates (Rusts third-party libraries) also come with documentation by default.
* Rust code can look complex, but this is more often than not, enforced by the compiler. Don't think people are smarter just because their code looks "weirdly complex". You will get used to reading Rust code.
* The compiler is first your enemy, and then your best friend. It takes a while to get used to not working with running code (like in scripting languages) and fix errors along the way. In Rust, you work **with** the compiler to make sure your code is correct *before* it runs.
* It takes a few weeks to get used to Rust. Once you are, it is hard to go back to a language without such a great type system and compiler. Keep at it, don't try to see progress every day. Give yourself a fixed time slot of 20 days to just work on the course.

In learning programming languages, there can be a moment of "Oh my, what else is there? How much do I need to know?". In Rust, you need to know the following concepts to be able to navigate the language and figure things out for yourself:

* Basic types
* Creating your own types (via `struct`)
* Traits
* Default implemantation of traits
* Ownership and Borrowing
* Error handling
* How to navigate the Rust documentation
* (Generics, but this is not needed for a while, and not as complex as you might think)
* (Lifetimes, but you might never face lifetimes, and on some level, not as complex as you think)
* (Macros, but you may not ever need to write your own, and it's just important to know how they generally work)

You can write Rust applications, productively, for years on end, without ever having to see a lifetime, a generic or write your own macros. Once you are familiar with Rust (as with any other language), adding one more thing isn't as hard as thinking you need to learn and understand these concepts first before you can call yourself proficient in Rust.

With this being said, let's have some fun with Rust!

## Setting up Rust

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

## Tooling

### Rust Language Server

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

### IDEs
* For larger projects, I recommend using [RustRover](https://www.jetbrains.com/rust/). It helps navigating large and complex code bases, and gives enough help to work with Rust for the first time.
* For smaller projects, [Zed](https://zed.dev/) is a perfect IDE. It gives enough help, and also supports [VIM mode](https://zed.dev/docs/vim).
* If you want to stay on the terminal, [helix](https://helix-editor.com/) is a modern, batteries-included editor for the command line.

# Kick-off the project

You have everything ready to become a Rust developer. Go ahead and create our project.

```bash
> cargo new firemarks
> cd firemarks
```

You can even run it and see what it will print out:

```bash
> cargo run
   Compiling rust-in-20-days v0.1.0 (/firemarks)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/firemarks`
Hello, World!
```

## Core Concepts

We will go through a first set of important concepts. We won't cover all, and you will get used to learning as you go. Don't feel like you need to know everything to be productive in Rust.

That's also your chance to try out the excellent [Rust Playground](https://play.rust-lang.org/). Especially in the beginning, if you want to figure out why certain pieces of code don't work, make a small example and run it in the browser.

### Variables and Types

#### Variables

In Rust, variables are immutable by default.

```rust
// Immutable - cannot be changed
let x = 5;
x = 6; // ERROR!

// Mutable - can be changed
let mut y = 5;
y = 6;
```

You can also "shadow" variables, so you don't have to reinvent new names in longer function bodys:

```rust
let name = "Rust";
println!("{}", name);  // Prints: Rust

// We will cover Strings in a bit.
let name = "Hello, ".to_string() + name;  // Shadows the previous `name`
println!("{}", name);  // Prints: Hello, Rust
```

#### Basic types

Rust is statically typed, but it can infer types:

```rust
let number = 5;    // Rust infers i32
let float = 5.0;   // Rust infers f64
let boolean = true;// Rust infers bool
```

You can also be explicit about the type (and sometimes in larger code bases, you might have to help the compiler with that technique):

```rust
let number: i32 = 5;
let float: f64 = 5.0;
let boolean: bool = true;
```

### Understanding Strings

This is one of the most important concepts in Rust. There are two main string types:

**String Slice (&str)**
* Think of it as "borrowing" text
* Fixed length
* Cannot be modified
* Like looking at text through a window

```rust
let text: &str = "Hello";  // String literal
```

**String**

* Owned text that you can modify
* Can grow or shrink
* Lives on the heap
* Like having your own copy of text that you can change

```rust
let text: String = String::from("Hello");
```

**Key differences**
* Use &str when you just need to read text
* Use String when you need to modify text or own it

### Understanding Vectors

A vector is a growable array. Think of it as a list that can change size.

```rust
// Create an empty vector
let mut list = Vec::new();

// Add items
list.push("item");

// Read items
let first = list[0];
```

### Input/Output Basics

Rust handles I/O through the standard library:

```rust
use std::io::{self, Write};

// Reading input
let mut input = String::new();
io::stdin().read_line(&mut input)?;

// Writing output
println!("Hello!");  // With newline
print!("Hello!");    // Without newline
```

# Today's Project: Simple Bookmark Storage

We're building a simple program that:

* Asks for URLs
* Stores them in memory
* Displays them back

We covered everything you need to know to create this program. But, not everything. For printing out the stored bookmarks, you will have to iterate over a vector.

In the example code for this day, you will find the following snippet to print out the stored bookmarks:

```rust
for (index, bookmark) in bookmarks.iter().enumerate() {
    println!("{}. {}", index + 1, bookmark);
}
```

## 1. What is an Iterator?

An iterator is an object that allows you to step through (or "iterate over") the elements in a collection, such as a `Vec` (vector) or an `array`, one by one.

In Rust, when you call `.iter()` on a collection like a vector (`Vec`), it returns an `Iterator` that can traverse each element in the vector. The `Iterator` itself is not the data, but a way to access the data, element by element.

If bookmarks is a vector containing three bookmarks:

```rust
let bookmarks = vec!["Bookmark 1", "Bookmark 2", "Bookmark 3"];
```

Calling `bookmarks.iter()` gives you an iterator that can be used to access "Bookmark 1", "Bookmark 2", and "Bookmark 3" one by one.

## 2. The .enumerate() Method:

`.enumerate()` is a method that you can call on an `Iterator`, and it modifies the `Iterator`. It makes the `Iterator` return a pair: the index (position) of each element in the collection and the element itself.

## How would I ever know that?

This is where it is important to get behind the idea of strictly typed languages. Every type has capabilities, and working with your bookmarks vector, you have to ask yourself: "What can a vector do?". The easiest way to find out is to have auto-completion and language hints in your IDE:

![Rust autocompletion for your IDE](https://raw.githubusercontent.com/gruberb/rust-in-20-days/day-01/days/ide.png)

Later on, you will have to get used to opening up a browser window, and navigating to the [Rust docs of the iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate) and find methods it is implementing.

You won't understand all of that as of now, but trust me, you will soon. And once it clicked (it will), it is pretty easy to navigate.
