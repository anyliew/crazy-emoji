use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn behind_do(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let self_locs = [
        (72, 5),
        (72, 5),
        (71, 2),
        (70, 3),
        (66, 5),
        (66, 5),
        (66, 5),
        (61, 7),
        (61, 7),
        (69, 5),
    ];
    let user_locs = [
        (174, 91),
        (174, 91),
        (173, 86),
        (171, 87),
        (170, 85),
        (170, 85),
        (167, 82),
        (170, 85),
        (170, 85),
        (172, 88),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("behind_do/{i}.png"))?;
        let self_head = images[0]
            .resize_fit((110, 110), Fit::Cover)
            .circle()
            .rotate(-15.0);
        let user_head = images[1].resize_fit((116, 116), Fit::Cover).circle();
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, user_locs[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&self_head, self_locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 10,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "behind_do",
    behind_do,
    min_images = 2,
    max_images = 2,
    keywords = &["后撅"],
    date_created = local_date(2025, 12, 6),
    date_modified = local_date(2025, 12, 6),
);
