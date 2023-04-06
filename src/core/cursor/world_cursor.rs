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
use bevy_scene_hook::{HookedSceneBundle, SceneHook};
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

use crate::GameState;

use super::cursor_actions::*;

pub struct WorldCursorPlugin;

#[derive(Serialize, Deserialize, Default, Debug, Component)]
pub struct WorldCursor {
    pub position: Vec3,
}

#[derive(Resource)]
struct CursorPack {
    cursor_scene: Handle<Scene>,
}

#[derive(Component, Debug)]
struct CursorIndicator;

#[derive(Component, Debug)]
struct CursorOutline;

impl FromWorld for CursorPack {
    fn from_world(world: &mut World) -> Self {
        let assets = world.get_resource::<AssetServer>().unwrap();
        let base_path = "Meshes/Cursor.glb#Scene0";

        CursorPack {
            cursor_scene: assets.load(base_path.to_owned()),
        }
    }
}

impl Plugin for WorldCursorPlugin {
    fn build(&self, app: &mut App) {
        app
            //* Resources
            .init_resource::<CursorPack>()
            //* Plugins
            .add_plugin(InputManagerPlugin::<CursorActions>::default())
            //* Startup
            .add_system(create_world_cursor.in_schedule(OnEnter(GameState::Startup)))
            //* Input
            .add_systems(
                (
                    super::cursor_movement::move_up,
                    super::cursor_movement::move_down,
                    super::cursor_movement::move_left,
                    super::cursor_movement::move_right,
                )
                    .in_set(OnUpdate(GameState::Main)),
            )
            //* State
            .add_system(update_cursor_state.in_set(OnUpdate(GameState::Main)));
    }
}

fn create_world_cursor(mut commands: Commands, cursor_pack: Res<CursorPack>) {
    commands
        .spawn(InputManagerBundle::<CursorActions> {
            action_state: ActionState::default(),
            input_map: get_cursor_mapping(),
        })
        .insert((
            HookedSceneBundle {
                scene: SceneBundle {
                    scene: cursor_pack.cursor_scene.clone(),
                    ..default()
                },
                hook: SceneHook::new(|entity, cmds| {
                    match entity.get::<Name>().map(|t| t.as_str()) {
                        Some("Indicator") => cmds.insert(CursorIndicator),
                        Some("Selection Outline") => cmds.insert(CursorOutline),
                        _ => cmds,
                    };
                }),
            },
            WorldCursor::default(),
        ));
}

fn update_cursor_state(
    time: Res<Time>,
    cursor_state: Query<&WorldCursor>,
    mut cursor_transform: Query<&mut Transform, With<WorldCursor>>,
    mut cursor_indicator: Query<&mut Transform, (With<CursorIndicator>, Without<WorldCursor>)>,
) {
    let state = cursor_state.single();
    let mut cursor = cursor_transform.single_mut();

    if let Ok(mut indicator) = cursor_indicator.get_single_mut() {
        indicator.rotate_y(0.05);
    }

    cursor.translation = cursor
        .translation
        .lerp(state.position, time.delta_seconds() * 25.0);
}
