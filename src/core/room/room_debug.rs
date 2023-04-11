use bevy::prelude::Mut;
use bevy_asset::Handle;
use bevy_inspector_egui::egui::{self, Ui};

use super::room_tile::{RoomTile, RoomTiles, TileAssets};

#[derive(Default, Debug)]
pub struct RoomDebuggerState {
    show_selector: bool,
    selected_tile: Handle<RoomTile>,
}

pub fn room_debug_ui(
    mut state: &mut RoomDebuggerState,
    ui: &mut Ui,
    room: Mut<RoomTiles>,
    assets: &TileAssets,
) {
    ui.label(format!("Tiles ({} in room):", room.tiles.iter().count()));

    if state.show_selector {
        egui::Window::new("Select tile type").show(ui.ctx(), |selector| {
            let mut selected = &state.selected_tile;

            if selector.button("Cancel").clicked() {
                state.show_selector = false;
            }

            //* For each type, add a button to add it
            egui::ComboBox::from_label("Tile to create")
                .selected_text(format!("{:#?}", selected))
                .show_ui(selector, |ui| {
                    for tile in assets.tiles.iter() {
                        ui.selectable_value(&mut selected, tile, format!("{:#?}", tile));
                    }
                });

            state.selected_tile = selected.to_owned()
        });
    } else {
        if ui.button("Add new tile").clicked() {
            state.show_selector = true;
        }
    }
}
