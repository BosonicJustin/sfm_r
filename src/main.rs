use std::f32::consts::PI;
use image::{ImageBuffer, ImageReader, Luma};


struct Gaussian {
    sigma_2: f32,
}

impl Gaussian {
    fn new(sigma: f32) -> Gaussian {
        Gaussian { sigma_2: sigma.powi(2) }
    }

    fn eval(&self, x: f32, y: f32) -> f32 {
        let pre_factor: f32 = 1.0 / (2.0 * PI * self.sigma_2);
        let inside: f32 = - (x.powi(2) + y.powi(2)) / (2.0 * self.sigma_2);

        pre_factor * inside.exp()
    }
}

fn convolve_image(image: &ImageBuffer<Luma<f32>, Vec<f32>>, kernel: &Gaussian, x_c: f32, y_c: f32) -> f32 {
    let (width, height) = image.dimensions();
    let mut sum: f32 = 0.0;

    for j in 0..height - 1 {
        for i in 0..width - 1 {
            let pixel_val = image.get_pixel(i, j)[0];
            let kernel_val = kernel.eval(x_c - i as f32, y_c - j as f32);

            sum += pixel_val * kernel_val;
        }
    }

    sum
}

fn main() {
    let img = ImageReader::open("munich.png")
        .unwrap().decode().unwrap().to_luma32f();

    let (width, height) = img.dimensions();
    // println!("Dimensions: {}x{}", width, height);

    let result = convolve_image(&img, &Gaussian::new(1.0), 10.0, 15.0);

}
