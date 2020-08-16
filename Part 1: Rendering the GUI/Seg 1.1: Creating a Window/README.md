# Segment 1.1: Creating a Window
In this segment, we will create a Window Instance for our GUI using the ggez crate. Lets dive right
into it!

## 1.1.1 Creating the Main Game Struct
Lets create our main game struct shall we? The reason we need to create this now is because of the way
ggez chooses to perform event handling. It basically exposes a trait called `EventHandler` to us. This
trait contains methods using which we can perform event handling. However, to use these methods, we 
need to implement the trait on a struct, which is our Main Game Struct.
<br>
For starters, we will simply create an Empty struct since we aren't implementing any game logic for now.
```rust
struct Game {}
```
We will also create an `impl` block for our struct with a `new` method to initialize the struct.
```rust
impl Game {
    fn new() -> Self {
        Self {}
    }
}
```
And there we have it, a basic main struct!

## 1.1.2 Implementing the EventHandler Trait
Lets implement the aforementioned `EventHandler` trait on our struct. We begin by bringing some stuff
into scope at the top of the file
```rust
// The event module contains the EventHandler trait.
use ggez::event;
use ggez::GameResult;
use ggez::Context;
```
###### `GameResult` is an enum type similar to `Result` that is returned by most functions in ggez and `Context` is a type that contains the context of the current ggez instance.
Next, we create a `impl` block for the trait on our main struct. For now, we will add the required
trait methods without implementing the logic.
```rust
impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}
```
###### We prefix the `ctx` arguments with an underscore to tell the compiler that "We know we aren't using those arguments in the functions, please don't warn us".
With this, we have the minimum of what's needed to create a window. Lets get started with creating
the window itself now shall we?

## 1.1.3 Creating the Window
To display a Window, we need to run a ggez instance, aka a ggez Event Loop. We start by bringing
the `ContextBuilder` into scope. We can use this struct to build the Context of our game.
```rust
use ggez::ContextBuilder;
```
Next, erase the previous Hello World code from the main function. Here we will write code using the
`ContextBuilder` to build the Context and the Event Loop of our Game. We will also give the main
function a return type of `GameResult<()>` for error handling purposes.
```rust
fn main() -> GameResult<()> {
    // The ContextBuilder's new method takes two arguments, a game id and the name of the author.
    // These can be anything you want. We subsequently call the build method without any further
    // modifications to build the Context and Event Loop with default values, and add the '?' operator
    // to unwrap the GameResult we get, propagating any Error to the top of the main function.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Chess", "Me")
        .build()?;

    // Lets create an instance of our game while we're at it.
    let mut game = Game::new();
}
```
And finally, we call the `ggez::event::run` function, passing in our context, event loop and game instance
which fires up the event loop and brings up a `800 x 600` Window.
```rust
    // At the bottom of the main function.
    // We don't put a semicolon at the end cuz this function returns a GameResult when the Event Loop terminates,
    // which we return out of the main function.
    event::run(&mut ctx, &mut event_loop, &mut game)
}
```
And that's pretty much it. Execute `cargo run` and your new main function will compile, run and bring
up a blank window!
<br>
In subsequent segments, we will turn the blackness of the window into a chess GUI. You're ready to
move onto `Segment 1.2`. Refer to the `code.rs` and `total.rs` files for the code so far.

## 1.1.4 Committing our Changes
Hey! Don't forget to commit your changes now. Although, lets not commit to the master branch. Lets
create a development branch and commit to that instead for all of the segments. We will merge the
master branch with the development branch at the end of Part 1. This is usually considered good
practice since this way the master branch remains free of any incomplete code and it's simpler to
roll back if we perform any blunders (Altho we won't in this tutorial). If you're not familiar with
how Git branching and merging works, I recommend you check out [this amazing video by David Mahler](https://www.youtube.com/watch?v=FyAAIHHClqI).
<br>
We create a branch called 'devel' (You can call it whatever you want) and switch to it.
```
git branch devel

git checkout devel
```
And commit our changes.
```
git add src/main.rs

git commit -m "Added main game struct and basic event loop"
```
And now we're officially done with this segment. Lets move on to `Segment 1.2`!
