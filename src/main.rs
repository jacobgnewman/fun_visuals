use image::{Rgb, RgbImage};
use num::complex::Complex;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

const max_iter: u16 = 1000;

fn main() {
    println!("Hello, world!");
    let (width, height) = (3480, 2160);
    let mut img = RgbImage::new(width, height);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || {
            for x in 0..width {
                let i = mandlebrot(width, height, x, y);
                let pixel = val_to_rgb(i);
                tx.send((x, y, pixel)).expect("could not send data!");
            }
        });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv().unwrap();
        img.put_pixel(x, y, pixel)
    }
    img.save("mandlebrot.jpg");
}

fn mandlebrot(width: u32, height: u32, x: u32, y: u32) -> u16 {
    // scaled x,y cooridnates
    let x = (x as f32) / (width as f32) * (2.47) - 2.0;
    let y = (y as f32) / (height as f32) * 2.24 - 1.12;
    let c = Complex::new(x, y);
    let mut z = c.clone();

    let mut iter: u16 = 0;

    for i in 0..max_iter {
        if z.norm() >= 2.0 {
            break;
        }
        z = z * z + c;
        iter = i;
    }
    iter
}

fn val_to_rgb(steps: u16) -> Rgb<i16> {

    if steps == max_iter {
        return Rgb([0,0,0]);
    } else {
        let hue = 360 * (steps / max_iter);
        let x = 1 - (hue /60);
        let color = match hue {
            0..=59 => Rgb([255 , 255*x, 0]),
            60..=119 => Rgb([255 * x, 255, 0]),
            120..=179 => Rgb([0, 255 * x, 255]),
            180..=239 => Rgb([0, 255 * x, 255]),
            240..=299 => Rgb([255*x, 0, 255]),
            300..=360 => Rgb([255, 0, 255*x])
        };
        return  color;
    }
}

