use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn fleshlight_cleaning_liquid(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = texts.first().unwrap_or(&images[0].name);
    let text = format!("{name}の❤️最爱");
    let frame = load_image("fleshlight_cleaning_liquid/0.png")?;

    let func = |images: Vec<Image>| {
        // 创建新的surface，先绘制用户图片作为最底层
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 先绘制用户图片作为最底层
        let img = images[0].resize_fit((920, 920), Fit::Cover).circle();
        canvas.draw_image(&img, (290, 20), None);
        
        // 然后绘制frame作为中层（覆盖在用户图片之上）
        canvas.draw_image(&frame, (0, 0), None);
        
        // 最后绘制文字作为最上层
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(13, 1039, 430, 1189),
            &text,
            20.0,
            100.0,
            text_params!(
                font_families = &["FZKaTong-M19S"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(255, 255, 255))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "fleshlight_cleaning_liquid",
    fleshlight_cleaning_liquid,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["清洗液"],
    date_created = local_date(2025, 3, 13),
    date_modified = local_date(2026, 10, 12),
);