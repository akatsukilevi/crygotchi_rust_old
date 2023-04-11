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

use std::any::TypeId;
use std::cell::RefCell;

use bevy::prelude::*;
use bevy_asset::HandleId;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_inspector_egui::egui::{self};
use egui_dock::Tree;
use egui_gizmo::GizmoMode;

use crate::core::room::room_debug::RoomDebuggerState;

pub mod debug_assets;
pub mod debug_gizmos;
pub mod debug_resources;
pub mod debug_state;
pub mod debug_tabs;
pub mod debug_ui;

#[derive(Eq, PartialEq)]
pub enum InspectorSelection {
    Entities,
    Resource(TypeId, String),
    Asset(TypeId, String, HandleId),
}

#[derive(Resource)]
pub struct DebugUiState {
    pub tree: Tree<EguiWindow>,
    pub viewport_rect: egui::Rect,
    pub selected_entities: SelectedEntities,
    pub selection: InspectorSelection,
    pub gizmo_mode: GizmoMode,
    pub room_editor: RoomDebuggerState,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum EguiWindow {
    GameView,
    Hierarchy,
    Resources,
    Assets,
    Inspector,
    Room,
    Save,
}

pub struct TabViewer<'a> {
    pub world: RefCell<World>,
    pub selected_entities: &'a mut SelectedEntities,
    pub selection: &'a mut InspectorSelection,
    pub viewport_rect: &'a mut egui::Rect,
    pub gizmo_mode: GizmoMode,
    pub room_editor: &'a mut RoomDebuggerState,
}
