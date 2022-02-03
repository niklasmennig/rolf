use specs::{World, WorldExt, Join};
use bracket_terminal::prelude::{BTerm};

use crate::components::{Position, Drawable};


pub fn draw(ctx : &mut BTerm, ecs : &World) {
    let positions = ecs.read_storage::<Position>();
    let drawables = ecs.read_storage::<Drawable>();

    ctx.cls();

    for (pos, drw) in (&positions, &drawables).join() {
        ctx.set(pos.x, pos.y, drw.color_fg, drw.color_bg, drw.glyph);
    }

}