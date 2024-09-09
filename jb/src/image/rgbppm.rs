use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Clone, Debug)]
pub struct RgbPpm {
    width:  usize,
    height: usize,
    data:   Vec<u8>,
}

impl RgbPpm {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width:  width,
               height: height,
               data:   vec![0; width * height * 3], }
    }

    pub fn pixel(&self, width: usize, height: usize) -> Option<&[u8]> {
        if width > self.width || height > self.height {
            return None;
        }

        let index = 3 * ((height * self.width) + width);
        Some(&self.data[index..(index + 3)])
    }

    pub fn pixel_mut(&mut self, width: usize, height: usize) -> Option<&mut [u8]> {
        if width > self.width || height > self.height {
            return None;
        }

        let index = 3 * ((height * self.width) + width);
        Some(&mut self.data[index..(index + 3)])
    }

    pub fn write_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = BufWriter::new(File::create(filename)?);

        write!(&mut file, "P3\n{} {}\n255\n", self.width, self.height)?;

        for h in 0..self.height {
            for w in 0..self.width {
                let index = ((h * self.width) + w) * 3;
                write!(&mut file, "{} {} {}\n", self.data[index], self.data[index + 1], self.data[index + 2])?;
            }
        }

        Ok(())
    }
}
