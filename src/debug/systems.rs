#[cfg(feature = "debug")]
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, ecs::system::SystemState, prelude::*, window::PrimaryWindow,
};

#[cfg(feature = "debug")]
use bevy_inspector_egui::{bevy_egui::EguiContext, egui};

#[cfg(feature = "debug")]
use crate::{
    debug::{DebugCategory, DebugConfig, generate_bug_report},
    debug_state_transitions,
};

/// Logs state transitions into console.
///
/// This system is provided to make debugging easier by tracking state changes.
#[cfg(feature = "debug")]
pub fn log_transitions<S: States>(mut transitions: EventReader<StateTransitionEvent<S>>) {
    // State internals can generate at most one event (of type) per frame.
    let Some(transition) = transitions.read().last() else {
        return;
    };

    let name = core::any::type_name::<S>();
    let StateTransitionEvent { exited, entered } = transition;
    debug_state_transitions!("{} transition: {:?} => {:?}", name, exited, entered);
}

/// Unified debug UI system that combines all development and debug tools
#[cfg(feature = "debug")]
pub fn unified_debug_ui_system(world: &mut World) {
    let mut state: SystemState<(
        Res<bevy::diagnostic::DiagnosticsStore>,
        ResMut<DebugConfig>,
        Query<&mut EguiContext, With<PrimaryWindow>>,
    )> = SystemState::new(world);

    let (diagnostics, mut debug_config, equi_query) = state.get_mut(world);
    let Ok(egui_context) = equi_query.single() else {
        return;
    };

    let mut ctx = egui_context.clone();

    egui::Window::new("🛠️ Development Tools").default_open(false).resizable(true).show(ctx.get_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Press");
            ui.code("Escape");
            ui.label("to toggle World Inspector");
        });

        ui.separator();

        // Performance metrics section
        ui.collapsing("📊 Performance Metrics", |ui| {
            if let Some(fps) =
                diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.smoothed())
            {
                ui.label(format!("FPS: {fps:.1}"));
            }

            if let Some(frame_time) = diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
                .and_then(|frame_time| frame_time.smoothed())
            {
                let frame_time_ms = frame_time * 1000.0;
                ui.label(format!("Frame Time: {frame_time_ms:.2}ms"));
            }
        });

        ui.separator();

        // Debug logging section
        ui.collapsing("🐛 Debug Logging", |ui| {
            ui.heading("Debug Categories");

            // Global controls
            ui.horizontal(|ui| {
                if ui.button("Enable All").clicked() {
                    debug_config.enable_all();
                }
                if ui.button("Disable All").clicked() {
                    debug_config.disable_all();
                }
            });

            ui.separator();

            // Individual category controls in a grid
            egui::Grid::new("debug_categories").num_columns(3).spacing([10.0, 4.0]).show(ui, |ui| {
                for &category in DebugCategory::all() {
                    let mut enabled = debug_config.is_category_enabled(category);

                    if ui.checkbox(&mut enabled, "").changed() {
                        debug_config.set_category_enabled(category, enabled);

                        // Also set environment variable for immediate effect
                        if enabled {
                            unsafe {
                                std::env::set_var(category.env_var(), "1");
                            }
                        } else {
                            unsafe {
                                std::env::remove_var(category.env_var());
                            }
                        }
                    }

                    ui.label(category.display_name());

                    // Show current status
                    let status = if category.is_env_enabled() { "🟢" } else { "🔴" };
                    ui.label(status);
                    ui.end_row();
                }
            });

            ui.separator();

            // Currently enabled categories
            let enabled_categories = debug_config.enabled_categories();
            if !enabled_categories.is_empty() {
                ui.label("Currently enabled:");
                for category in enabled_categories {
                    ui.horizontal(|ui| {
                        ui.label("•");
                        ui.label(category.display_name());
                    });
                }
            }
        });

        ui.separator();

        // File logging section
        ui.collapsing("📁 File Logging", |ui| {
            ui.checkbox(&mut debug_config.file_logging_enabled, "Enable file logging");

            ui.horizontal(|ui| {
                ui.label("Log directory:");
                let mut path_string = debug_config.log_file_path.to_string_lossy().into_owned();
                if ui.text_edit_singleline(&mut path_string).changed() {
                    debug_config.log_file_path = PathBuf::from(path_string);
                }
            });

            ui.horizontal(|ui| {
                ui.label("Max file size (MB):");
                let mut size_mb = (debug_config.max_log_file_size / (1024 * 1024)) as u32;
                if ui.add(egui::DragValue::new(&mut size_mb).range(1..=100)).changed() {
                    debug_config.max_log_file_size = (size_mb as u64) * 1024 * 1024;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Max log files:");
                ui.add(egui::DragValue::new(&mut debug_config.max_log_files).range(1..=20));
            });
        });

        ui.separator();

        // Bug report section
        ui.collapsing("🐞 Bug Reports", |ui| {
            if ui.button("Generate Bug Report").clicked() {
                match generate_bug_report() {
                    Ok(report) => {
                        let report_path = std::path::PathBuf::from("bug_report.txt");
                        if let Err(e) = std::fs::write(&report_path, report) {
                            error!("Failed to save bug report: {}", e);
                        } else {
                            info!("Bug report saved to {:?}", report_path);
                        }
                    }
                    Err(e) => {
                        error!("Failed to generate bug report: {}", e);
                    }
                }
            }

            ui.label(
                "Generates a comprehensive bug report with recent log entries from \
                 all enabled debug categories.",
            );
        });

        ui.separator();

        // Quick tips section
        ui.collapsing("💡 Quick Tips", |ui| {
            ui.label("Environment Variables:");
            ui.code("DEBUG_AI=1 cargo run --features debug");
            ui.code("DEBUG_TURNS=1 DEBUG_AI=1 cargo run --features debug");
            ui.separator();
            ui.label("Keyboard Shortcuts:");
            ui.horizontal(|ui| {
                ui.code("Escape");
                ui.label("- Toggle World Inspector");
            });
            ui.horizontal(|ui| {
                ui.code("`");
                ui.label("- Toggle UI Debug Overlay");
            });
        });
    });
}
