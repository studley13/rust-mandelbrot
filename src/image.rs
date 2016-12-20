
extern crate byteorder;
use self::byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Write, stdout};
use std::fs::File;
use std::path::Path;

const BITMAP_PREFIX: [u8; 2] = [0x42, 0x4D];
const PIXEL_START:   i32 = 54;
const DIB_SIZE:      i32 = 40;
const COMPRESSION:   i32 = 0;
const DATA_SIZE:     i32 = 0;
const H_RESOLUTION:  i32 = 3780;
const V_RESOLUTION:  i32 = 3780;
const PALETTE_SIZE:  i32 = 0;
const IMPORTANT:     i32 = 0;
const ROW_PAD_WIDTH: i32 = 4;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Color {
    RGB(u8, u8, u8),
    Grey(u8),
    Black,
}

#[derive(Debug)]
pub struct Bitmap {
    width:  i32,
    height: i32,
    depth:  i16,
    data:   Vec<Color>,
}

impl Bitmap {
    pub fn new(width: i32, height: i32, depth: i16) -> Option<Bitmap> {
        match depth {
            //1 | 4 | 8 | 16 | 24 | 32 => Some(Bitmap {
            8 | 24 => Some(Bitmap {
                width:  width,
                height: height,
                depth:  depth,
                data:   Vec::with_capacity((width * height) as usize),
            }),
            _ => None,
        }
    }

    pub fn set(&mut self, x: i32, y: i32, color: Color) {
        let index = self.width * y + x;

        // Fill out with data
        while self.data.len() <= index as usize && self.data.len() < self.data.capacity() {
            self.data.push(Color::Black);
        }

        if let Some(pixel) = self.data.get_mut(index as usize) {
            *pixel = color
        }
    }

    #[allow(dead_code)]
    pub fn to_file<P: AsRef<Path>>(&self, ref path: &P) {
        let mut file = File::create(path).unwrap();
        self.write_out(&mut file);
    }

    pub fn to_stdout(&self) {
        let mut file = stdout();
        self.write_out(&mut file);
    }

    fn write_out<W: Write>(&self, ref mut file: &mut W) {
        let row_base_length = self.width * self.depth as i32 / 8;
        let row_padding = match row_base_length % ROW_PAD_WIDTH {
            0 => 0,
            _ => ROW_PAD_WIDTH - (self.width % ROW_PAD_WIDTH),
        };
        let size = PIXEL_START + (row_base_length + row_padding) * self.height;

        //println!("Writing {:?} of length {}", self.data, self.data.len());

        self.write_header(file, size);
        self.write_dib(file);
        self.write_pixels(file, row_padding);
        file.flush().unwrap();
    }

    fn write_header<W: Write>(&self, ref mut buf: &mut W, size: i32) {
        buf.write(&BITMAP_PREFIX).unwrap();
        // File size
        buf.write_i32::<LittleEndian>(size).unwrap();
        // These two values are reserved
        buf.write_u16::<LittleEndian>(0).unwrap();
        buf.write_u16::<LittleEndian>(0).unwrap();
        // Pixel Array Start
        buf.write_i32::<LittleEndian>(PIXEL_START).unwrap();
    }

    fn write_dib<W: Write>(&self, ref mut buf: &mut W) {
        // DIB header size
        buf.write_i32::<LittleEndian>(DIB_SIZE).unwrap();
        // Bitmap width
        buf.write_i32::<LittleEndian>(self.width).unwrap();
        // Bitmap height
        buf.write_i32::<LittleEndian>(self.width).unwrap();
        // Color planes
        buf.write_i16::<LittleEndian>(1).unwrap();
        // Bit Depth
        buf.write_i16::<LittleEndian>(self.depth).unwrap();
        // Compression Method
        buf.write_i32::<LittleEndian>(COMPRESSION).unwrap();
        // Bitmap data size
        buf.write_i32::<LittleEndian>(DATA_SIZE).unwrap();
        // Horizontal resolution
        buf.write_i32::<LittleEndian>(H_RESOLUTION).unwrap();
        // Vertical resolution
        buf.write_i32::<LittleEndian>(V_RESOLUTION).unwrap();
        // Number of colors in palette
        buf.write_i32::<LittleEndian>(PALETTE_SIZE).unwrap();
        // Number of omportant colors
        buf.write_i32::<LittleEndian>(IMPORTANT).unwrap();
    }

    fn write_pixels<W: Write>(&self, ref mut buf: &mut W, row_padding: i32) {
        // Generate the row padding
        let mut padding: Vec<u8> = Vec::new();
        for _ in 1..row_padding {
            padding.push(0u8);
        }
        let pad_buf = padding.into_boxed_slice();

        for y in 0..(self.height) {
            for x in 0..(self.width) {
                let bytes = match self.depth {
                    8 => to_grey(self.get_pixel(x, y)),
                    24 => to_bgr(self.get_pixel(x, y)),
                    _ => Vec::new(),
                };
                buf.write(bytes.as_slice()).unwrap();
            }
            buf.write(&*pad_buf).unwrap();
        }
    }

    fn get_pixel(&self, x: i32, y: i32) -> Color {
        let index = self.width * y + x;
        let color = self.data.get(index as usize).unwrap().clone();
        return color;
    }
}

fn to_bgr(color: Color) -> Vec<u8> {
    match color {
        Color::RGB(r, g, b) => vec![b, g, r],
        Color::Grey(g) => vec![g, g, g],
        Color::Black => vec![0u8, 0u8, 0u8],
    }
}

fn to_grey(color: Color) ->  Vec<u8> {
    match color {
        Color::RGB(r, g, b) => vec![((r as u32 + g as u32 + b as u32) / 3) as u8],
        Color::Grey(g) => vec![g],
        Color::Black => vec![0u8],
    }
}
