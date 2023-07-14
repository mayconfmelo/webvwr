//#![windows_subsystem = "windows"]

mod custom_js;
mod gui;
mod webview;

use std::path::PathBuf;
use std::env;
use clap::{arg, value_parser, ArgAction, Command, Arg};
use wry::application::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

fn main() -> wry::Result<()> {
    let matches = Command::new("webvwr")
        .version("1.0.0")
        .author("Maycon F. Melo")
        .about("Transform your favorite sites into a native-looking web viewer.")
        .arg(
            arg!([URL] "Defines the url to be used.").action(ArgAction::Set)
        )
        .arg(
            arg!(-a --add "Add the site as a webview application.").action(ArgAction::SetTrue)
        )
        .arg(
            arg!(-r --remove "Remove the site from the webview application list.").action(ArgAction::SetTrue)
        )
        .arg(
            arg!(-t --title "Set the webview title.").action(ArgAction::Set)
        )
        .arg(
            arg!(-i --icon "Set the icon asociated to the webview.").value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(-f --fullscreen "Run the program in full screen mode.").action(ArgAction::SetTrue)
        )
        .get_matches();

    let mut window_title = String::from("");
    if let Some(title) = matches.get_one::<String>("title") {
        window_title.push_str(title);
    }
    else {
        window_title.push_str("Webvwr");
    }

    let mut url = String::from("");
    if let Some(new_url) = matches.get_one::<String>("URL") {
        url.push_str(new_url)
    }
    else {
        url.push_str("https://google.com/")
    }

    let event_loop = EventLoop::new();
    let window = gui::generate_gui(window_title, false, &event_loop);
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