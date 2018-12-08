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
    fn new() -> Coefs {
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

pub struct Iteration {
    coefs: Coefs,
    height: u32,
    width: u32,
    max_iterations: u32,
}

impl Iteration {
    pub fn new(height: u32, width: u32, max_iterations: u32) -> Iteration {
        Iteration {
            coefs: Coefs::new(),
            height,
            width,
            max_iterations,
        }
    }

    pub fn generate(&self) -> image::GrayImage {
        let x_scale = self.width as f32 / 7.0;
        let y_scale = self.height as f32 / 7.0;

        let x_shift = self.width as f32 / 2.0;
        let y_shift = self.height as f32 / 2.0;

        let mut imgbuf = image::GrayImage::new(self.width, self.height);

        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        for _ in 0..self.max_iterations {
            x = (self.coefs.a * x).sin() + (self.coefs.b * y).sin() - (self.coefs.c * z).cos();

            y = (self.coefs.d * x).sin() + (self.coefs.e * y).sin() - (self.coefs.f * z).cos();

            z = z + 0.1;

            let put_x = x * x_scale + x_shift;
            let put_y = y * y_scale + y_shift;

            imgbuf.put_pixel(
                put_x.round() as u32,
                put_y.round() as u32,
                image::Luma([255]),
            );
        }

        imgbuf
    }
}
