//#![windows_subsystem = "windows"]

mod custom_js;
mod gui;
mod webview;

use wry::application::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

fn main() -> wry::Result<()> {
    let window_title = "Webvwr";
    let url = "https://google.com/";

    let event_loop = EventLoop::new();
    let window = gui::generate_gui(window_title, true, &event_loop);
    let _webview = webview::generate_webview(window, url).expect("Could not build the webview");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Started"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}