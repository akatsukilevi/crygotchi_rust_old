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
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::GameState;

use super::room_tile::*;

pub struct RoomPlugin;
impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app
            //* Plugins
            .add_plugin(RonAssetPlugin::<RoomTile>::new(&["tiles.ron"]))
            .add_plugin(ResourceInspectorPlugin::<TileAssets>::default())
            //* Loaders
            .add_collection_to_loading_state::<_, TileAssets>(GameState::AssetLoading)
            .add_system(test_function.in_schedule(OnEnter(GameState::Main)));
    }
}

fn test_function(assets_collection: Res<TileAssets>, tiles: Res<Assets<RoomTile>>) {
    for tile in &assets_collection.tiles {
        if let Some(tile_data) = tiles.get(tile) {
            info!("Found tile {}", tile_data.name);
        }
    }
}
