#![no_std]
#![feature(start)]

mod display;
use crate::display::Display;

extern crate alloc;

use embedded_graphics::{
    image::Image, mono_font::{ascii::FONT_6X10, MonoTextStyle}, pixelcolor::Rgb888, prelude::*, primitives::{PrimitiveStyle, Rectangle}, text::Text, Drawable
};

use ogc_rs::prelude::*;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let curdata = include_bytes!("../assets/Wii-Cursor.tga");

    let curtga = Tga::from_slice(curdata).unwrap();
    let curimage =Image::new(&curtga, Point::zero());

    let mut video = Video::init();
    Input::init(ControllerType::Gamecube);
    Input::init(ControllerType::Wii);

    let gcn_ctrl = Input::new(ControllerType::Gamecube, ControllerPort::One);
    let wii_ctrl = (0..4)
        .filter_map(|port| Input::new(ControllerType::Wii, ControllerPort::from(port)).ok())
        .find(|ctrl| ctrl.is_connected())
        .expect("No Wii controller connected");
    wii_ctrl
        .as_wpad()
        .set_data_format(WPadDataFormat::ButtonsAccelIR);

    Console::init(&video);
    Video::configure(&video.render_config);
    unsafe {
        Video::set_next_framebuffer(video.framebuffer);
    }
    Video::set_black(true);
    Video::flush();
    Video::wait_vsync();

    let mut wii_display = Display::new(256 * 1024);
    wii_display.setup(&mut video.render_config);
    Video::set_black(false);

    println!("Hello World!");

    const BACKGROUND: Rectangle = Rectangle::new(Point::zero(), Size::new(640, 528));

    let style = MonoTextStyle::new(&FONT_6X10, Rgb888::CSS_LIGHT_BLUE);

    const CENTER: Point = Point::new(640 / 2, 528 / 2);
    let area: Rectangle = Rectangle::with_center(CENTER, Size::new(200, 200));

    const POINTER: Rectangle = Rectangle::new(Point::zero(), Size::new_equal(10));
    loop {
        Input::update(ControllerType::Gamecube);
        Input::update(ControllerType::Wii);

        if gcn_ctrl.is_button_down(Button::Start) {
            break 0;
        }

        if wii_ctrl.is_button_down(Button::Home) {
            break 0;
        }
        Gx::set_viewport(
            0.0,
            0.0,
            video.render_config.framebuffer_width as f32,
            video.render_config.embed_framebuffer_height as f32,
            0.0,
            0.0,
        );

        let ir = Point::new(
            wii_ctrl.as_wpad().ir().0 as i32,
            wii_ctrl.as_wpad().ir().1 as i32,
        );

        BACKGROUND
            .into_styled(PrimitiveStyle::with_fill(Rgb888::WHITE))
            .draw(&mut wii_display)
            .unwrap();

        if POINTER.translate(ir).intersection(&area).size != Size::zero() {
            area.into_styled(PrimitiveStyle::with_fill(Rgb888::RED))
                .draw(&mut wii_display)
                .unwrap();
            if wii_ctrl.is_button_held(Button::A) {
                area.into_styled(PrimitiveStyle::with_fill(Rgb888::GREEN))
                    .draw(&mut wii_display)
                    .unwrap();
            }
        } else {
            area.into_styled(PrimitiveStyle::with_fill(Rgb888::BLUE))
                .draw(&mut wii_display)
                .unwrap();
        }

        POINTER
            .translate(ir)
            .into_styled(PrimitiveStyle::with_fill(Rgb888::CYAN))
            .draw(&mut wii_display)
            .unwrap();

        Text::new("Hello Rust!", Point::new(20, 30), style).draw(&mut Display).unwrap();

        wii_display.flush(video.framebuffer);

        unsafe {
            Video::set_next_framebuffer(video.framebuffer);
        }
        Video::flush();
        Video::wait_vsync();
    }
}