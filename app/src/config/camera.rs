use nalgebra as na;
use serde::Deserialize;

// TODO: remove redundant pos fields
#[derive(Deserialize, Debug)]
pub struct CameraProperties {
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub img_height: u32,
    pub img_width: u32,
    pub intrensic_params: na::Matrix3<f64>,
    pub rotation_matrix: na::Matrix3<f64>,
}

impl Default for CameraProperties {
    fn default() -> Self {
        Self {
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            img_height: 0,
            img_width: 0,
            intrensic_params: Default::default(),
            rotation_matrix: Default::default(),
        }
    }
}

impl CameraProperties {
    pub fn test_new() -> Self {
        let sample_intrensic_matrix = na::matrix![
            1.425_355_597_530_572e3, 0., 7.255_278_875_079_987e22;
            0., 1.403_960_548_626_719_9e3, 4.003_098_490_699_321e2;
            0., 0., 1.
        ];
        let sample_rotation_matrix = na::matrix![
            1.0, 0.0, 0.0;
            0.0, 1.0, 0.0;
            0.0, 0.0, 1.0
        ];
        CameraProperties {
            intrensic_params: sample_intrensic_matrix,
            rotation_matrix: sample_rotation_matrix,
            img_height: 720,
            img_width: 1280,
            ..Default::default()
        }
    }
}
