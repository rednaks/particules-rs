use crate::common::Stats;
use crate::particule::ParticuleType;
use crate::world::Settings;
use macroquad::prelude::vec2;
use macroquad::ui::{hash, root_ui, widgets};

pub fn handle_ui(settings: &mut Settings, stats: &Stats) {
    widgets::Window::new(hash!(), vec2(470., 50.), vec2(200., 200.))
        .label("Settings")
        .ui(&mut *root_ui(), |ui| {
            // STATS
            let count_text = format!("particules created: {}", stats.created_particules_count);
            widgets::Label::new(count_text).ui(ui);
            let world_particules_text = format!("world particules: {}", stats.world_particules);
            widgets::Label::new(world_particules_text).ui(ui);

            let fps_text = format!("FPS: {}", stats.fps);
            widgets::Label::new(fps_text).ui(ui);

            let mut choice = match settings.selected_particule_type {
                ParticuleType::SAND => 0,
                ParticuleType::WATER => 1,
                ParticuleType::OIL => 2,
                _ => unreachable!(),
            };

            // Settings
            ui.combo_box(
                hash!(),
                "Particule Type",
                &["Sand", "Water", "Oil"],
                &mut choice,
            );

            settings.selected_particule_type = match choice {
                0 => ParticuleType::SAND,
                1 => ParticuleType::WATER,
                2 => ParticuleType::OIL,
                _ => unreachable!(),
            };
        });
}
