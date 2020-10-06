use bevy::{
    app::prelude::*,
    core::{Time, Timer},
    diagnostic::{Diagnostic, DiagnosticId, Diagnostics},
    ecs::{IntoQuerySystem, Res, ResMut},
};
use log::{debug, info};
use std::time::Duration;

/// An App Plugin that prints diagnostics to the console
pub struct DebugDiagnosticsPlugin {
    pub debug: bool,
    pub wait_duration: Duration,
    pub filter: Option<Vec<DiagnosticId>>,
}

/// State used by the [DebugDiagnosticsPlugin]
pub struct DebugDiagnosticsState {
    timer:  Timer,
    filter: Option<Vec<DiagnosticId>>,
}

impl Default for DebugDiagnosticsPlugin {
    fn default() -> Self {
        DebugDiagnosticsPlugin { debug: false,
                                 wait_duration: Duration::from_secs(1),
                                 filter: None, }
    }
}

impl Plugin for DebugDiagnosticsPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_resource(DebugDiagnosticsState {
            timer: Timer::new(self.wait_duration, true),
            filter: self.filter.clone(),
        });

        if self.debug {
            app.add_system_to_stage(
                stage::POST_UPDATE,
                Self::debug_diagnostics_debug_system.system(),
            );
        } else {
            app.add_system_to_stage(stage::POST_UPDATE,
                                    Self::debug_diagnostics_system.system());
        }
    }
}

impl DebugDiagnosticsPlugin {
    pub fn filtered(filter: Vec<DiagnosticId>) -> Self {
        DebugDiagnosticsPlugin { filter: Some(filter),
                                 ..Default::default() }
    }

    fn debug_diagnostic(diagnostic: &Diagnostic) {
        if let Some(value) = diagnostic.value() {
            info!("{:<65}: {:<10.4}", diagnostic.name, value);
            if let Some(average) = diagnostic.average() {
                print!("  (avg {:.4})", average);
            }

            info!("\n");
        }
    }

    pub fn debug_diagnostics_system(mut state: ResMut<DebugDiagnosticsState>,
                                    time: Res<Time>,
                                    diagnostics: Res<Diagnostics>) {
        state.timer.tick(time.delta_seconds);
        if state.timer.finished {
            info!("Diagnostics:");
            info!("{}", "-".repeat(70));
            if let Some(ref filter) = state.filter {
                for diagnostic in
                    filter.iter().map(|id| diagnostics.get(*id).unwrap())
                {
                    Self::debug_diagnostic(diagnostic);
                }
            } else {
                for diagnostic in diagnostics.iter() {
                    Self::debug_diagnostic(diagnostic);
                }
            }
        }
    }

    pub fn debug_diagnostics_debug_system(mut state: ResMut<DebugDiagnosticsState>,
                                          time: Res<Time>,
                                          diagnostics: Res<Diagnostics>) {
        state.timer.tick(time.delta_seconds);
        if state.timer.finished {
            debug!("Diagnostics (Debug):");
            debug!("{}", "-".repeat(70));
            if let Some(ref filter) = state.filter {
                for diagnostic in
                    filter.iter().map(|id| diagnostics.get(*id).unwrap())
                {
                    debug!("{:#?}\n", diagnostic);
                }
            } else {
                for diagnostic in diagnostics.iter() {
                    debug!("{:#?}\n", diagnostic);
                }
            }
        }
    }
}
