# Segment 1.2: Rendering the Board
We created a blank window in the last Segment. Lets spice it up now by rendering the board shall we?

## 1.2.1 Fixing a Window Size
Before we start rendering anything, remember that the default dimensions for the window are `800x600`?
When divided by 8 along both directions, it would give us a square size of `80x75`. Now I don't 
know bout you but I've always seen square shaped chessboards all my life so the very prospect of
a chessboard having rectangular squares makes me uncomfortable. So lets 'squarify' the window!
<br>
<br>
I'm gonna go with a size of `800x800`. Feel free to go with any size you want as long as it's a
square.<br>
Now that we have fixed a size, lets create a constant for this size outside the `main` function:
```rust
const WIN_SIZE: f32 = 800.0;
```
This is useful since now we can do all our rendering calculations with respect to this one variable,
in which case, if ever we feel dissatisfied with the window size, we can just tweak this constant
without worrying about rewriting any calculations.
<br>
<br>
Now, lets instruct ggez to draw the window in accordance to our preferred dimensions. We'll also
set a cool title for our window how bout that!<br>
Lets start by bringing some stuff into scope again:
```rust
// The WindowMode struct will allow us to set the dimensions
// and with the WindowSetup struct, we'll set the title of the window.
use ggez::conf::{WindowMode, WindowSetup};
```
And then we create instances of these with the required parameters. Both of these structs follow
the builder pattern, much like the `ContextBuilder`.
```rust
fn main() -> GameResult {
    let win_mode = WindowMode::default().dimensions(WIN_SIZE, WIN_SIZE);

    let win_setup = WindowSetup::default().title("My Cool Title");

    // All our other stuff
}
```
And finally, we'll tell our `ContextBuilder` to take these while building the context:
```rust
fn main() -> GameResult {
    // Stuff 

    let (mut ctx, mut event_loop) = ContextBuilder::new("Chess", "Me")
        .window_setup(win_setup)
        .window_mode(win_mode)
        .build()?;

    // Stuff
}
```
Now you can go ahead and `cargo run`. You'll notice that the window looks kind of different now ;)
