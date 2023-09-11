use std::path::Path;

use eframe::{
    egui::{self, TextStyle::*},
    epaint::{Color32, Vec2},
};

use crate::{
    library::fonts::gen_rich_text, model::config::Config, widget::button_icon::ButtonIcon, CLI,
};

pub struct ConfigVideoPlayerPath {
    checkmark: ButtonIcon,
}

impl ConfigVideoPlayerPath {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            checkmark: ButtonIcon::from_rgba_image_files(
                "video_player_path_checkmark",
                Path::new(CLI.assets_path.as_str())
                    .join("image")
                    .join("icon")
                    .join("checkmark.png"),
                ctx,
            ),
        }
    }

    pub fn show_config(&self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if ui
            .button(gen_rich_text(
                ctx,
                t!("ui.config.video_player_path.label"),
                Body,
                None,
            ))
            .clicked()
        {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                Config::set_video_player_path(Some(path.display().to_string()));
            }
        }
    }

    pub fn show_hint(&self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(video_player_path) = Config::video_player_path() {
            let max_height = ui.text_style_height(&Body);
            self.checkmark.show(Vec2::new(max_height, max_height), ui);
            ui.label(gen_rich_text(
                ctx,
                format!(
                    "{}: {video_player_path}",
                    t!("ui.config.video_player_path.set")
                ),
                Body,
                None,
            ));
        } else {
            ui.label(gen_rich_text(
                ctx,
                t!("ui.config.video_player_path.unset"),
                Body,
                Some(Color32::RED),
            ));
        }
    }
}
