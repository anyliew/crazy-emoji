use skia_safe::Color;
use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};
use crate::{options::NoOptions, register_meme};

fn masturbate(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (207, 178, 156, 17),
        (194, 172, 159, 27),
    ];

    let img = images[0].image.square().circle().resize_exact((110, 110));
    let mut encoder = GifEncoder::new();
    
    for i in 0..2 {
        let (w, h, x, y) = locs[i];
        let frame = load_image(format!("masturbate/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        
        let resized_img = img.resize_exact((w, h));
        canvas.draw_image(&resized_img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "masturbate",
    masturbate,
    min_images = 1,
    max_images = 1,
    keywords = &["导", "打飞机"],
    date_created = local_date(2025, 5, 27),
    date_modified = local_date(2025, 6, 14),
}