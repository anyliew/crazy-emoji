use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn oral_sex(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let self_locs = [(30, 37), (36, 42)];
    let user_locs = [(67, 99), (71, 98)];

    let func = |i: usize, images: Vec<Image>| {
        let template = load_image(format!("oral_sex/{i}.png"))?;
        let self_head = images[0]
            .resize_fit((58, 58), Fit::Cover)
            .circle()
            .rotate(-15.0);
        let user_head = images[1].resize_fit((48, 48), Fit::Cover).circle();
        let mut surface = new_surface(template.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, user_locs[i], None);
        canvas.draw_image(&self_head, self_locs[i], None);
        canvas.draw_image(&template, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 2,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "oral_sex",
    oral_sex,
    min_images = 2,
    max_images = 2,
    keywords = &["口"],
    date_created = local_date(2025, 5, 27),
    date_modified = local_date(2025, 6, 14),
);
