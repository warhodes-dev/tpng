use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::fmt;
use std::fmt::Write;
use png::Decoder;
use colored::*;


#[allow(dead_code)]
pub struct Image {
    data: Vec<u8>,
    width: u32,
    height: u32,
    size: u32,
}

impl Image {

    pub fn new(path: &Path) -> Result<Image, Box<dyn Error>> {
        let decoder = Decoder::new(File::open(path)?);
        let mut reader = decoder.read_info()?;
        let mut img_data = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut img_data)?;

        if info.color_type != png::ColorType::Rgba {
            return Err("PNG is not of color_type Rgba".into());
        }

        Ok( Image { data: img_data, 
                    width: info.width, 
                    height: info.height,
                    size: (info.width * info.height) } )
    }

    pub fn as_string(&self) -> Result<String, Box<dyn Error>> {
        let img = self;
        let mut buf = String::new();

        for y in (0..img.height).step_by(2) {
            for x in (0..(img.width * 4)).step_by(4) {
                
                let top = y * (img.width * 4) + x;
                let t = top as usize;
                let tr = img.data[t];
                let tg = img.data[t+1];
                let tb = img.data[t+2];

                let (br, bg, bb);
                if y+1 != img.height {
                    let bot = (y+1) * (img.width * 4) + x;
                    let b = bot as usize;
                    br = img.data[b];
                    bg = img.data[b+1];
                    bb = img.data[b+2];
                } else {
                    (br, bg, bb) = (0, 0, 0)
                }

                write!(&mut buf, "{0}", "▀".truecolor(tr, tg, tb).on_truecolor(br, bg, bb))?;
            }
            writeln!(&mut buf)?;
        }
        Ok(buf)
    }

    /*
    fn to_string(img: &Image) -> Result<(), Box<dyn Error>> {
        for idx in (0..(img.size * 4)).step_by(4) {
            
            // Wrap back around
            if (idx) % (img.width * 4) == 0 { println!() }

            let i = idx as usize;
            let r = img.data[i];
            let g = img.data[i+1];
            let b = img.data[i+2];

            print!("{0}{0}", "█".truecolor(r, g, b));
        }
        println!();

        Ok(())
    }
    */

}

    /* --- Trait Implementations --- */

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buf = self.as_string().map_err(|_| fmt::Error)?;
        write!(f, "{}", buf)?;
        Ok(())
    }
}











