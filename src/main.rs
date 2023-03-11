extern crate image;
extern crate nalgebra;

use image::{Rgb, RgbImage};
use nalgebra::{Rotation3, Vector3};
use std::env;

// Width or height, always square
const DEFAULT_SIZE: u32 = 256;
const DEFAULT_ANGLE: f32 = 15.0 * std::f32::consts::PI / 180.0;

const USAGE: &str = "
Create a discretised RGB sphere of tangent-space normals for use in hand-creating normal maps.
Usage:    
    normsphere [size [angle]]
`size` is the pixel width and height of the resulting image (default 256)
`angle` is the size of the angle step in degrees azimuth and inclination of the normal vector (default 15.0)";

fn main() {
    let mut args = env::args().skip(1);
    let size: u32 = match args.next() {
        Some(x) => x.parse().expect(USAGE),
        None => DEFAULT_SIZE,
    };

    let angle: f32 = match args.next() {
        Some(x) => x.parse::<f32>().expect(USAGE) * std::f32::consts::PI / 180.0,
        None => DEFAULT_ANGLE,
    };

    let mut img = RgbImage::new(size, size);
    for y in 0..size {
        for x in 0..size {
            // These are the direction they tilt the vector in, not the axes to rotate around
            let rad_angle = RadAngle::from(Coord { x, y }, size);
            let angle_around_z = discretise(rad_angle.angle, angle);
            let tilt_angle = discretise(rad_angle.radius.asin(), angle);
            let norm_vec = Rotation3::from_axis_angle(&Vector3::z_axis(), angle_around_z)
                * Rotation3::from_axis_angle(&Vector3::y_axis(), tilt_angle)
                * Vector3::z(); // XXX YOU ARE HERE

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
    x: u32,
    y: u32,
}

impl RadAngle {
    fn from(coord: Coord, size: u32) -> Self {
        let half_size: f32 = size as f32 / 2.0;
        let x = (coord.x as f32) - half_size;
        let y = -(coord.y as f32) + half_size;
        Self {
            radius: x.hypot(y) / half_size,
            angle: y.atan2(x),
        }
    }
}

fn discretise(val: f32, chunks: f32) -> f32 {
    (val / chunks).round() * chunks
}
