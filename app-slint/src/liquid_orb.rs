//! Area-filter the embedded orb at the window's physical DPI before Slint draws it.
//! The software renderer samples raster images without an area downsampling filter.
use image::{imageops, RgbaImage};

pub const FRAME_COUNT: usize = 48;
const FRAME_W: u32 = 384;
const FRAME_H: u32 = 416;
const COLS: u32 = 6;

pub struct LiquidOrb {
    sheet: RgbaImage,
    size: (u32, u32),
    frames: Vec<slint::Image>,
}

impl LiquidOrb {
    pub fn new() -> Result<Self, image::ImageError> {
        let mut sheet =
            image::load_from_memory(include_bytes!("../pets/default/orb.png"))?.to_rgba8();
        // Filter premultiplied color and alpha together to avoid dark/pale fringes.
        for pixel in sheet.pixels_mut() {
            let alpha = pixel[3] as u16;
            for channel in &mut pixel.0[..3] {
                *channel = ((*channel as u16 * alpha + 127) / 255) as u8;
            }
        }
        Ok(Self {
            sheet,
            size: (0, 0),
            frames: Vec::new(),
        })
    }

    pub fn frame(&mut self, index: usize, width: u32, height: u32) -> slint::Image {
        let size = (width.max(1), height.max(1));
        if self.size != size {
            self.frames.clear();
            self.size = size;
            for frame in 0..FRAME_COUNT as u32 {
                let source = imageops::crop_imm(
                    &self.sheet,
                    frame % COLS * FRAME_W,
                    frame / COLS * FRAME_H,
                    FRAME_W,
                    FRAME_H,
                )
                .to_image();
                // Triangle has no negative lobes, preserving valid premultiplied colors.
                let resized =
                    imageops::resize(&source, size.0, size.1, imageops::FilterType::Triangle);
                let buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(
                    resized.as_raw(),
                    size.0,
                    size.1,
                );
                self.frames
                    .push(slint::Image::from_rgba8_premultiplied(buffer));
            }
        }
        self.frames[index % FRAME_COUNT].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orb_frames_cover_complete_sheet_and_keep_transparent_margins() {
        let orb = LiquidOrb::new().unwrap();
        assert_eq!(orb.sheet.dimensions(), (FRAME_W * COLS, FRAME_H * 8));
        for index in [0, 23, 47] {
            let source = imageops::crop_imm(
                &orb.sheet,
                index % COLS * FRAME_W,
                index / COLS * FRAME_H,
                FRAME_W,
                FRAME_H,
            )
            .to_image();
            for (width, height) in [(48, 52), (60, 65), (72, 78), (96, 104)] {
                let frame =
                    imageops::resize(&source, width, height, imageops::FilterType::Triangle);
                assert_eq!(frame.get_pixel(0, 0)[3], 0);
                assert_eq!(frame.get_pixel(width - 1, height - 1)[3], 0);
                assert_eq!(frame.get_pixel(width / 2, height / 2)[3], 255);
                assert!(frame.pixels().filter(|p| p[3] > 0 && p[3] < 255).count() > 40);
                assert!(frame.pixels().all(|p| p.0[..3].iter().all(|v| *v <= p[3])));
            }
        }
    }
}
