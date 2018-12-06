extern crate image;
extern crate rand;

use std::f32::consts::PI;

struct Coefs {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
}

impl Coefs {
    pub fn new() -> Coefs {
        Coefs {
            a: PI * rand::random::<f32>(),
            b: PI * rand::random::<f32>(),
            c: PI * rand::random::<f32>(),
            d: PI * rand::random::<f32>(),
            e: PI * rand::random::<f32>(),
            f: PI * rand::random::<f32>(),
        }
    }
}

fn main() {
    let max_iterations = 10000;

    let imgx = 800;
    let imgy = 800;

    let scalex = 6.0 / imgx as f32;
    let scaley = 6.0 / imgy as f32;

    let mut imgbuf = image::GrayImage::new(imgx, imgy);

    let coefs = Coefs::new();
    let mut x = 0 as f32;
    let mut y = 0 as f32;
    let mut z = 0 as f32;

    for _ in 0..max_iterations {
        x = (coefs.a * x).sin() + (coefs.b * y).sin() - (coefs.c * z).cos();
        y = (coefs.d * x).sin() + (coefs.e * y).sin() - (coefs.f * z).cos();
        z = z + 0.1;

        let putx = (x + 3.0) / scalex;
        let puty = (y + 3.0) / scaley;

        imgbuf.put_pixel(putx.round() as u32, puty.round() as u32, image::Luma([255]));
    }

    imgbuf.save("fractal.png").unwrap();
}
