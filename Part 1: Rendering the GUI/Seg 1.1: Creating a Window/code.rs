use ggez::event;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;

fn main() -> GameResult<()> {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Chess", "Me").build()?;

    let mut game = Game::new();

    event::run(&mut ctx, &mut event_loop, &mut game)
}

struct Game {}

impl Game {
    fn new() -> Self {
        Self {}
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}
