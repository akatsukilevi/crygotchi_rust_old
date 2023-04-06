/*
Copyright (c) 2023 CovenFox Studios

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{cursor_actions::CursorActions, world_cursor::WorldCursor};

pub fn move_up(
    query: Query<&ActionState<CursorActions>, With<WorldCursor>>,
    mut cursor: Query<&mut WorldCursor>,
) {
    if !query.single().just_pressed(CursorActions::Up) {
        return;
    }

    cursor.single_mut().position += Vec3 {
        x: 0.0,
        y: 0.0,
        z: -2.0,
    };
}

pub fn move_down(
    query: Query<&ActionState<CursorActions>, With<WorldCursor>>,
    mut cursor: Query<&mut WorldCursor>,
) {
    if !query.single().just_pressed(CursorActions::Down) {
        return;
    }

    cursor.single_mut().position += Vec3 {
        x: 0.0,
        y: 0.0,
        z: 2.0,
    };
}

pub fn move_left(
    query: Query<&ActionState<CursorActions>, With<WorldCursor>>,
    mut cursor: Query<&mut WorldCursor>,
) {
    if !query.single().just_pressed(CursorActions::Left) {
        return;
    }

    cursor.single_mut().position += Vec3 {
        x: -2.0,
        y: 0.0,
        z: 0.0,
    };
}

pub fn move_right(
    query: Query<&ActionState<CursorActions>, With<WorldCursor>>,
    mut cursor: Query<&mut WorldCursor>,
) {
    if !query.single().just_pressed(CursorActions::Right) {
        return;
    }

    cursor.single_mut().position += Vec3 {
        x: 2.0,
        y: 0.0,
        z: 0.0,
    };
}
