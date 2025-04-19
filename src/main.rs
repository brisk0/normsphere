extern crate image;
extern crate nalgebra;

use clap::Parser;
use image::{Rgb, RgbImage};
use nalgebra::{Rotation3, Vector3};
use std::{f32::consts::PI, path::PathBuf};

fn main() {
    let args = Args::parse();
    let angle = args.angle * PI / 180.0;

    let mut img = RgbImage::new(args.size, args.size);
    for y in 0..args.size {
        for x in 0..args.size {
            // These are the direction they tilt the vector in, not the axes to rotate around
            let rad_angle = RadAngle::from(Coord { x, y }, args.size);
            let angle_around_z = discretise(rad_angle.angle, angle);
            let tilt_angle = discretise(rad_angle.radius.asin(), angle);
            let norm_vec = Rotation3::from_axis_angle(&Vector3::z_axis(), angle_around_z)
                * Rotation3::from_axis_angle(&Vector3::y_axis(), tilt_angle)
                * Vector3::z();

            let rgb_vec = norm_vec * 127.5 + Vector3::new(127.5, 127.5, 127.5);
            let rgb: Vec<u8> = rgb_vec.as_slice().iter().map(|x| x.ceil() as u8).collect();
            let rgb: [u8; 3] = rgb.try_into().unwrap();
            img.put_pixel(x, y, Rgb(rgb));
        }
    }
    img.save(&args.outfile).unwrap();
    eprintln!("Image saved to {}", args.outfile.display());
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long, default_value_t = 256)]
    /// Pixel width and height of the resulting image
    size: u32,
    #[arg(short, long, default_value_t = 15f32)]
    /// Angle step in degrees in azimuth and inclination
    angle: f32,
    #[arg(short, long, default_value = "normsphere.png")]
    /// Path to write output image. Format determined by extension using the `image` crate
    outfile: PathBuf,
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
