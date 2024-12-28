use std::{
    fs,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    sync::OnceLock,
};

use error_stack::ResultExt;
use rust_3d::AABBTree3D;
use serde::Deserialize;

mod camera;
mod devices;

pub use camera::CameraProperties;
pub use devices::Device;

use crate::GError;

fn read_json_from_file<T: for<'de> Deserialize<'de>>(
    path: impl AsRef<Path>,
) -> error_stack::Result<T, GError> {
    serde_json::from_str(
        &fs::read_to_string(path.as_ref())
            .change_context(GError::ConfigError)
            .attach_printable("Couldn't read the config file")?,
    )
    .change_context(GError::ConfigError)
}

fn hpe_addr() -> String {
    "/tmp/hpe.sock".into()
}

fn head_detection_addr() -> String {
    "/tmp/head.sock".into()
}

fn gesture_detection_addr() -> String {
    "/tmp/gesture.sock".into()
}

fn picam_addr() -> String {
    "/tmp/picam.sock".into()
}

const fn pool_size() -> usize {
    3
}

#[derive(Deserialize)]
pub struct BaseConfig {
    pub camera1_pos: (f64, f64, f64),
    pub devices: Vec<Device>,
    #[serde(default = "hpe_addr")]
    pub hpe_addr: String,
    #[serde(default = "head_detection_addr")]
    pub head_detection_addr: String,
    #[serde(default = "gesture_detection_addr")]
    pub gesture_detection_addr: String,
    #[serde(default = "picam_addr")]
    pub picam_addr: String,
    #[serde(default = "pool_size")]
    pub pool_size: usize,
}

pub struct Config {
    pub camera1: CameraProperties,
    pub camera2: CameraProperties,
    pub base_config: BaseConfig,
    aabbtree: OnceLock<AABBTree3D<Device>>,
}

impl Deref for Config {
    type Target = BaseConfig;

    fn deref(&self) -> &Self::Target {
        &self.base_config
    }
}

impl DerefMut for Config {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base_config
    }
}

impl Config {
    pub fn open(dir: PathBuf) -> error_stack::Result<Self, GError> {
        let camera1 = dir.join("camera1-params.json");
        let camera2 = dir.join("camera2-params.json");
        let base_config = dir.join("config.json");
        let camera1: CameraProperties = read_json_from_file(camera1)?;
        let camera2: CameraProperties = read_json_from_file(camera2)?;
        let base_config: BaseConfig = read_json_from_file(base_config)?;

        Ok(Self {
            camera1,
            camera2,
            base_config,
            aabbtree: OnceLock::new(),
        })
    }

    pub fn aabbtree(&self) -> &AABBTree3D<Device> {
        self.aabbtree
            .get_or_init(|| AABBTree3D::new(self.devices.clone(), usize::MAX, 1))
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn parse_config() {
        let config_toml = r#"
        [camera1]
        fov_x = 0.3
        fov_y = 0.3
        pos_x = 0
        pos_y = 0
        pos_z = 0
        pitch = -1
        yaw = -0.5
        roll = 0

        [camera2]
        fov_x = 0.3
        fov_y = 0.3
        pos_x = 3
        pos_y = 3
        pos_z = 3
        pitch = -1
        yaw = 1
        roll = 0

        [[devices]]
        name = "Fist of Family Values"
        min_x = -69
        min_y = -69
        min_z = -69
        max_x = -37
        max_y = -37
        max_z = -37

        [[devices]]
        name = "Distributor of Freedom"
        min_x = 0
        min_y = 0
        min_z = 0
        max_x = 37
        max_y = 37
        max_z = -37"#;

        //let config: Config = toml::from_str(config_toml).unwrap();

        //assert_eq!(config.devices.len(), 2);
    }
}
