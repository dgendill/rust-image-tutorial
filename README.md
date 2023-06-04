# Learning about images with Rust

Let's learn some more about images and image formats with Rust.

Let's start with a simple example: load an image and get information about it. This is basically the same example given in the [image](https://github.com/image-rs/image/blob/master/README.md#opening-and-saving-images) crate.

```rust
use image::{GenericImageView};

fn main() {
    let img = image::open("tests/images/image.jpg").unwrap();
    println!("dimensions {:?}", img.dimensions());
    println!("color {:?}", img.color());
}
```

The "img" variable is a [DynamicImage](https://docs.rs/image/0.24.6/image/enum.DynamicImage.html) which is a matrix of pixels that can be converted to and from RGBA. `DynamicImage` is an abstraction for a lot of different image types. For example `png` images could be encoded on one extreme as a grayscale image (Luma8 or Luma16)([L8 and L16](https://docs.rs/image/0.24.6/image/enum.ColorType.html)) and on the other as RGBA with 16-bit color ([Rgba16](https://docs.rs/image/0.24.6/image/enum.ColorType.html)). Here's how we can create a 8bit grayscale png vs a rgba 16bit image and how we can see that the 16bit image has a larger file size. Note that we first create the `DynamicImage` abstraction with the [new_luma8](https://docs.rs/image/0.24.6/image/enum.DynamicImage.html#method.new_luma8) and [new_rgba16](https://docs.rs/image/0.24.6/image/enum.DynamicImage.html#method.new_rgba16) functions. Then we cast to the concrete image type so we can manipulate the pixel data with `put_pixel`.

```rust
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

/* Output:
"tests/images/gray_cross.png"
dimensions: (25, 25)
color: L8
size (bytes): 301
"tests/images/rgba16_cross.png"
dimensions: (25, 25)
color: Rgba16
size (bytes): 328
*/
```

