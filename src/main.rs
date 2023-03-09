extern crate image;
extern crate nalgebra;

use image::{Rgb, RgbImage};
use nalgebra::{Rotation3, Vector3};

// Width or height, always square
const SIZE: i32 = 64;
const ANGLE: f32 = 15.0 * std::f32::consts::PI / 180.0;

fn main() {
    let mut img = RgbImage::new(11, 11);
    for x in 0..11 {
        for y in 0..11 {
            let z = Vector3::z();
            // These are the direction they tilt the vector in, not the axes to rotate around
            let x_angle = ((x as f32 - 5.0) * 15.0).to_radians();
            let y_angle = ((y as f32 - 5.0) * 15.0).to_radians();
            let norm_vec = Rotation3::from_euler_angles(y_angle, x_angle, 0.0) * z;
            let rgb_vec = norm_vec * 127.5 + Vector3::new(127.5, 127.5, 127.5);
            let rgb: Vec<u8> = rgb_vec.as_slice().iter().map(|x| x.ceil() as u8).collect();
            let rgb: [u8; 3] = rgb.try_into().unwrap();
            img.put_pixel(x, y, Rgb(rgb));
        }
    }
    img.save("normsquare.png").unwrap();
}

struct RadAngle {
    radius: f32,
    angle: f32,
}

struct Coord {
    x: i32,
    y: i32,
}

impl From<Coord> for RadAngle {
    fn from(coord: Coord) -> Self {
        const HALFSIZE: f32 = SIZE as f32 / 2.0;
        let x = (coord.x as f32) - HALFSIZE;
        let y = -(coord.y as f32) - HALFSIZE;
        Self {
            radius: x.hypot(y) / HALFSIZE,
            angle: y.atan2(x),
        }
    }
}

fn discretise(val: f32, chunks: f32) -> f32 {
    (val / chunks).round() * chunks
}
