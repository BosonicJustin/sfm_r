use std::f32::consts::PI;
use image::ImageReader;


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

fn main() {
    let img = ImageReader::open("munich.png");

    let g = Gaussian::new(1.0);
    println!("{}", g.eval(0.0,0.0));
}
