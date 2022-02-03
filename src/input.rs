use bracket_terminal::prelude::VirtualKeyCode;
use specs::{Join, World, WorldExt};

use crate::components::{PlayerControl, Position};

fn key_to_dir(key : VirtualKeyCode) -> Option<(i32,i32)> {
    match key {
        VirtualKeyCode::Numpad1 => Some((-1, 1)),
        VirtualKeyCode::Numpad2 => Some(( 0, 1)),
        VirtualKeyCode::Numpad3 => Some(( 1, 1)),
        VirtualKeyCode::Numpad4 => Some((-1, 0)),
        VirtualKeyCode::Numpad5 => Some(( 0, 0)),
        VirtualKeyCode::Numpad6 => Some(( 1, 0)),
        VirtualKeyCode::Numpad7 => Some((-1,-1)),
        VirtualKeyCode::Numpad8 => Some(( 0,-1)),
        VirtualKeyCode::Numpad9 => Some(( 1,-1)),
        
        _=> None
    }
}

pub fn try_move_player(ecs : &World, dir : (i32, i32)) {
    let mut positions = ecs.write_storage::<Position>();
    let player_controls = ecs.read_storage::<PlayerControl>();

    for (pos,_) in (&mut positions, &player_controls).join() {
        pos.x += dir.0;
        pos.y += dir.1;
    }
}

pub fn handle_input(key : VirtualKeyCode, ecs : &World) {
    let dir = key_to_dir(key);

    // numpad movement
    match dir {
        Some(dir) => {
            try_move_player(ecs, dir);
        }
        None => {}
    }
}