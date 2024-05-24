use std::path::PathBuf;

use image::{GenericImage, GenericImageView, ImageBuffer};
use orbrs::common::adaptive_nonmax_suppression;

fn main() {
    let assets_base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/assets");

    let img_path = assets_base_path.join("fast.png");

    let mut img = image::open(img_path).unwrap();
    let (w, h) = img.dimensions();
    let img = img.crop(0, 0, w / 2, h);

    let mut fast_keypoints = orbrs::fast::fast(
        &img.to_luma8(),
        Some(orbrs::fast::FastType::TYPE_9_16),
        None,
    )
    .unwrap();

    let mut img_draw = img.clone().to_rgba8();
    orbrs::fast::draw_moments(&mut img_draw, &fast_keypoints);

    let suppressed_fast_keypoints = adaptive_nonmax_suppression(&mut fast_keypoints, 30);
    let mut img_draw_sup = img.clone().to_rgba8();
    orbrs::fast::draw_moments(&mut img_draw_sup, &suppressed_fast_keypoints);

    let mut img_out = ImageBuffer::new(w + w / 2, h);
    img_out.copy_from(&img, 0, 0).unwrap();
    img_out.copy_from(&img_draw, w / 2, 0).unwrap();
    img_out.copy_from(&img_draw_sup, w, 0).unwrap();

    img_out
        .save(assets_base_path.join("fast_out.png"))
        .expect("Failed to save image file");
}
