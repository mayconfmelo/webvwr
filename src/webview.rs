use crate::custom_js;

use std::path::PathBuf;
use wry::{
    application::window::Window,
    webview::{
        WebContext,
        WebViewBuilder,
        WebView,
    },
};

pub fn generate_webview(window: Window, url: &str) -> Result<WebView, wry::Error> {
    let mut context = WebContext::new(Some(PathBuf::from(r".\.webview-data")));

    WebViewBuilder::new(window)
        .expect("Could not generate the webview")
        .with_url(url)
        .expect("Could not open URL")
        .with_web_context(&mut context)
        .with_initialization_script(&custom_js::inject())
        .build()
}