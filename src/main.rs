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

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_scene_hook::HookPlugin;

use crygotchi::core::cursor::world_cursor::WorldCursorPlugin;
use crygotchi::core::mods::modloader::ModloaderPlugin;
use crygotchi::core::room::room::RoomPlugin;
use crygotchi::GameState;

fn main() {
    App::new()
        //* State management
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Startup),
        )
        .add_loading_state(LoadingState::new(GameState::Startup).continue_to_state(GameState::Main))
        //* Third-party Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(HookPlugin)
        //* Game Plugins
        .add_plugin(ModloaderPlugin)
        .add_plugin(WorldCursorPlugin)
        .add_plugin(RoomPlugin)
        //* Global systems
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sun
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 15000.0,
            ..Default::default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 70.0,
            maximum_distance: 100.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // Floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // Camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}
