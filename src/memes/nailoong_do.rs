use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn nailoong_do(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (194, 87), (193, 86), (198, 78), (201, 73), (201, 71),
        (196, 68), (186, 76), (178, 89), (172, 103), (163, 113),
        (162, 113), (175, 107), (184, 98), (194, 90), (199, 84),
        (203, 80), (203, 80), (199, 82), (194, 91), (187, 102),
        (177, 119), (164, 140), (152, 142), (161, 141), (174, 132),
        (189, 121), (203, 109), (210, 101), (208, 93), (204, 90),
        (203, 90), (191, 97), (181, 105), (171, 118), (156, 135),
        (153, 138), (167, 134), (180, 122), (196, 113), (205, 107),
        (211, 96), (207, 93), (205, 92), (200, 98), (191, 107),
        (185, 124), (175, 136), (157, 140), (157, 140), (171, 136),
        (186, 128), (197, 117),
    ];
    let sizes = [(76, 76); 52];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("nailoong_do/{i}.png"))?;
        let user_head = images[0].resize_exact(sizes[i]).rotate(45.0);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, positions[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 52,
            duration: 0.02,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "nailoong_do",
    nailoong_do,
    min_images = 1,
    max_images = 1,
    keywords = &["奶龙撅"],
    date_created = local_date(2026, 5, 19),
    date_modified = local_date(2026, 5, 19),
);
