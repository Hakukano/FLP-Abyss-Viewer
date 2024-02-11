use parking_lot::RwLock;

pub mod app_config;
pub mod playlist;

pub struct Models {
    pub app_config: RwLock<Box<dyn app_config::AppConfig>>,
}

impl Models {
    pub fn new() -> Self {
        Self {
            app_config: RwLock::new(app_config::instantiate()),
        }
    }
}
