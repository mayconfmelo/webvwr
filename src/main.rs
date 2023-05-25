//#![windows_subsystem = "windows"]

use std::path::PathBuf;
use std::fs;
use wry::{
  application::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    window::Fullscreen,
  },
  webview::WebViewBuilder,
  webview::WebContext,
};

fn main() -> wry::Result<()> {
  let event_loop = EventLoop::new();
  let mut context = WebContext::new(Some(PathBuf::from(r".\.webview-data")));

  let mut js_dir = std::env::current_exe()?;
  js_dir.pop();
  js_dir.push("init.js");

  let js;
  if js_dir.exists() {
    js = fs::read_to_string(js_dir.clone()).expect("Unable to read file");
  }
  else {
    js = String::from("");
  }

  println!("{:?}", js_dir);

  let window = WindowBuilder::new()
    .with_title("Webvwr")
    //.with_fullscreen(Some(Fullscreen::Borderless(None)))
    .build(&event_loop)?;

  let _webview = WebViewBuilder::new(window)?
    .with_url("https://google.com/")?
    .with_web_context(&mut context)
    .with_initialization_script(&js)
    .build()?;

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