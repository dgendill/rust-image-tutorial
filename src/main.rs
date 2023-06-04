use image::{DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, Rgba};

fn make_gray_cross() -> GrayImage {
    let mut img = DynamicImage::new_luma8(25, 25).to_luma8();

    for x in 11..=13 {
        for y in 0..25 {
            img.put_pixel(x, y, Luma([255]));
            img.put_pixel(y, x, Luma([255]));
        }
    }
    img
}

fn make_rgba16_cross() -> ImageBuffer<Rgba<u16>, Vec<u16>> {
    let mut img = DynamicImage::new_rgba16(25, 25).to_rgba16();

    for x in 11..=13 {
        for y in 0..25 {
            img.put_pixel(x, y, Rgba([65535, 65534, 65533, 65532]));
            img.put_pixel(y, x, Rgba([65535, 65534, 65533, 65532]));
        }
    }
    img
}

fn show_image_details(path: &str) {
    let img = image::open(path).unwrap();
    let metadata = std::fs::metadata(path).unwrap();
    println!("{:?}", path);
    println!("dimensions: {:?}", img.dimensions());
    println!("color: {:?}", img.color());
    println!("size (bytes): {:?}", metadata.len())
}

fn main() {
    let mut path;

    path = "tests/images/gray_cross.png";
    let img = make_gray_cross();
    img.save(path).unwrap();
    show_image_details(path);

    path = "tests/images/rgba16_cross.png";
    let img = make_rgba16_cross();
    img.save(path).unwrap();
    show_image_details(path);
}
