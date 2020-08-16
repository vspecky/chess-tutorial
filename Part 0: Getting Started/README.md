# Part 0: Getting Started
## 0.1: Initializing the Project
Like all Rust projects, we will initialize this project using `cargo`, Rust's package manager. if you
don't have Rust installed yet, you can install it from the [Official Website](https://www.rust-lang.org/tools/install).
If you have Rust installed then you're all set.
<br>
Open up your Terminal (or Command Prompt if you're on Windows), navigate to your favorite directory and
type in the command `cargo new <project_name>` where you can replace `<project_name>` with whatever
you wanna name your project.
<br>
This will create a project directory with a Git repository and the following structure :-
```
.
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs
```
###### Side Note: Don't panic if you don't have the `Cargo.lock` file. It's a file that Cargo will generate automatically once we run the project.
This is the bare minimum we need to get started with our project. You will see that Cargo 
automatically generates a `main.rs` for us. The `main.rs` file is where we will
write all of our code. Right now, it contains a simple Hello World program (courtesy of Cargo).
```rust
fn main() {
    println!("Hello, world!");
}
```
You can type in the command `cargo run` and Cargo will compile and run your project, displaying the
Hello World message in the Terminal/Cmd.

## 0.2: Adding Dependencies
We will be using the `ggez` crate to implement the GUI of our game. ggez is a popular Rust library 
for making 2D games and is based upon the LOVE Framework. You can check out the official website [here](https://ggez.rs/).
To add ggez as a dependency, simple add the line `ggez = "0.5"` under `[dependencies]` in your
Cargo.toml. (The latest release version of `ggez` at the time of writing this tutorial is `0.5`)
<br>
Once you've done that, run the command `cargo build` inside your project directory. Cargo will fetch
the crate plus all the dependencies and compile them. This might take a while but once everything is
compiled once, we don't need to compile it again.
<br>
If you encounter any errors while building, you probably might be missing some required dev packages.
Check out [this page](https://github.com/ggez/ggez/blob/master/docs/BuildingForEveryPlatform.md).
<br>
<br>
Once you've done all that, congratulations, you are now ready to start implementing your very own
chess game! You can now move on to Part 1.
