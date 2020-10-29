#[macro_use]
extern crate sciter;

fn main() {
    struct EventHandler;
    impl EventHandler {
        fn ocr(&self, screenshot_filename: String) -> sciter::Value {
            let result = tesseract::ocr(&screenshot_filename, "eng").unwrap();
            sciter::Value::from(format!("{}", result))
        }
    }
    impl sciter::EventHandler for EventHandler {
        dispatch_script_call! (
            fn ocr(String);
        );
    }
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
    sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8
    | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8
    )).unwrap();
    let mut frame = sciter::Window::new();
    let handler = EventHandler {};
	frame.event_handler(handler);
	let dir = std::env::current_dir().unwrap().as_path().display().to_string();
	let filename = format!("{}\\{}", dir, "main.htm");
    frame.load_file(&filename);
    frame.run_app();
}
