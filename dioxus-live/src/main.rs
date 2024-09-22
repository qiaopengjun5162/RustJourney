use dioxus_live::app;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    // launch the web app
    dioxus_web::launch(app);
}
