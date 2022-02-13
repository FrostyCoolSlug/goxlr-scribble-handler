use std::borrow::{Borrow, BorrowMut};
use goxlr_usb::error::ConnectError;
use goxlr_usb::goxlr::GoXLR;
use image::{ColorType, GenericImage, GenericImageView, Rgba};
use image::imageops::{BiLevel, dither, Nearest};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "mic3.png";

    // Load the image, resize it to the 'size' of the display, then grayscale it.
    let mut img = image::open(format!("test-images-in/{}", filename)).unwrap();
    let mut img = img.grayscale();

    if img.color() == ColorType::La8 || img.color() == ColorType::La16 {
        /* Image has an alpha channel, we need to ensure all transparent pixes are set to 'white'
           to avoid weirdness when processing (this is an issue with the 'official' images, where
           pixels are black with 100% transparency. */

        for pixel in img.clone().pixels() {
            if pixel.2.0[3] == 0 {
                img.put_pixel(pixel.0, pixel.1, Rgba::from([255, 255, 255, 255]));
            }
        }
    }

    // We use 'Nearest' for resizing here, due to the small size and black and white dithering,
    // other methods can lead to extra pixels being set, which doesn't look great.
    let mut img = img.resize(128, 64, Nearest);



    let mut img = img.to_luma8();
    dither(img.borrow_mut(), &BiLevel);
    img.save(format!("test-images-out/{}", filename));

    // Ok, quick connect to the GoXLR..
    //let mut device = GoXLR::open()?;
    Ok(())
}
