use rand::RngExt;
use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::number_option, register_meme};

number_option!(Number, 1, 25);

fn fleshlight_random(
    images: Vec<InputImage>,
    texts: Vec<String>,
    options: Number,
) -> Result<Vec<u8>, Error> {
    let total_num = 25;
    let num = options
        .number
        .unwrap_or(rand::rng().random_range(1..=total_num));
    let bg = (num - 1) as usize;

    let name = if !texts.is_empty() {
        texts[0].clone()
    } else if !images[0].name.is_empty() {
        images[0].name.clone()
    } else {
        "他".to_string()
    };
    let text = format!("{name}の❤️最爱");

    let text_positions = [
        (305, 1, 800, 133),
        (533, 39, 779, 140),
        (13, 1039, 430, 1189),
        (40, 110, 374, 207),
        (686, 1100, 1200, 1200),
        (55, 135, 812, 282),
        (261, 31, 758, 91),
        (566, 606, 764, 644),
        (40, 110, 374, 207),
        (26, 74, 380, 141),
        (9, 708, 577, 790),
        (43, 403, 328, 485),
        (13, 1039, 428, 1191),
        (331, 40, 797, 136),
        (465, 5, 792, 87),
        (840, 393, 1200, 464),
        (799, 59, 1168, 211),
        (52, 692, 334, 755),
        (40, 110, 374, 207),
        (252, 648, 649, 692),
        (0, 1037, 339, 1106),
        (93, 688, 493, 769),
        (35, 111, 428, 210),
        (35, 111, 428, 210),
        (22, 638, 739, 737),
    ];
    let text_colors = [
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::BLACK,
        Color::WHITE,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::BLACK,
        Color::WHITE,
        Color::BLACK,
        Color::WHITE,
        Color::BLACK,
        Color::WHITE,
        Color::BLACK,
        Color::WHITE,
        Color::BLACK,
        Color::BLACK,
    ];
    let avatar_positions = [
        (252, 133),
        (65, 105),
        (290, 20),
        (202, 252),
        (130, 180),
        (340, 340),
        (475, 180),
        (499, 611),
        (202, 252),
        (145, 180),
        (60, 110),
        (15, 115),
        (275, -10),
        (320, 144),
        (544, 326),
        (-64, 81),
        (107, 180),
        (105, 75),
        (202, 252),
        (663, 575),
        (140, 195),
        (15, 185),
        (210, 265),
        (215, 222),
        (130, 150),
    ];
    let avatar_sizes = [
        (300, 300),
        (675, 675),
        (920, 920),
        (770, 770),
        (920, 920),
        (920, 920),
        (180, 180),
        (65, 65),
        (770, 770),
        (500, 500),
        (680, 680),
        (350, 350),
        (950, 950),
        (512, 512),
        (230, 230),
        (860, 860),
        (1000, 1000),
        (630, 630),
        (770, 770),
        (125, 125),
        (920, 920),
        (580, 580),
        (770, 770),
        (780, 780),
        (475, 475),
    ];

    let frame = load_image(format!("fleshlight_random/{bg}.png"))?;

    let func = |images: Vec<Image>| {
        let img = images[0].circle().resize_fit(avatar_sizes[bg], Fit::Cover);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, avatar_positions[bg], None);
        canvas.draw_image(&frame, (0, 0), None);
        let (x1, y1, x2, y2) = text_positions[bg];
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(x1, y1, x2, y2),
            &text,
            5.0,
            100.0,
            text_params!(
                font_families = &["FZShaoEr-M11S"],
                text_align = TextAlign::Left,
                paint = new_paint(text_colors[bg]),
            ),
        )?;
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "fleshlight_random",
    fleshlight_random,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["随机杯子"],
    date_created = local_date(2025, 9, 2),
    date_modified = local_date(2025, 9, 2),
);
