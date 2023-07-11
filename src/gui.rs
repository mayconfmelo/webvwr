use wry::application::{
    event_loop::EventLoop,
    window::{
        Window,
        WindowBuilder,
    },
};

pub fn generate_gui(window_title: &str, fullscreen: bool, event_loop: &EventLoop<()>) -> Window {
    if fullscreen {
        use wry::application::window::Fullscreen;

        WindowBuilder::new()
            .with_title(window_title)
            .with_fullscreen(Some(Fullscreen::Borderless(None)))
            .build(&event_loop)
            .expect("Could not create window")
    }
    else {
        WindowBuilder::new()
            .with_title(window_title)
            .build(&event_loop)
            .expect("Could not create window")
    }
}