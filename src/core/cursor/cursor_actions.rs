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

use bevy::prelude::KeyCode;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum CursorActions {
    Up,
    Down,
    Left,
    Right,
}

pub fn get_cursor_mapping() -> InputMap<CursorActions> {
    InputMap::new([
        (KeyCode::W, CursorActions::Up),
        (KeyCode::A, CursorActions::Left),
        (KeyCode::S, CursorActions::Down),
        (KeyCode::D, CursorActions::Right),
        (KeyCode::Up, CursorActions::Up),
        (KeyCode::Left, CursorActions::Left),
        (KeyCode::Down, CursorActions::Down),
        (KeyCode::Right, CursorActions::Right),
    ])
}
