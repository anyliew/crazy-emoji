use skia_safe::Color;
use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};
use crate::{options::NoOptions, register_meme};

fn sitdown_do(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (180, 55), (180, 68), (181, 111),
    ];

    let user_head = images[0].image.resize_exact((215, 215));
    let mut encoder = GifEncoder::new();
    
    for i in 0..3 {
        let frame_num = (i % 3) + 1;
        let frame = load_image(format!("sitdown_do/{frame_num}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        
        let (x, y) = positions[i];
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.15)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "sitdown_do",
    sitdown_do,
    min_images = 1,
    max_images = 1,
    keywords = &["坐撅"],
    date_created = local_date(2025, 8, 21),
    date_modified = local_date(2025, 9, 4),
}