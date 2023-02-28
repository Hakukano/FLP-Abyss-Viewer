pub mod args;
pub mod json;

use std::{ops::RangeInclusive, sync::RwLock};

use anyhow::{anyhow, Result};
use clap::ValueEnum;
use once_cell::sync::OnceCell;
use serde::Deserialize;

use crate::{get_cli, locale};

pub const AUTO_INTERVAL_RANGE: RangeInclusive<u32> = 1..=60;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, ValueEnum)]
pub enum MediaType {
    Unset,
    Image,
    Video,
}

impl MediaType {
    pub fn is_unset(&self) -> bool {
        matches!(self, Self::Unset)
    }
}

impl Default for MediaType {
    fn default() -> Self {
        Self::Unset
    }
}

impl ToString for MediaType {
    fn to_string(&self) -> String {
        let media_type = &locale::get().ui.config.media_type;
        match self {
            Self::Unset => "--".to_string(),
            Self::Image => media_type.image.clone(),
            Self::Video => media_type.video.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, ValueEnum)]
pub enum VideoPlayer {
    Unset,
    Qtp,
    Vlc,
}

impl VideoPlayer {
    pub fn is_unset(&self) -> bool {
        matches!(self, Self::Unset)
    }
}

impl Default for VideoPlayer {
    fn default() -> Self {
        Self::Unset
    }
}

impl ToString for VideoPlayer {
    fn to_string(&self) -> String {
        let video_player = &locale::get().ui.config.video_player;
        match self {
            Self::Unset => "--".to_string(),
            Self::Qtp => video_player.qtp.clone(),
            Self::Vlc => video_player.vlc.clone(),
        }
    }
}

#[derive(Default, Deserialize)]
pub struct Config {
    pub media_type: MediaType,
    pub root_path: Option<String>,

    pub repeat: bool,
    pub auto: bool,
    pub auto_interval: u32,
    #[serde(rename = "loop")]
    pub lop: bool,
    pub random: bool,

    pub video_player: VideoPlayer,
    pub video_player_path: Option<String>,
}

impl Config {
    pub fn validate(self) -> Result<Self> {
        if !AUTO_INTERVAL_RANGE.contains(&self.auto_interval) {
            return Err(anyhow!(format!(
                "auto_interval should be in range {:?} but found {}",
                AUTO_INTERVAL_RANGE, self.auto_interval
            )));
        }

        Ok(self)
    }
}

pub fn get() -> &'static RwLock<Config> {
    static CONFIG: OnceCell<RwLock<Config>> = OnceCell::new();
    CONFIG.get_or_init(|| {
        let cli = get_cli();
        let config = match (
            cli.config_file.as_ref(),
            cli.media_type,
            cli.root_path.as_ref(),
            cli.video_player,
        ) {
            (None, MediaType::Unset, None, VideoPlayer::Unset) => Config::default(),
            (Some(config_file), _, _, _) => json::new(config_file),
            _ => args::new(cli),
        };
        RwLock::new(config)
    })
}
