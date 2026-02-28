use skia_safe::Color;
use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};
use crate::{options::NoOptions, register_meme};

fn laydown_do(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (135, 18), (135, 18), (136, 33), (136, 33), (136, 33),
        (133, 61), (133, 61), (133, 61), (138, 26), (138, 26),
        (138, 26),
    ];

    let user_head = images[0].image.resize_exact((110, 110));
    let mut encoder = GifEncoder::new();
    
    for i in 0..11 {
        let frame_num = (i % 11) + 1;
        let frame = load_image(format!("laydown_do/{frame_num}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        
        let (x, y) = positions[i];
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        
        encoder.add_frame(surface.image_snapshot(), 0.03)?;
    }
    
    Ok(encoder.finish()?)
}

register_meme! {
    "laydown_do",
    laydown_do,
    min_images = 1,
    max_images = 1,
    keywords = &["躺撅"],
    date_created = local_date(2025, 8, 21),
    date_modified = local_date(2025, 8, 21),
}