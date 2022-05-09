use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::fmt;
use std::fmt::Write;
use png::Decoder;
use png::ColorType;
use colored::*;

pub struct Image {
    data: Vec<u8>,
    color_type: ColorType,
    width: u32,
    height: u32,
//  size: u32,
}

impl Image {

    /// Constructs new Image object from PNG file at path
    pub fn new(path: &Path) -> Result<Image, Box<dyn Error>> {
        let decoder = Decoder::new(File::open(path)?);
        let mut reader = decoder.read_info()?;
        let mut img_data = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut img_data)?;

        if info.color_type == ColorType::Indexed {
            return Err("Indexed color type not supported".into());
        }

        Ok( Image {
            data:       img_data,
            color_type: info.color_type,
            width:      info.width,
            height:     info.height,
        })
    }

    /// Returns a string of unicode characters and truecolor escape sequences that,
    /// when printed to a terminal that supports truecolor, will render a PNG in text.
    pub fn as_string(&self) -> Result<String, Box<dyn Error>> {
        let mut buf = String::new();

        for y in (0..self.height).step_by(2) {
            for x in 0..self.height {
                let (tr, tg, tb, _) = self.get_pixel(x, y)?;
                let (br, bg, bb, _) = if (y+1) < self.height {
                    self.get_pixel(x, y+1)?
                } else {
                    // TODO: Try to handle background colors
                    (0, 0, 0, 0)
                };
                write!(&mut buf, "{0}", "▀".truecolor(tr, tg, tb).on_truecolor(br, bg, bb))?;
            }
            if y != self.height - 2 { writeln!(&mut buf)?; }
        }
        Ok(buf)
    }

    /// Returns the width of the imported image
    pub fn width(&self) -> u32 { self.width }

    /// Returns the height of the imported image
    pub fn height(&self) -> u32 { self.height }

    /// Returns a quad tuple corresponding to the (R,G,B,alpha) values
    /// of the pixel at x,y. (with x,y = 0,0 being the top left of the image)
    pub fn get_pixel(&self, x: u32, y: u32) -> Result<(u8, u8, u8, u8), Box<dyn Error>> {
        
        if x > self.width {
            return Err(format!("get_pixel on ({},{}) but width is only {}", 
                    x, y, self.width).into());
        } else if y > self.height {
            return Err(format!("get_pixel on ({},{}) but height is only {}", 
                    x, y, self.height).into());
        }

        let step_size = match self.color_type {
            ColorType::Rgba => 4,
            ColorType::Rgb => 3,
            ColorType::GrayscaleAlpha => 2,
            ColorType::Grayscale => 1,
            ColorType::Indexed => { return Err("Indexed colortype not supported".into()); }
        };

        // Target pixel index
        let p = ((y * self.width * step_size) + (x * step_size)) as usize;

        match self.color_type {
            ColorType::Rgba => Ok((self.data[p], self.data[p+1], self.data[p+2], self.data[p+3])),
            ColorType::Rgb => Ok((self.data[p], self.data[p+1], self.data[p+2], 255)),
            ColorType::GrayscaleAlpha => Ok((self.data[p], self.data[p], self.data[p], self.data[p+1])),
            ColorType::Grayscale => Ok((self.data[p], self.data[p], self.data[p], 255)),
            ColorType::Indexed => { Err("Indexed colortype not supported".into()) }
        }
    }
}

    /* --- Trait Implementations --- */

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buf = self.as_string().map_err(|_| fmt::Error)?;
        write!(f, "{}", buf)?;
        Ok(())
    }
}











