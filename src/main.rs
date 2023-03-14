#![windows_subsystem = "windows"]

fn main() -> wry::Result<()> {
  use std::path::PathBuf;
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

  let event_loop = EventLoop::new();
  let mut context = WebContext::new(Some(PathBuf::from(r".\.webview-data")));
  
  let mut js = String::new();
  js.push_str("document.addEventListener('keydown', (event) => {if (event.key == 'F5') {event.preventDefault()}});");

  let window = WindowBuilder::new()
    .with_title("Microlins Italva")
    .with_fullscreen(Some(Fullscreen::Borderless(None)))
    .build(&event_loop)?;

  let _webview = WebViewBuilder::new(window)?
    .with_url("https://portaldoaluno.microlins.com.br/")?
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