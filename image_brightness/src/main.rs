use std::fmt;
use std::time::Instant;
use std::fs;
extern crate image;
use image::{GenericImageView, DynamicImage,  Rgba};


struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour { 
    
    fn from_rgba(input: &Rgba<u8>) -> Colour {
        Colour {
            r: input[0],
            g: input[1],
            b: input[2],
        }
    }

    // Approximation for greater speed    
    fn brightness(&self) -> u32 {
        let r = self.r as u32;
        let g = self.g as u32;
        let b = self.b as u32;
        // (R * 3 + G * 4 + B ) / 8
        ((r << 1 ) + r + (g << 2) + b) >> 3
    }   
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "R: {}\nG: {}\nB: {}", self.r, self.g, self.b)
    }
}

fn fetch_image(path: &str) -> DynamicImage { 
    let _image = image::open(path).unwrap();
    _image
}


fn main() {

    let paths = fs::read_dir("./test-images/").unwrap();

    for path in paths {
        
        println!("Current file: {}", path.as_ref().unwrap().path().display());
        let now = Instant::now();
        let image = fetch_image(path.unwrap().path().display().to_string().as_str());
        let elapsed = now.elapsed();
        println!("Took {:.2?} to load image.", elapsed);
        let image_size = [image.width(), image.height()];
        
        let mut total_brightness: u32 = 0;
        let now = Instant::now();
        // We step 2 and 2 in order to get every fourth pixel. This significantly increases speed
        // while keeping accuracy ~80% with pictures. 
        for x in (0..image_size[0]).step_by(2) { 
            let mut brightness: u32 = 0;
            for y in (0..image_size[1]).step_by(2) {
                let _pixel = image.get_pixel(x, y);
                let _colour = Colour::from_rgba(&_pixel);
                
                brightness += _colour.brightness();
           
            }
            total_brightness += brightness / (image_size[1] / 4) as u32;
        }

        total_brightness = total_brightness / image_size[0] as u32;
        let elapsed = now.elapsed();
        println!("Took {:.2?} to analyze pixels.\nBrightness: {}", elapsed, total_brightness);

    }

}
