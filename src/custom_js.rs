use std::fs;

pub fn inject() -> String {
    let mut js_dir = std::env::current_exe().expect("Could not define current program path");
    js_dir.pop();
    js_dir.push("init.js");

    if js_dir.exists() {
      fs::read_to_string(js_dir.clone()).expect("Unable to read file")
    }
    else {
      String::from("")
    }
}