# Learning about images with Rust

Let's learn some more about images and image formats with Rust.

## Lesson 1

Let's start with a simple example: load an image and get information about it. This is basically the same example given in the [image](https://github.com/image-rs/image/blob/master/README.md#opening-and-saving-images) crate.

```rust
use image::{GenericImageView};

fn main() {
    let img = image::open("tests/images/image.jpg").unwrap();
    println!("dimensions {:?}", img.dimensions());
    println!("color {:?}", img.color());
}
```

The "img" variable is a [DynamicImage](https://docs.rs/image/0.24.6/image/enum.DynamicImage.html) which is a matrix of pixels that can be converted to and from RGBA. `DynamicImage` is an abstraction for a lot of different image types. For example `png` images could be encoded on one extreme as a grayscale image (Luma8 or Luma16) ([L8 and L16](https://docs.rs/image/0.24.6/image/enum.ColorType.html)) and on the other as RGBA with 16-bit color ([Rgba16](https://docs.rs/image/0.24.6/image/enum.ColorType.html)). Here's how we can create a 8bit grayscale png vs a rgba 16bit image and how we can see that the 16bit image has a larger file size. Note that we first create the `DynamicImage` abstraction with the [new_luma8](https://docs.rs/image/0.24.6/image/enum.DynamicImage.html#method.new_luma8) and [new_rgba16](https://docs.rs/image/0.24.6/image/enum.DynamicImage.html#method.new_rgba16) functions. Then we cast to the concrete image type so we can manipulate the pixel data with `put_pixel`.

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

## Lesson 2: Combining Images

Let's try combining two images by stacking them side-by-side to create a new image. It should as easy as looping over the images one row at a time and putting that pixel data into a new image buffer whose width is firstImageWidth + secondImageWidth, and whose height equals the taller of the two images.

When I first attempted this, I attempted to create a ImageBuffer directly with `ImageBuffer::new()`, but I don't believe that's correct and in alignment with the crate. Instead, you should use `DynamicImage` as the starting point for new ImageBuffers. The crate says, "This type can act as a converter between specific ImageBuffer instances."

Here's what I ended up for my `combine_side_by_side` function.

```rust
use image::{DynamicImage, GenericImageView, ImageBuffer};

pub fn combine_side_by_side(img1: DynamicImage, img2: DynamicImage) -> DynamicImage {

    let d1 = img1.dimensions();
    let d2 = img2.dimensions();

    let new_width = d1.0 + d2.0;
    let new_height = d1.1.max(d2.1);

    let mut new_image = DynamicImage::new_rgba8(new_width, new_height).to_rgba8();

    println!("{:?} {:?}", new_width, new_height);

    for y in 0..new_height {
        if d1.1 > y {
            for x1 in 0..d1.0 {
                let pixel = new_image.get_pixel_mut(x1, y);
                *pixel = img1.get_pixel(x1, y);
            }
        }

        if d2.1 > y {
            for x2 in 0..d2.0 {
                let pixel = new_image.get_pixel_mut(d1.0 + x2, y);
                *pixel = img2.get_pixel(x2, y);
            }
        }
    }

    DynamicImage::from(new_image)
}

```

I first grabbed the image dimensions and calculated the size of the new image. Then I created the new ImageBuffer to hold rgba8 values. From there, I looped over each row of the images and copied each pixel value from the source images into the new image.