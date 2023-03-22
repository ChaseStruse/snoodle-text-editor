#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

fn main() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "editor_canvas", 
            web_options,
            Box::new(|cc| Box::new(snoodle_text_editor::TextEditorApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}

