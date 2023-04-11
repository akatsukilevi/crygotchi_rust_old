use bevy::prelude::*;
use bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui;
use bevy_inspector_egui::bevy_inspector::{
    self, ui_for_entities_shared_components, ui_for_entity_with_children,
};
use bevy_inspector_egui::egui::{self, Color32};

use super::debug_assets::select_asset;
use super::debug_gizmos::draw_gizmo;
use super::debug_resources::select_resource;
use super::EguiWindow;
use super::InspectorSelection;
use super::TabViewer;

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui::Ui, window: &mut Self::Tab) {
        let world = self.world.into_inner();

        let type_registry = world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = type_registry.read();

        match window {
            EguiWindow::GameView => {
                *self.viewport_rect = ui.clip_rect();

                draw_gizmo(ui, &mut world, self.selected_entities, self.gizmo_mode);
            }
            EguiWindow::Hierarchy => {
                let selected = hierarchy_ui(&mut world, ui, self.selected_entities);
                if selected {
                    *self.selection = InspectorSelection::Entities;
                }
            }
            EguiWindow::Resources => select_resource(ui, &type_registry, self.selection),
            EguiWindow::Assets => select_asset(ui, &type_registry, &mut world, self.selection),
            EguiWindow::Inspector => match *self.selection {
                InspectorSelection::Entities => match self.selected_entities.as_slice() {
                    &[entity] => ui_for_entity_with_children(&mut world, entity, ui),
                    entities => ui_for_entities_shared_components(&mut world, entities, ui),
                },
                InspectorSelection::Resource(type_id, ref name) => {
                    ui.label(name);
                    bevy_inspector::by_type_id::ui_for_resource(
                        &mut world,
                        type_id,
                        ui,
                        name,
                        &type_registry,
                    )
                }
                InspectorSelection::Asset(type_id, ref name, handle) => {
                    ui.label(name);
                    bevy_inspector::by_type_id::ui_for_asset(
                        &mut world,
                        type_id,
                        handle,
                        ui,
                        &type_registry,
                    );
                }
            },
            EguiWindow::Room => {
                let tiles = world.resource::<crate::core::room::room_tile::TileAssets>();

                let room_query =
                    world.get_resource_mut::<crate::core::room::room_tile::RoomTiles>();

                if let Some(room) = room_query {
                    crate::core::room::room_debug::room_debug_ui(
                        &mut *self.room_editor,
                        ui,
                        room,
                        tiles,
                    );
                } else {
                    ui.colored_label(
                        Color32::from_rgb(255, 0, 0),
                        "Cannot render: No tiles resource",
                    );
                }
            }
            _ => {
                ui.label(format!("Window: {:#?}", window));
            }
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, EguiWindow::GameView)
    }
}
