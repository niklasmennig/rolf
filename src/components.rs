use bracket_terminal::{prelude::RGB, FontCharType};
use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Position{
    pub x : i32,
    pub y : i32
}

#[derive(Component)]
pub struct Drawable{
    pub glyph: FontCharType,
    pub color_fg : RGB,
    pub color_bg : RGB
}

#[derive(Component)]
pub struct PlayerControl{}