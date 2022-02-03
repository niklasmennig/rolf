// Realm Of Legendary Fables

use bracket_terminal::prelude::*;
use specs::prelude::*;

mod components;
use components::*;

mod rendering;
mod input;
mod utils;
mod systems;
mod map;

struct State{
    ecs : World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match ctx.key {
            Some(key) => input::handle_input(key, &self.ecs),
            _ => {}
        }
        rendering::draw(ctx, &self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
    .with_title("rolf")
    .build()?;

    context.with_post_scanlines(true);

    let mut gs = State {
        ecs : World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Drawable>();
    gs.ecs.register::<PlayerControl>();

    gs.ecs.create_entity()
    .with(Position{x:5,y:5})
    .with(Drawable{
        glyph : to_cp437('@'),
        color_fg : RGB::named(RED),
        color_bg : RGB::named(BLACK)
    })
    .with(PlayerControl{})
    .build();

    main_loop(context, gs)
}
