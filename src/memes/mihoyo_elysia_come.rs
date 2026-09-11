use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn mihoyo_elysia_come(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("mihoyo_elysia_come/0.png")?;
    let male = images[1].image.circle().resize_exact((190, 190));
    let elysia = images[0].image.circle().resize_exact((130, 130));
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.draw_image(&male, (410, 380), None);
    canvas.draw_image(&elysia, (92, 310), None);
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "mihoyo_elysia_come",
    mihoyo_elysia_come,
    min_images = 2,
    max_images = 2,
    keywords = &["爱莉希雅降临"],
    tags = MemeTags::mihoyo(),
    date_created = local_date(2025, 5, 25),
    date_modified = local_date(2025, 5, 25),
);
