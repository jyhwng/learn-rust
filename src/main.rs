use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;

fn main() -> GameResult {
    let (mut ctx, mut event_loop) =
       ContextBuilder::new("Bubble Puzzle", "jyhwng")
           .build()
           .unwrap();
    let ctx = &mut ctx;
    
    let mut state = State::new(ctx);

    graphics::set_drawable_size(ctx, 400.0, 600.0)?;
    
    event::run(ctx, &mut event_loop, &mut state)
}

#[derive(Debug)]
struct State {} // Type declaration

impl State {    // Implementation
    fn new(ctx: &mut Context) -> Self {
        State {}    // Expression oriented (<-> Statement) Always return the last 
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}