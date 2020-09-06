# Segment 1.3: Rendering the Pieces
Alright, time to get into the nitty gritty stuff! We will first take a look at what are the various
chess pieces and how they are denoted in official chess engines. If you're already familiar with this,
you can skip ahead to `1.3.2`, no problem!

## 1.3.1 But What is a Chess Piece?
A chess piece is simply a movable entity that can be used by a player to... well... play chess.
<br>
There are a total of 6 types of chess pieces, each with two variants (Black and White), they are:
- Pawn ( ♙ , ♟ )
- Knight ( ♘ , ♞ )
- Bishop ( ♗ , ♝ )
- Rook ( ♖ , ♜ )
- Queen ( ♕ , ♛ )
- King ( ♔ , ♚ )

For convenience, these pieces are usually denoted using characters as follows:
<br>
For White:
- `P` - Pawn
- `N` - Knight
- `B` - Bishop
- `R` - Rook
- `Q` - Queen
- `K` - King

And for Black:
- `p` - Pawn
- `n` - Knight
- `b` - Bishop
- `r` - Rook
- `q` - Queen
- `k` - King

As you can see, the characters are the same, just with different capitalizations. We will be using
these character representations to represent our chessboard too. We will look into how
each piece functions in the next Part. For now we are only concerned about rendering the pieces in the
form of the initial chessboard configuration.


## 1.3.2 Coding the Pieces on The Board
You might have seen separate classes for each piece being used in other implementations of chess. However
we are not gonna do anything like that. The method we use is going to be simple and straightforward.
We will just create another 2D array like the one we used for the Chessboard. Except this one will be
a `[[char; 8]; 8]` in which we will represent every piece using its character at the index position
corresponding to the piece's actual position on the board.
<br>
<br>
With that being said, lets start coding the Pieceboard!
<br>
Initially, lets add the field to the main struct:
```rust
struct Game {
    // Stuff above

    board_pieces: [[char; 8]; 8],

    // Stuff below
}
```
And modify the `new` method too:
```rust
impl Game {
    fn new() -> Self {
        // Stuff above

        let board = [['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
                     ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
                     ['-', '-', '-', '-', '-', '-', '-', '-'],
                     ['-', '-', '-', '-', '-', '-', '-', '-'],
                     ['-', '-', '-', '-', '-', '-', '-', '-'],
                     ['-', '-', '-', '-', '-', '-', '-', '-'],
                     ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
                     ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']];

        Self {
            // Other Fields

            board_pieces: board
        }
    }
}
```
In the code above, we create the board with all the pieces in their initial positions, with White at
the bottom. Now that we have our board representation, lets move onto the next step.

## 1.3.3 Acquiring and Loading the Assets
To render the pieces, we need to have images of the various pieces. Luckily for us, Wikimedia has
us covered. The PNGs they provide are public domain and hence free to use. You can get em [here](https://commons.wikimedia.org/wiki/Category:PNG_chess_pieces/Standard_transparent).
<br>
<br>
HALT! Before you download them, there's something I need to tell ya. See, ggez has a system for managing
assets. ggez by default will look for assets in a folder called `resources` which it would look for in
the directory of the executable.
<br>
When we execute the command `cargo run`, the executable that runs is the debug executable which is located
in `target/debug/`. Hence, when this executable runs, ggez will look for assets in the folder `target/debug/resources/`
which doesn't exist.
<br>
<br>
Now you might be like "Okay, so lets create the folder". And to be honest, you would be right. Creating
the folder and putting all the assets in it would work. However, ggez also gives us the option of specifying
additional directories in which to look for assets. We will make use of this feature and store all our
assets in `src/assets/`. This way, we can keep our code and assets in the same parent folder.
<br>
So what are you waiting for? Go ahead and create the folder!
<br>
<br>
Another thing to note is, while downloading the images, save each image with a single character name
corresponding to the piece it represents. For example, the image for the black pawn shall receive the
name `p.png`, while that of the white queen will receive the same `Q.png`, etc. This is important since
we will be making use of the filenames to load the pieces in memory!
<br>
Alright now you can go ahead and download all of the pieces. Come back when you're done.  
.  
.  
.  
.  

