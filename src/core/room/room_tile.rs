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

use bevy::prelude::{Color, Component, Resource, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum RoomTileType {
    #[default]
    Floor,
    Decoration,
}

#[derive(Default, Debug, Serialize, Deserialize, Resource)]
pub struct RoomTile {
    id: String,
    name: String,
    description: String,
    tile_type: RoomTileType,
    color: Color,
}

#[derive(Default, Debug, Serialize, Deserialize, Component)]
pub struct RoomTileInstance<D: RoomTileData> {
    tile: String,
    position: Vec2,
    data: D,
}

pub trait RoomTileData {
    fn serialize(&self);
    fn deserialize(&self);
}
