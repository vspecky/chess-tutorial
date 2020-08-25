use ggez::conf::{WindowMode, WindowSetup};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect};

const WIN_SIZE: f32 = 800.0;
const WHITE_COLOR: u8 = 200;
const BLACK_COLOR: u8 = 50;

fn main() -> GameResult<()> {
    let win_mode = WindowMode::default().dimensions(WIN_SIZE, WIN_SIZE);

    let win_setup = WindowSetup::default().title("My Cool Title");

    let (mut ctx, mut event_loop) = ContextBuilder::new("Chess", "Me")
        .window_setup(win_setup)
        .window_mode(win_mode)
        .build()?;

    // Stuff
}

struct Game {
    board: [[Color; 8]; 8],
    needs_draw: bool,
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            board: [[Color::from_rgb(WHITE_COLOR, WHITE_COLOR, WHITE_COLOR); 8]; 8],
            needs_draw: true,
        };

        game.reset_board();

        game
    }

    fn reset_board(&mut self) {
        for y in 0..8 {
            let row_even = y % 2 == 0;

            for x in 0..8 {
                let col_even = x % 2 == 0;

                self.board[y][x] = if row_even == col_even {
                    Color::from_rgb(WHITE_COLOR, WHITE_COLOR, WHITE_COLOR)
                } else {
                    Color::from_rgb(BLACK_COLOR, BLACK_COLOR, BLACK_COLOR)
                };
            }
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.needs_draw {
            return Ok(());
        }

        graphics::clear(ctx, Color::from_rgb(0, 0, 0));

        let sq_size = (WIN_SIZE / 8.0) as i32;

        for (y, row) in self.board.iter().enumerate() {
            for (x, sq) in row.iter().enumerate() {
                let x_pos = x as i32 * sq_size;
                let y_pos = y as i32 * sq_size;

                let rect = Rect::new_i32(x_pos, y_pos, sq_size, sq_size);
                let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, *sq)?;

                graphics::draw(ctx, &mesh, DrawParam::default())?;
            }
        }

        self.needs_draw = false;

        graphics::present(ctx)
    }
}
