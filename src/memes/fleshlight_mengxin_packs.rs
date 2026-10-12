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

fn fleshlight_mengxin_packs(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("fleshlight_mengxin_packs/0.png")?;
    
    let name = &images[0].name;
    
    let text = format!("{name}喜爱的萌新礼包");

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 处理用户图片：圆形裁剪并调整尺寸
        let img = images[0]
            .resize_fit((350, 350), Fit::Cover)
            .circle();
        canvas.draw_image(&img, (15, 115), None);
        
        // 绘制frame
        canvas.draw_image(&frame, (0, 0), None);
        
        // 绘制文字
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(43, 403, 328, 485),
            &text,
            20.0,
            100.0,
            text_params!(
                font_families = &["FZShaoEr-M11S"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(255, 255, 255))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "fleshlight_mengxin_packs",
    fleshlight_mengxin_packs,
    min_images = 1,
    max_images = 1,

    keywords = &["萌新礼包"],
    date_created = local_date(2025, 6, 1),
    date_modified = local_date(2026, 10, 12),
);