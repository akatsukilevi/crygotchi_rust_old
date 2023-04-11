use std::cell::RefCell;

use bevy::prelude::*;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_inspector_egui::egui;
use egui_dock::{NodeIndex, Tree};
use egui_gizmo::GizmoMode;

use crate::core::room::room_debug::RoomDebuggerState;

use super::DebugUiState;
use super::EguiWindow;
use super::InspectorSelection;
use super::TabViewer;

impl DebugUiState {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec![EguiWindow::GameView]);
        let [inspector, _] = tree.split_right(NodeIndex::root(), 0.8, vec![EguiWindow::Inspector]);
        let [center, hierarchy] = tree.split_left(inspector, 0.25, vec![EguiWindow::Hierarchy]);

        tree.split_below(center, 0.75, vec![EguiWindow::Room, EguiWindow::Save]);

        tree.split_below(
            hierarchy,
            0.5,
            vec![EguiWindow::Resources, EguiWindow::Assets],
        );

        Self {
            tree,
            selected_entities: SelectedEntities::default(),
            selection: InspectorSelection::Entities,
            viewport_rect: egui::Rect::NOTHING,
            gizmo_mode: GizmoMode::Translate,
            room_editor: RoomDebuggerState::default(),
        }
    }

    pub fn ui(&mut self, world: World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            world: RefCell::new(world),
            viewport_rect: &mut self.viewport_rect,
            selected_entities: &mut self.selected_entities,
            selection: &mut self.selection,
            room_editor: &mut self.room_editor,
            gizmo_mode: self.gizmo_mode,
        };

        egui::TopBottomPanel::top("main_menu").show(ctx, |panel| {
            egui::menu::bar(panel, |menu| {
                menu.menu_button("Main", |submenu| {
                    if submenu.button("Load Save").clicked() {
                        todo!();
                    }

                    if submenu.button("Write Save").clicked() {
                        todo!();
                    }

                    submenu.separator();

                    if submenu.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });

        egui_dock::DockArea::new(&mut self.tree)
            .style(egui_dock::Style {
                tab_bar_background_color: ctx.style().visuals.window_fill(),
                ..egui_dock::Style::from_egui(ctx.style().as_ref())
            })
            .show(ctx, &mut tab_viewer);
    }
}
