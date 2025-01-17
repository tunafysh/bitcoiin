#![no_std]
#![feature(start)]

extern crate alloc;
use ogc_rs::prelude::*;
use ogc_rs::network::*;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {

    let video = Video::init();

    Input::init(ControllerType::Gamecube);
    Input::init(ControllerType::Wii);

    let gcn_ctrl = Input::new(ControllerType::Gamecube, ControllerPort::One);
    let wii_ctrl = Input::new(ControllerType::Wii, ControllerPort::One);

    Console::init(&video);

    Video::configure(&video.render_config);

    unsafe {
        Video::set_next_framebuffer(video.framebuffer);
    }

    Video::set_black(false);
    Video::flush();
    Video::wait_vsync();

    println!("\033[1;36mHello World!");
    
    let net = Network::init().expect("Failed to initialize network");
    let sock = Network::new_socket(ProtocolFamily::AfInet, SocketType::SockStream).expect("Failed to create socket");
    let addr = IPV4Address {
        address: 0xC0A8B2F4,
    };
    
    sock.bind(addr, 80).expect("Failed to bind socket");
    sock.listen(5).expect("Failed to listen on socket");

   loop {
        Input::update(ControllerType::Gamecube);
        Input::update(ControllerType::Wii);
        if gcn_ctrl.is_button_down(Button::Start) {
            break 0;
        }
        if wii_ctrl.is_button_down(Button::Home) {
            break 0;
        }
        Video::wait_vsync();
    }
}