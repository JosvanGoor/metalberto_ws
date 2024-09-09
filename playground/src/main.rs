mod tools; 

use jb::image::RgbPpm;
use tools::ProgressBar;


fn main() {
    let mut image = RgbPpm::new(256, 256);
    let mut progressbar = ProgressBar::new("Scanline Progress", 255);

    for h in 0..256 {
        for w in 0..256 {
            let r = (w as f64) / 255.0;
            let g = (h as f64) / 255.0;
            
            let slice = image.pixel_mut(w, h).unwrap();
            slice[0] = (r * 255.99) as u8;
            slice[1] = (g * 255.99) as u8;
        }

        progressbar.update();
        progressbar.output();
    }
    drop(progressbar);

    image.write_to_file("output.ppm").unwrap();
    println!("done!");
}
