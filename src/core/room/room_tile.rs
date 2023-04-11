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

use bevy::{
    asset::AssetServer,
    prelude::{Color, Handle, Resource, Vec2},
    reflect::{Reflect, TypeUuid},
    utils::hashbrown::HashMap,
};
use bevy_asset_loader::prelude::AssetCollection;
use serde::{Deserialize, Serialize};

#[derive(AssetCollection, Debug, Reflect, Resource)]
pub struct TileAssets {
    #[asset(path = "Mods", collection(typed))]
    pub tiles: Vec<Handle<RoomTile>>,
}

#[derive(Resource)]
pub struct RoomTiles {
    pub tiles: HashMap<Vec2, RoomTileInstance>,
}

#[derive(Default, Debug, Serialize, Deserialize, Resource, TypeUuid)]
#[uuid = "ba2265d0-51d5-4e8c-b878-443b36ea63b6"]
pub struct RoomTile {
    pub name: String,
    pub description: String,
    pub tile_type: RoomTileType,
    pub color: Color,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum RoomTileType {
    #[default]
    Floor = 0,
    Decoration = 1,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RoomTileInstance {
    pub tile: String,
    pub position: Vec2,
}
