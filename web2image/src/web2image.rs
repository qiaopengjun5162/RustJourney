use std::time::Instant;

use anyhow::Result;

use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptions};
use image::{imageops::overlay, DynamicImage, ImageFormat, Luma};
use qrcode::QrCode;

fn url2image(url: &str) -> Result<DynamicImage> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .window_size(Some((1200, 1600)))
            .build()
            .unwrap(),
    )?;
    let tab = browser.new_tab()?;
    let viewport = tab
        .navigate_to(url)?
        .wait_for_element("body")?
        .get_box_model()?
        .margin_viewport();
    dbg!(&viewport);
    let data = tab.capture_screenshot(ScreenshotFormat::PNG, Some(8), Some(viewport), true)?;
    Ok(data)
}

fn gen_qrcode(url: &str) -> Result<DynamicImage> {
    let code = QrCode::new(url.as_bytes())?;

    // Render the bits into an image.
    let buf = code.render::<Luma<u8>>().build();
    Ok(DynamicImage::ImageLuma8(buf))
}

fn do_overlay(bottom: &mut DynamicImage, top: &DynamicImage) {
    let start = Instant::now();
    let x = (bottom.width() - top.width() - 10) as i64;
    let y = (bottom.height() - top.height() - 10) as i64;
    overlay(bottom, top, x, y);
    println!("overlay took {:?}", start.elapsed().as_millis());
}

pub fn web2image(url: &str, output: &str, format: ImageFormat) -> Result<()> {
    let mut bottom = url2image(url)?;
    let qrcode = gen_qrcode(url)?;
    do_overlay(&mut bottom, &qrcode);
    // fs::write(output, &data)?;
    let start = Instant::now();
    bottom.save_with_format(output, format)?;
    println!("time spent on {:?}", start.elapsed().as_millis());

    Ok(())
}
