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
use std::fs;

use crate::GameState;

use super::Modlist;

#[derive(Resource)]
struct ModHandle(Handle<Modlist>);

pub struct ModloaderPlugin;
impl Plugin for ModloaderPlugin {
    fn build(&self, app: &mut App) {
        app
            //* Loaders
            .add_system(load_mods.in_schedule(OnEnter(GameState::AssetLoading)));
    }
}

fn load_mods(mut commands: Commands) {
    //* Attept to fetch all mods from the mod folder
    if let Ok(entries) = fs::read_dir("assets/Mods") {
        let mods = entries
            .map(|res| res.map(|e| e.path()).unwrap())
            .filter(|res| res.is_dir());

        let collected: Vec<String> = mods.map(|res| res.to_str().unwrap().to_string()).collect();

        //* Insert new resource
        info!("Discovered {} mods available", collected.iter().count());
        commands.insert_resource(Modlist {
            // path: mod_path.to_str().unwrap().to_string(),
            mods: collected,
        });
    }
}
