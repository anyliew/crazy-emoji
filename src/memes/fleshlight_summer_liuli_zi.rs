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

fn fleshlight_summer_liuli_zi(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("fleshlight_summer_liuli_zi/0.png")?;
    
    let name = texts.first().unwrap_or(&images[0].name);
    
    let text = format!("{name}の❤️最愛");

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 处理用户图片：圆形裁剪并调整尺寸
        let img = images[0]
            .resize_fit((770, 770), Fit::Cover)
            .circle();
        canvas.draw_image(&img, (210, 265), None);
        
        // 绘制frame
        canvas.draw_image(&frame, (0, 0), None);
        
        // 绘制文字
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(35, 111, 428, 210),
            &text,
            20.0,
            100.0,
            text_params!(
                font_families = &["FZShaoEr-M11S"],
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(255, 255, 255))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "fleshlight_summer_liuli_zi",
    fleshlight_summer_liuli_zi,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,

    keywords = &["夏日琉璃子"],
    date_created = local_date(2024, 12, 21),
    date_modified = local_date(2026, 10, 12),
);