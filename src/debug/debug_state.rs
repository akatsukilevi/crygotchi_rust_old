use bevy::prelude::*;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_inspector_egui::egui;
use egui_dock::{NodeIndex, Tree};
use egui_gizmo::GizmoMode;

use super::EguiWindow;
use super::InspectorSelection;
use super::TabViewer;
use super::UiState;

impl UiState {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec![EguiWindow::GameView]);
        let [inspector, _] = tree.split_right(NodeIndex::root(), 0.75, vec![EguiWindow::Inspector]);
        let [_, hierarchy] = tree.split_left(inspector, 0.25, vec![EguiWindow::Hierarchy]);

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
        }
    }

    pub fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            selected_entities: &mut self.selected_entities,
            selection: &mut self.selection,
            gizmo_mode: self.gizmo_mode,
        };

        egui::TopBottomPanel::top("main_menu").show(ctx, |panel| {
            egui::menu::bar(panel, |menu| {
                menu.menu_button("File", |submenu| {
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
