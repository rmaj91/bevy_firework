use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

#[derive(Default, Clone, Copy, Debug, Reflect, Serialize, Deserialize)]
pub enum EmissionShape {
    #[default]
    Point,
    Sphere(f32),
    Circle {
        normal: Vec3,
        radius: f32,
    },
}

impl EmissionShape {
    pub fn generate_point(&self) -> Vec3 {
        match self {
            Self::Point => Vec3::ZERO,
            Self::Sphere(radius) => {
                let u = rand::random::<f32>() * 2.0 * PI;
                let v = rand::random::<f32>() * PI;
                let r = rand::random::<f32>();
                let horizontal_y = v.sin();
                let xz_factor = v.cos();
                let horizontal_x = -u.sin() * xz_factor;
                let horizontal_z = -u.cos() * xz_factor;

                Vec3::new(horizontal_x, horizontal_y, horizontal_z) * r * (*radius)
            }
            Self::Circle { normal, radius } => {
                let (u, r) = (rand::random::<f32>() * 2. * PI, rand::random::<f32>());
                Quat::from_rotation_arc(Vec3::Y, *normal)
                    * Quat::from_rotation_y(u)
                    * Vec3::new(r * radius, 0., 0.)
            }
        }
    }
}
