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
